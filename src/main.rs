use std::sync::Arc;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Context, EmptySubscription, Object, Schema,
};
use async_graphql_axum::GraphQL;
use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use config::{dbconfig::DB, dberror::MyError};
use dotenv::dotenv;
use schema::Mutation;
use tower_http::cors::CorsLayer;

pub mod config;
pub mod schema;
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

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
pub struct AppState {
    pub db: DB,
}
#[tokio::main]
async fn main() -> Result<(), MyError> {
    dotenv().ok();

    let db = DB::init().await?;
    let schema = Schema::build(QueryRoot, Mutation::default(), EmptySubscription)
        .data(AppState { db: db.clone() })
        .finish();

    let app = Router::new()
        .route(
            "/",
            get(graphql_playground).post_service(GraphQL::new(schema)),
        )
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
                .allow_credentials(true)
                .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]),
        );

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
