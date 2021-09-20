use actix_rt;
use serde_json::json;
use std::sync::{Arc, RwLock, Weak};
use std::{thread, time};
use uuid::Uuid;
use thing_server::ThingServer;

