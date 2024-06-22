use crate::{
    model::UserModel,
    schema::{InfoQuery, SearchQuery},
    AppState,
};
use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Jungle Challenge";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

#[get("/info")]
async fn info_handler(opts: web::Query<InfoQuery>, data: web::Data<AppState>) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let mut conditions: Vec<String> = vec![];

    if let Some(uid) = &opts.uid {
        conditions.push(format!("uid = '{}'", uid));
    }

    if let Some(upper_date) = &opts.upper_date {
        conditions.push(format!("birthday <= '{}'", upper_date));
    }

    if let Some(lower_date) = &opts.lower_date {
        conditions.push(format!("birthday >= '{}'", lower_date));
    }

    if conditions.len() == 0 {
        let message = "Please provide at least one query parameter";
        return HttpResponse::BadRequest().json(json!({"status": "error","message": message}));
    }

    let conditions_str = conditions.join(" AND ");

    let base_query = "SELECT uid, name, birthday, sex FROM users u";
    let mut query = if !conditions_str.is_empty() {
        format!("{} WHERE {}", base_query, conditions_str)
    } else {
        base_query.to_string()
    };
    println!("BASE QUERY: {:?}", base_query);
    println!("QUERY: {:?}", query);

    // If we have lower_date or upper_date, we need to sort the result
    // by name (2nd item).
    if opts.lower_date.is_some() || opts.upper_date.is_some() {
        query.push_str(" ORDER BY u.name ");
    }

    query.push_str(" LIMIT $1 OFFSET $2 ");

    println!("Query: {:?}", query);

    let query_result = sqlx::query_as::<_, UserModel>(&query)
        .bind(limit as i32)
        .bind(offset as i32)
        .fetch_all(&data.db)
        .await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching user info";
        println!("Error: {:?}", query_result.err().unwrap().to_string());
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let user = query_result.unwrap();

    let json_response = serde_json::json!(user);

    HttpResponse::Ok().json(json_response)
}

#[get("/search")]
async fn search_handler(
    opts: web::Query<SearchQuery>,
    data: web::Data<AppState>,
) -> impl Responder {
    let name = format!("%{}%", &opts.name);

    let query_result = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users u
        WHERE u.name ILIKE $1
        ORDER by u.name",
        name,
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching all user items";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let users = query_result.unwrap();

    let json_response = serde_json::json!(users);
    HttpResponse::Ok().json(json_response)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(info_handler)
        .service(search_handler);

    conf.service(scope);
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    // use dotenv::dotenv;
    // use sqlx::postgres::PgPoolOptions;

    #[actix_rt::test]
    async fn test_health_checker_handler() {
        let mut app = test::init_service(App::new().service(health_checker_handler)).await;

        let req = test::TestRequest::get().uri("/healthchecker").to_request();

        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
        let response_body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(response_body["status"], "success");
        assert_eq!(response_body["message"], "Jungle Challenge");
    }

    /* #[actix_rt::test]
    async fn test_info_handler() {
        dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
            .unwrap();

        let mut app = test::init_service(
            App::new()
                .app_data(AppState { db: pool.clone() })
                .service(info_handler),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/info?uid=0000000000&limit=1")
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
    } */
}
