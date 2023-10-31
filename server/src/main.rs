use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use axum::extract::Extension;
use tower_http::cors::{Any, CorsLayer};
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, Sqlite, Pool, migrate::MigrateDatabase};
use std::str::FromStr;
use std::sync::Arc;

mod routes;
mod handlers;
mod response;
mod model;

use routes::router;

pub struct AppState {
    db: Pool<Sqlite>,
}

#[tokio::main]
async fn main() {
    const DB_URL: &str = "sqlite://blue.db";

    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("⚙️ Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await 
        {
            Ok(_) => {
                println!("✅ Creaation of database is successful!")
            }
            Err(err) => {
                panic!("❌ Failed to create database: {}", err)
            }
        }
    } else {
        println!("💾 Database already exists");
    }

    //let connect_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
    let connect_opts = SqliteConnectOptions::from_str(DB_URL).unwrap();

    let db_pool = match SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_opts)
        .await
    {
        Ok(db_pool) => {
            println!("✅ Connection to the database is successful!");
            db_pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    match sqlx::migrate!().run(&db_pool).await
    {
        Ok(_) => {
            println!("✅ Migration is successful!");
        }
        Err(err) => {
            println!("🔥 Failed to migrate: {:?}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        //.allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = router(Arc::new(AppState { db: db_pool.clone() })).layer(cors).layer(Extension(db_pool));

    println!("🚀 Blue Framework Server v0 started successfully");
    axum::Server::bind(&"127.0.0.1:3030".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
