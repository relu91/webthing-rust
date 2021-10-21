
use actix_rt;
use std::collections::BTreeMap;
use std::sync::{Arc,RwLock};

//use url::String;

use webthing::{
    thing_server::ThingServer, 
    ThingObject, 
    FormOperationType,
    ThingHelpers,
};

fn make_things() -> BTreeMap<String,Arc<RwLock<ThingObject>>> {
    let mut ret = BTreeMap::new();

    let to = Arc::new(RwLock::new(ThingObject::new(&"/".to_string())));

    
    ThingHelpers::add_readonly_property(
        to.clone(),
        &"get_name".to_string(),
        &Some("A test property".to_string()),
        &"/single/getName".to_string(),
        &Some(serde_json::Value::String("a value".to_string()))

    );
    
    ThingHelpers::add_writeonly_property(
        to,
        &"set_name".to_string(),
        &Some("A test property".to_string()),
        &"/single/setName".to_string(),
        &Some(serde_json::Value::String("another value".to_string()))

    );

    //create evemt
    ThingHelpers::add_event(
        to,
        &"an_event".to_string(),
        &Some("An Event".to_string()),
        &"/single/anEvent".to_string(),
        &None
    );

    //creates action
/*    
    to.add_action(
        &"an_event".to_string(),
        &Some("An Event".to_string()),
        &"/single/anEvent".to_string(),
        &None

    );
*/    
    ret.insert("THING".to_string(),to.clone());

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

