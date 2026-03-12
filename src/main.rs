use actix_files::{Files, NamedFile};
use actix_web::middleware::Logger;
use actix_web::{App, HttpResponse, HttpServer, Result, web};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use log::{error, info, warn};
use path_slash::PathExt;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use validator::Validate;

mod errors;
mod models;
mod schema;

use self::errors::UserError;
use self::models::*;
use self::schema::cats::dsl::*;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[derive(Deserialize, Validate)]
struct CatEndpointPath {
    #[validate(range(min = 1, max = 150))]
    id: u64,
}

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}

fn setup_database() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(&database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool.")
}

async fn add_cat_endpoint(
    pool: web::Data<DbPool>,
    mut parts: awmp::Parts,
) -> Result<HttpResponse, UserError> {
    let file_path = parts
        .files
        .take("image")
        .pop()
        .and_then(|f| f.persist_in("./image").ok())
        .ok_or_else(|| {
            error!("Error in getting image path");
            UserError::ValidationError
        })?;
    let formatted_file_path = Path::new(&file_path).to_string_lossy().into_owned();
    let normalized_image_path = Path::new(&formatted_file_path)
        .to_slash()
        .unwrap_or_default();
    let normalized_image_path = normalized_image_path
        .strip_prefix(".")
        .unwrap_or(&normalized_image_path)
        .to_string();
    let text_fields: HashMap<_, _> = parts.texts.as_pairs().into_iter().collect();
    let mut connection = pool.get().map_err(|_| {
        error!("Failed to get DB connection from pool");
        UserError::DBPoolGetError
    })?;
    let new_cat = NewCat {
        name: text_fields
            .get("name")
            .ok_or_else(|| {
                error!("Error in getting name field");
                UserError::ValidationError
            })?
            .to_string(),
        image_path: normalized_image_path,
    };
    web::block(move || {
        diesel::insert_into(cats)
            .values(&new_cat)
            .execute(&mut connection)
    })
    .await
    .map_err(|_| {
        error!("Blocking Thread Pool Error");
        UserError::UnexpectedError
    })?
    .map_err(|_| {
        error!("Failed to get DB connection from pool");
        UserError::DBPoolGetError
    })?;
    Ok(HttpResponse::Created().finish())
}

async fn cat_endpoint(
    pool: web::Data<DbPool>,
    cat_id: web::Path<CatEndpointPath>,
) -> Result<HttpResponse, UserError> {
    cat_id.validate().map_err(|_| {
        warn!("Parameter validation failed");
        UserError::ValidationError
    })?;
    let mut connection = pool.get().map_err(|_| {
        error!("Failed to get DB connection from pool");
        UserError::DBPoolGetError
    })?;
    let query_id = cat_id.id.clone();
    let cat_data = web::block(move || cats.filter(id.eq(query_id)).first::<Cat>(&mut connection))
        .await
        .map_err(|_| {
            error!("Blocking Thread Pool Error");
            UserError::UnexpectedError
        })?
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                error!("Cat ID: {} not found in DB", &cat_id.id);
                UserError::NotFoundError
            }
            _ => {
                error!("Unexpected error");
                UserError::UnexpectedError
            }
        })?;
    Ok(HttpResponse::Ok().json(cat_data))
}

async fn cats_endpoint(pool: web::Data<DbPool>) -> Result<HttpResponse, UserError> {
    let mut connection = pool.get().map_err(|_| {
        error!("Failed to get DB connection from pool");
        UserError::DBPoolGetError
    })?;
    let cats_data = web::block(move || cats.limit(100).load::<Cat>(&mut connection))
        .await
        .map_err(|_| {
            error!("Blocking Thread Pool Error");
            UserError::UnexpectedError
        })?
        .map_err(|_| {
            error!("Failed to get DB connection from pool");
            UserError::DBPoolGetError
        })?;

    Ok(HttpResponse::Ok().json(cats_data))
}

fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .app_data(
                web::PathConfig::default().error_handler(|_, _| UserError::ValidationError.into()),
            )
            .route("/cats", web::get().to(cats_endpoint))
            .route("/add_cat", web::post().to(add_cat_endpoint))
            .route("/cat/{id}", web::get().to(cat_endpoint)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let pool = setup_database();

    info!("Listening on port 8080");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(awmp::PartsConfig::default().with_temp_dir("./tmp"))
            .service(Files::new("/static", "static").show_files_listing())
            .service(Files::new("/image", "image").show_files_listing())
            .configure(api_config)
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, test};

    #[actix_web::test]
    async fn test_cats_endpoint_get() {
        let pool = setup_database();
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(api_config),
        )
        .await;
        let req = test::TestRequest::get().uri("/api/cats").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }
}
