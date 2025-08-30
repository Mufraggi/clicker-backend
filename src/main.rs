
mod domain;
mod repositories;

extern crate pretty_env_logger;
#[macro_use] extern crate log;
use fred::prelude::*;
use log::{debug, info};
use std::{env, io, str, time::Duration};
use std::time::SystemTime;
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
use crate::domain::answer::AnswerName;
use crate::domain::poll::PollId;
use uuid::{Uuid, Timestamp, NoContext};
use crate::repositories::redis_db::impls::pool_repository::{PoolRedisRepositoryTrait, PoolRepository};

#[derive(Debug, Deserialize)]
struct KeyPath {
    key: String,
}
#[actix_web::main]
async fn main() -> io::Result<()> {
    pretty_env_logger::init();
    let pool = get_redis_pool().await;
    println!("Hello, world!");
    let repo = repositories::redis_db::impls::pool_repository::PoolRepository::new(pool.clone());
    test(repo).await;
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


async fn test(repo: PoolRepository) -> io::Result<String> {
    let timestamp = Timestamp::now(NoContext);
    let poll_id = create_poll_id(Uuid::new_v7(timestamp));

    println!("🚀 Début des tests d'incrémentation...");

    // Test 1: Incrémenter "Oui" plusieurs fois
    println!("\n📊 Test 1: Incrémentation de 'Oui'");
    let answer_oui = create_answer_name("Oui");

    for i in 1..=3 {
        match repo.increment_answer_for_poll(&poll_id, &answer_oui).await {
            Ok(count) => println!("   ✅ Incrémentation {} - Total: {}", i, count),
            Err(e) => println!("   ❌ Erreur incrémentation {}: {}", i, e),
        }
    }

    // Test 2: Incrémenter "Non" plusieurs fois
    println!("\n📊 Test 2: Incrémentation de 'Non'");
    let answer_non = create_answer_name("Non");

    for i in 1..=5 {
        match repo.increment_answer_for_poll(&poll_id, &answer_non).await {
            Ok(count) => println!("   ✅ Incrémentation {} - Total: {}", i, count),
            Err(e) => println!("   ❌ Erreur incrémentation {}: {}", i, e),
        }
    }

    // Test 3: Incrémenter "Peut-être" quelques fois
    println!("\n📊 Test 3: Incrémentation de 'Peut-être'");
    let answer_maybe = create_answer_name("Peut-être");

    for i in 1..=2 {
        match repo.increment_answer_for_poll(&poll_id, &answer_maybe).await {
            Ok(count) => println!("   ✅ Incrémentation {} - Total: {}", i, count),
            Err(e) => println!("   ❌ Erreur incrémentation {}: {}", i, e),
        }
    }

    // Test 4: Incrémenter "Abstention" une fois
    println!("\n📊 Test 4: Incrémentation de 'Abstention'");
    let answer_abstention = create_answer_name("Abstention");

    match repo.increment_answer_for_poll(&poll_id, &answer_abstention).await {
        Ok(count) => println!("   ✅ Incrémentation - Total: {}", count),
        Err(e) => println!("   ❌ Erreur incrémentation: {}", e),
    }

    // Test 5: Récupérer tous les résultats
    println!("\n🔍 Test 5: Récupération de tous les résultats");
    match repo.get_poll_results(&poll_id).await {
        Ok(results) => {
            println!("   ✅ Résultats du poll:");
            let mut total_votes = 0;
            for (answer, count) in &results {
                println!("     - {}: {} votes", answer, count);
                total_votes += count;
            }
            println!("   📈 Total des votes: {}", total_votes);
            println!("   📊 Nombre de réponses différentes: {}", results.len());
        },
        Err(e) => println!("   ❌ Erreur récupération: {}", e),
    }

    println!("\n🎉 Tests terminés!");
    Ok("aa".to_string())

}

fn create_poll_id(id: Uuid) -> PollId {
    // Supposons que vous avez une méthode pour créer un PollId
    // Adaptez selon votre implémentation
    PollId::new(id)
}

fn create_answer_name(name: &str) -> AnswerName {
    AnswerName::new(name.to_string())
}

async fn get_redis_pool() -> Pool {
    let pool_size = env::var("REDIS_POOL_SIZE")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(8);
    let config = Config::from_url("redis://@127.0.0.1:6379").unwrap();
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
    pool
}