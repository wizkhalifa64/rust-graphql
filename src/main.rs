use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Context, EmptySubscription, Object, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    http::HeaderMap,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
use schema::auth::userschema::UserMutation;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::config::dbconfig::Config;

pub mod config;
pub mod schema;
async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres,and Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
struct QueryRoot;

#[Object(cache_control(max_age = 60))]
impl QueryRoot {
    async fn integer(&self, _context: &Context<'_>) -> i64 {
        1
    }
    async fn hello(&self) -> String {
        "Hello".to_string()
    }
}

struct AppState {
    schema: Schema<QueryRoot, UserMutation, EmptySubscription>,
}
struct DbState {
    db: Pool<Postgres>,
    env: Config,
}
type MySchema = Schema<QueryRoot, UserMutation, EmptySubscription>;
async fn graphql_handler(
    schema: State<MySchema>,
    _headers: HeaderMap,
    request: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(request.0).await.into()
}
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Config::init();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    let db = DbState {
        db: pool.clone(),
        env: config.clone(),
    };
    let schema = Schema::build(QueryRoot, UserMutation, EmptySubscription).data(db);
    let state = AppState {
        schema: schema.finish(),
    };
    let app = Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/graphql", post(graphql_handler))
        .route("/playground", get(graphql_playground))
        .with_state(state.schema);
    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
