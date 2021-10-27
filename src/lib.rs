#![deny(missing_docs)]

//! Implementation of an HTTP [Web Thing](https://webthings.io/api/).

extern crate std;
/*
/// Action trait and base implementation.
pub mod action;

/// Event trait and base implementation.
pub mod event;

/// Property trait and base implementation.
pub mod property;

/// WebThingServer implementation.
pub mod server;

/// Thing trait and base implementation.
pub mod thing;

/// Utility functions.
pub mod utils;


pub use action::{Action, BaseAction};
pub use event::{BaseEvent, Event};
pub use property::{BaseProperty, Property};
pub use server::{ThingsType, WebThingServer};
pub use thing::{BaseThing, Thing};
*/
///1
pub mod thing_server;



///Expected response data
pub mod affordances;
///1
pub mod objects;


pub use affordances::action_affordance::ActionAffordance;
pub use affordances::event_affordance::EventAffordance;
pub use affordances::property_affordance::PropertyAffordance;
pub use affordances::form::Form;
pub use affordances::form::FormOperationType;
pub use affordances::link::Link;


pub use objects::thing_object::ThingObject;
pub use objects::action_object::{ ActionObject, ActionHandlerTraits};
pub use objects::event_object::{EventObject,EventHandlerTraits};
pub use objects::property_object::PropertyObject;

pub use objects::notifiable_object::NotifiableObject;
pub use objects::observable_object::ObservableObject;

pub use objects::thing_helpers::ThingHelpers;














