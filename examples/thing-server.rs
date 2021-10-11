
use actix_rt;
use std::collections::BTreeMap;
//use url::String;

use webthing::{
    thing_server::ThingServer, 
    ThingObject, 
    FormOperationType
};

fn make_things() -> BTreeMap<String,ThingObject> {
    let mut ret = BTreeMap::new();

    let mut to = ThingObject::new(&"/".to_string());

    to.add_property(
        &"name".to_string(),
        &Some("A test property".to_string()),
        &"/single/name".to_string(),
        &Some(FormOperationType::ReadProperty),
        &Some(serde_json::Value::String("a value".to_string()))

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
        true,
        Some(8888),
        None,
        None,
        things
    );

    let z = server.start(None);

    z.await

    
}

