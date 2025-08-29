mod repositories;
mod domain;

extern crate pretty_env_logger;
#[macro_use] extern crate log;
use fred::prelude::*;
use log::{debug, info};
use std::{env, io, str, time::Duration};

use actix_web::{
    body::BoxBody,
    http::StatusCode,
    middleware,
    web::{delete, get, post, resource, Data, Path},
    App,
    HttpResponse,
    HttpServer,
};
use bytes::Bytes;
use fred::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct KeyPath {
    key: String,
}
#[actix_web::main]
async fn main() -> io::Result<()> {
    pretty_env_logger::init();

    let pool_size = env::var("REDIS_POOL_SIZE")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(8);
    let config = Config::from_url("redis://foo:bar@127.0.0.1:6379").unwrap();
    let pool: Pool = Builder::from_config(config)
        .with_connection_config(|config| {
            config.connection_timeout = Duration::from_secs(10);
        })
        // use exponential backoff, starting at 100 ms and doubling on each failed attempt up to 30 sec
        .set_policy(ReconnectPolicy::new_exponential(0, 100, 30_000, 2))
        .build_pool(pool_size)
        .expect("Failed to create redis pool");

    pool.init().await.expect("Failed to connect to redis");
    info!("Connected to Redis");
    println!("Hello, world!");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(
                resource("/{key}")
                  //  .route(get().to(get_key))
                  //  .route(post().to(set_key))
                  //  .route(delete().to(del_key)),
            )
           // .service(resource("/{key}/incr").route(post().to(incr_key)))
            .wrap(middleware::NormalizePath::trim())
    })
        .workers(2)
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
