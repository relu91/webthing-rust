use actix_rt;
use rand::Rng;
use serde_json::json;
use std::sync::{Arc, RwLock, Weak};
use std::{thread, time};
use uuid::Uuid;
use super::super::server::WebThingServer;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let mut things =  Vec::new();


    // If adding more than one thing, use ThingsType::Multiple() with a name.
    // In the single thing case, the thing's name will be broadcast.
    let mut server = WebThingServer::new(
        ThingsType::Multiple(things, "LightAndTempDevice".to_owned()),
        Some(8888),
        None,
        None,
        Box::new(Generator),
        None,
        None,
    );
    server.start(None).await
}
