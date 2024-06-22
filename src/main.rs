mod handler;
mod model;
mod schema;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    before_init().await;

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
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
    if let Some(should_create_indexes) = std::env::var_os("CREATE_INDEXES") {
        if should_create_indexes == "true" {
            create_database_indexes(pool.clone()).await;
        }
    }

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

async fn before_init() {
    dotenv().ok();
    env_logger::init();
}

async fn create_database_indexes(pool: Pool<Postgres>) {
    println!("🕒 Creating indexes. This will most likely take some time...");
    let mut tx = pool.begin().await.unwrap();

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_uid ON users (uid)")
        .execute(&mut tx)
        .await
        .unwrap();
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_birthday ON users (birthday)")
        .execute(&mut tx)
        .await
        .unwrap();
    sqlx::query("CREATE EXTENSION IF NOT EXISTS pg_trgm")
        .execute(&mut tx)
        .await
        .unwrap();
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_name_trgm ON users USING gin (name gin_trgm_ops)")
        .execute(&mut tx)
        .await
        .unwrap();

    let result = tx.commit().await;

    match result {
        Ok(_) => println!("✅ Indexes created successfully"),
        Err(err) => println!("🔥 Failed to create indexes: {:?}", err),
    }
}
