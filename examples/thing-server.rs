use actix_rt;
use serde_json::json;
use std::sync::{Arc, RwLock, Weak};
use std::{thread, time};
use uuid::Uuid;
use std::collections::BTreeMap;
use url::Url;

use webthing::{
    thing_server::ThingServer, 
    ActionAffordance, 
    EventAffordance, 
    PropertyAffordance, 
    ActionObject, 
    ThingObject, 
    EventObject, 
    PropertyObject,
    Form,
    FormOperationType
};

fn make_things() -> BTreeMap<String,ThingObject> {
    let mut ret = BTreeMap::new();

    let mut to = ThingObject::new(&Url::parse("/").unwrap());

    to.add_property(
        &"name".to_string(),
        &Some("A test property".to_string()),
        &Url::parse("/single/name").ok().unwrap(),
        &Some(FormOperationType::ReadProperty)

    );
    
    ret.insert("THING".to_string(),to);

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

