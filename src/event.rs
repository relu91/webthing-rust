/*
use serde_json;
use std::marker::{Send, Sync};
use super::affordances::event_affordance::EventAffordance;
use super::utils::timestamp;

/// High-level Event trait.
pub trait Event: Send + Sync {
    /// Get the event's name.
    fn get_name(&self) -> String;

    /// Get the event's data.
    fn get_data(&self) -> Option<serde_json::Value>;

    /// Get the event's timestamp.
    fn get_time(&self) -> String;

    fn get_affordance(&self) -> &Box<dyn EventAffordance>;
}

/// Basic event implementation.
///
/// An Event represents an individual event from a thing.
///
/// This can easily be used by other events to handle most of the boring work.
pub struct BaseEvent {
    name: String,
    data: Option<serde_json::Value>,
    time: String,
    aff:  Box<dyn EventAffordance>
}

impl BaseEvent {
    /// Create a new BaseEvent.
    pub fn new(
        name: String, 
        data: Option<serde_json::Value>,
        aff : Box<dyn EventAffordance> 
    ) -> Self {
        Self {
            name: name,
            data: data,
            time: timestamp(),
            aff : aff
        }
    }
}

impl Event for BaseEvent {
    /// Get the event's name.
    fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Get the event's data.
    fn get_data(&self) -> Option<serde_json::Value> {
        self.data.clone()
    }

    /// Get the event's timestamp.
    fn get_time(&self) -> String {
        self.time.clone()
    }

    fn get_affordance(&self) -> &Box<dyn EventAffordance> {
        &self.aff
    }
}
*/
