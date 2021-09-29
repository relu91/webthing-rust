use actix_rt;
use serde_json::json;
use std::sync::{Arc, RwLock, Weak};
use std::{thread, time};
use uuid::Uuid;
use std::collections::BTreeMap;

use webthing::{
    thing_server::ThingServer, 
    ActionAffordance, 
    EventAffordance, 
    PropertyAffordance, 
    ActionObject, 
    ThingObject, 
    EventObject, 
    PropertyObject
};

fn make_things() -> BTreeMap<String,ThingObject> {
    let mut ret = BTreeMap::new();


    ret
}
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let things = make_things();

    // If adding more than one thing, use ThingsType::Multiple() with a name.
    // In the single thing case, the thing's name will be broadcast.
    let mut server = ThingServer::new(
        "/".to_string(),
        Some(8888),
        None,
        None,
        things
    );

    let z = server.start(None);

    z.await

    
}

