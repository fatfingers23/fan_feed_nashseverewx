mod wp_admin_response;

use actix_files::{Files};
use actix_cors::Cors;
use actix_web::{Error, get, HttpResponse};
use actix_web::error::ErrorBadRequest;
use log::{error};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use anyhow::anyhow;
use redis::{Client, Commands, RedisResult};
use crate::wp_admin_response::Root;


/// End point that gets called to load in posts json from WP Admin
#[get("/recent-posts")]
async fn cached_wp_admin(redis_client: Data<Client>) -> Result<HttpResponse, Error> {
    let posts = get_or_set_posts(redis_client).await;
    match posts {
        Ok(result) => {
            Ok(HttpResponse::Ok().json(result))
        }
        Err(err) => {
            error!("Failed to fetch data from WP Admin: {}", err);
            return Err(ErrorBadRequest("Failed to fetch data from WP Admin"));
        }
    }
}

/// This method checks the cache to see if the posts are already stored in Redis. If they are, it returns the posts.
/// If not it calls the wp api endpoint
async fn get_or_set_posts(redis_client: Data<Client>) -> Result<Root, anyhow::Error> {
    let posts_key = "recent_posts";
    let mut redis_connection = redis_client.get_connection().expect("Failed to get Redis connection");
    let exists: RedisResult<bool> = redis_connection.exists(posts_key);

    let does_key_exist = match exists {
        Ok(exists) => exists,
        Err(err) => {
            error!("Failed to check if key exists: {:?}", err);
            return Err(anyhow!("Failed to check if key exists"));
        }
    };

    match does_key_exist {
        true => {
            let posts: RedisResult<String> = redis_connection.get(posts_key);
            return match posts {
                Ok(posts) => {
                    let posts_json: Root = serde_json::from_str(&posts).expect("Failed to parse JSON from Redis");
                    Ok(posts_json)
                }
                Err(err) => {
                    error!("Failed to get posts from Redis: {:?}", err);
                    Err(anyhow!("Failed to get posts from Redis"))
                }
            };
        }
        false => {
            let web_call = fetch_recent_posts().await;
            match web_call {
                Ok(result) => {
                    let posts_json = serde_json::to_string(&result).expect("Failed to serialize JSON");
                    let _: () = redis_connection.set(posts_key, posts_json).expect("Failed to set posts in Redis");
                    Ok(result)
                }
                Err(err) => {
                    error!("Failed to fetch data from WP Admin: {}", err);
                    return Err(anyhow!("Failed to fetch data from WP Admin"));
                }
            }
        }
    }
}


/// Fetches the most recent post from the WP Admin API
async fn fetch_recent_posts() -> anyhow::Result<Root, anyhow::Error> {
    let client = reqwest::Client::new();

    let res = client.get("https://nashvillesevereweather.com/wp-json/wp/v2/posts/?_embed&per_page=10")
        .send()
        .await;
    match res {
        Ok(result) => {
            let result_json = result.json::<Root>().await;
            // Err(anyhow!("Failed to parse JSON from WP Admin"))
            match result_json {
                Ok(json) => {
                    Ok(json)
                }
                Err(err) => {
                    error!("Failed to parse JSON from WP Admin: {}", err);
                    return Err(anyhow!("Failed to parse JSON from WP Admin"));
                }
            }
        }
        Err(err) => {
            error!("Failed to fetch data from WP Admin: {}", err);
            return Err(anyhow!("Failed to fetch data from WP Admin"));
        }
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let redis_client = redis::Client::open("redis://127.0.0.1/")
        .expect("Failed to connect to Redis");

    HttpServer::new(move ||
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(redis_client.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method(),
            )
            .service(
                web::scope("/api").service(cached_wp_admin)
            )
            .service(Files::new("/", "./static_files")
                .index_file("index.html"))
    )

        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}