use std::collections::{ BTreeMap};
use super::property_object::PropertyObject;
use super::action_object::{ActionHandlerTrait, ActionObject};
use super::event_object::{EventObject,EventHandlerTraits};
use super::notifiable_object::NotifiableObject;
use super::observable_object::ObservableObject;

use super::super::affordances::thing_description::{ ThingDescription,ThingDescriptionFactory};
use super::super::affordances::property_affordance::{ PropertyAffordance, PropertyAffordanceFactory};
use super::super::affordances::event_affordance::{ EventAffordance, EventAffordanceFactory};
use super::super::affordances::action_affordance::{ ActionAffordance, ActionAffordanceFactory};
use super::super::affordances::form::{Form,FormOperationType };
use std::boxed::Box;
use std::sync::Arc;
use std::marker::Sync;
use std::vec::Drain;


///1
pub struct ThingObject {
    ///1
    td                  : Arc<Box<dyn ThingDescription>>,
    ///1
    props               : BTreeMap<String, PropertyObject>,
    ///1
    actions             : BTreeMap<String, ActionObject>,
    ///1
    events              : BTreeMap<String, EventObject>,
}
unsafe impl Sync for ThingObject {}
unsafe impl Send for ThingObject {}
///1
pub fn coerce<S: ?Sized>(r: &mut Box<S>) -> &mut S {
    r
}


impl ThingObject {
    ///1
    pub fn get_description(&self) -> Arc<Box<dyn ThingDescription>> {
        self.td.clone()
    }
    ///1
    pub fn get_description_mut(&mut self) -> Arc<Box<dyn ThingDescription>> {
        self.td.clone()
    }
    ///1
    pub fn drain_queue(&mut self, ws_id: String, object_name : &String ) -> Vec<Drain<String>> {
        let mut drains: Vec<Drain<String>> = Vec::new();
        let mut evt : &mut dyn NotifiableObject = match self.events.get_mut(object_name) {
            None => return Vec::new(),
            Some(x) => x
        };


        let o = evt.get_notifications(&ws_id);
        let v : &mut Vec<String> = 
        match o {
            None => return drains,
            Some(x) => x
        };
        
        drains.push(v.drain(..));

        drains
    }


    ///1
    pub fn new(ctx : &String) -> Self {
        let ret  = ThingObject {
            td : Arc::new(ThingDescriptionFactory::new(ctx)),
            props : BTreeMap::new(),
            actions : BTreeMap::new(),
            events : BTreeMap::new()
        };

        ret
    }
    
    
    ///1
    pub fn add_property(&mut self, name : &String, prop : PropertyObject) {
        self.props.insert(name.to_string(), prop);
    }
    ///1
    pub fn  remove_property(&mut self, k : &String) {
        self.props.remove(k);
        let td: &mut Box<dyn ThingDescription >= &mut Arc::get_mut(&mut self.td).unwrap();
        td.remove_property(k);
    }
    ///1
    pub fn get_properties(&self) -> & BTreeMap<String, PropertyObject> {
        &self.props
    }
    ///1
    pub fn get_properties_mut(&mut self) -> &mut BTreeMap<String, PropertyObject> {
        &mut self.props
    }

    ///1
    pub fn add_event(
        &mut self, 
        name    : &String, 
        evt     : EventObject

    ) {
        self.events.insert(name.to_string(),evt);

    }
    ///1
    pub fn  remove_event(&mut self, k : &String) {
        self.events.remove(k);
        let td: &mut Box<dyn ThingDescription >= &mut Arc::get_mut(&mut self.td).unwrap();
        td.remove_event(k);
    }
    ///1
    pub fn get_events(&self) -> & BTreeMap<String, EventObject> {
        &self.events
    }
    ///1
    pub fn get_events_mut(&mut self) -> &mut BTreeMap<String, EventObject> {
        &mut self.events
    }

    ///1
    pub fn add_action(
        &mut self, 
        name    : String, 
        act     : ActionObject,
    ) {
        self.actions.insert(name,act);
    }
    ///1
    pub fn  remove_action(&mut self, k : &String) {
        self.actions.remove(k);
    }
    ///1
    pub fn get_actions(&self) -> & BTreeMap<String, ActionObject> {
        &self.actions
    }
    ///1
    pub fn get_actions_mut(&mut self) -> &mut BTreeMap<String, ActionObject> {
        &mut self.actions
    }
    ///1
    pub fn get_thing_description(&self) -> Arc<Box<dyn ThingDescription>> {
        self.td.clone()
    }
    
    ///1
    pub fn get_property(&self, n : &String) -> Option<&PropertyObject> {
        self.props.get(n)
    }
    ///1
    pub fn get_property_mut(&mut self, n : &String) -> Option<&mut PropertyObject> {
        self.props.get_mut(n)
    }
    ///1
    pub fn get_event(&self, n : &String) -> Option<&EventObject> {
        self.events.get(n)
    }
    ///1
    pub fn get_event_mut(&mut self, n : &String) -> Option<&mut EventObject> {
        self.events.get_mut(n)
    }
    ///1
    pub fn get_action(&self, n : &String) -> Option<&ActionObject> {
        self.actions.get(n)
    }
    ///1
    pub fn get_action_mut(&mut self, n : &String) -> Option<&mut ActionObject> {
        self.actions.get_mut(n)
    }
    
}

