use async_graphql::{Context, Error, InputObject, Object, Result, SimpleObject};
use bcrypt::{hash, verify};
use chrono::prelude::{DateTime, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::DbState;

#[derive(Debug, Deserialize, FromRow, Serialize, Clone, SimpleObject)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub age: Option<i16>,
    pub role: i16,
    pub email: String,
    pub password: String,
    pub photo: String,
    pub verified: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, InputObject, Clone)]
pub struct RegisterUserSchema {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub age: i16,
}

#[derive(Debug, Deserialize, InputObject, Clone)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TokenClaims {
    sub: String,
    iat: usize,
    exp: usize,
}
#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject)]
pub struct UserResponse {
    pub user: User,
    pub token: String,
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn register(
        &self,
        ctx: &Context<'_>,
        body: RegisterUserSchema,
    ) -> Result<UserResponse> {
        let user_exist: Option<bool> =
            sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM users WHERE email = $1)")
                .bind(body.email.to_owned().to_ascii_lowercase())
                .fetch_one(&ctx.data::<DbState>()?.db)
                .await
                .map_err(|e| e)?;

        if let Some(exists) = user_exist {
            if exists {
                return Err(Error::new("Email already exists"));
            }
        }
        let hashed = hash(body.password.as_bytes(), 5).expect("Error");
        let saved_user: User = sqlx::query_as(
            "INSERT INTO users (email,password,first_name,last_name,age) VALUES ($1,$2,$3,$4,$5) RETURNING *",
        )
        .bind(body.email.to_string().to_owned().to_ascii_lowercase())
        .bind(hashed)
        .bind(body.first_name.to_string())
        .bind(body.last_name.to_string())
        .bind(body.age)
        .fetch_one(&ctx.data::<DbState>()?.db)
        .await
        .map_err(|e|e)?;

        let now = chrono::Utc::now();
        let iat = now.timestamp() as usize;
        let exp = 3600;
        let claims: TokenClaims = TokenClaims {
            sub: saved_user.id.to_string(),
            exp,
            iat,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(ctx.data::<DbState>()?.env.jwt_secret.as_ref()),
        )
        .unwrap();

        Ok(UserResponse {
            user: saved_user,
            token,
        })
    }

    async fn login(&self, ctx: &Context<'_>, body: LoginUserSchema) -> Result<UserResponse> {
        let user: User = sqlx::query_as("SELECT * FROM users WHERE email = $1")
            .bind(body.email.to_ascii_lowercase())
            .fetch_optional(&ctx.data::<DbState>()?.db)
            .await
            .map_err(|e| e)?
            .ok_or_else(|| "Username nor exist")?;
        let compare_pass = match verify(body.password.as_bytes(), &user.password) {
            Ok(valid) => valid,
            Err(_) => false,
        };
        if !compare_pass {
            return Err(Error::new("Invalid credential"));
        }
        let now = chrono::Utc::now();
        let iat = now.timestamp() as usize;
        let exp = 3600;
        let claims: TokenClaims = TokenClaims {
            sub: user.id.to_string(),
            exp,
            iat,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(ctx.data::<DbState>()?.env.jwt_secret.as_ref()),
        )
        .unwrap();
        Ok(UserResponse { user, token })
    }
}
