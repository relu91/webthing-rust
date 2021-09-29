use std::collections::{ BTreeMap};
use super::property_object::PropertyObject;
use super::action_object::{ActionHandlerTrait, ActionObject};
use super::event_object::EventObject;
use super::super::affordances::thing_description::{ ThingDescription,ThingDescriptionFactory};
use super::super::affordances::property_affordance::{ PropertyAffordance, PropertyAffordanceFactory};
use super::super::affordances::event_affordance::{ EventAffordance, EventAffordanceFactory};
use super::super::affordances::action_affordance::{ ActionAffordance, ActionAffordanceFactory};
use url::Url;
use super::super::affordances::form::{Form,FormOperationType };
use std::boxed::Box;
use std::sync::Arc;
use std::marker::Sync;

///1
/// 
/*
pub trait ThingObjectTraits {
    ///1
    fn get_actions(&self) -> & BTreeMap<String, ActionObject>;
    //1
    fn get_actions_mut(&mut self) -> &mut BTreeMap<String, ActionObject>;
    ///1
    fn get_thing_description(&self) -> Arc<Box<dyn ThingDescription>>;
}
*/
///1

pub struct ThingObject {
    ///1
    td                  : Arc<Box<dyn ThingDescription>>,
    ///1
    props               : BTreeMap<String, PropertyObject>,
    ///1
    actions             : BTreeMap<String, ActionObject>,
    ///1
    events              : BTreeMap<String, EventObject>

}
unsafe impl Sync for ThingObject {}
unsafe impl Send for ThingObject {}

fn coerce<S: ?Sized>(r: &mut Box<S>) -> &mut S {
    r
}


impl ThingObject {
    ///1
    pub fn new(ctx : &Url) -> Self {
        let ret  = ThingObject {
            td : Arc::new(ThingDescriptionFactory::new(ctx)),
            props : BTreeMap::new(),
            actions : BTreeMap::new(),
            events : BTreeMap::new()

        };

        ret
    }
    
    ///1
    pub fn add_property(
        &mut self, 
        name    : &String, 
        desc    : &Option<String>,
        href    : &Url,
        id      : &Option<FormOperationType>
    ) {
        let mut pa : Arc<Box<dyn PropertyAffordance>> = Arc::new(PropertyAffordanceFactory::new());
        let mut ppa: &mut Box<dyn PropertyAffordance >= &mut Arc::get_mut(&mut pa).unwrap();
        let mut s_ref: &mut dyn PropertyAffordance = coerce(&mut ppa);

        PropertyAffordance::set_description(&mut *s_ref,desc);
        PropertyAffordance::set_title(&mut *s_ref,&Some(name.clone()));


        let mut frm  : Form = Form::new(href);

        match id {
            None => (),
            Some(x) => frm.set_operation(*x),
        }

        ppa.add_form(frm);

        let ppo = PropertyObject::new(name,pa.clone());

        self.props.insert(name.to_string(),ppo);

        let td: &mut Box<dyn ThingDescription >= &mut Arc::get_mut(&mut self.td).unwrap();

        td.add_property(name,pa.clone());


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
        desc    : &Option<String>,
        href    : &Url,
        id      : &Option<FormOperationType>
    ) {
        let mut pa : Arc<Box<dyn EventAffordance>> = Arc::new(EventAffordanceFactory::new());
        let mut ppa: &mut Box<dyn EventAffordance >= &mut Arc::get_mut(&mut pa).unwrap();
        let mut s_ref: &mut dyn EventAffordance = coerce(&mut ppa);

        super::super::affordances::interaction_affordance::InteractionAffordance::set_title(s_ref,&Some(name.to_string()));
        super::super::affordances::interaction_affordance::InteractionAffordance::set_description( s_ref, desc);

        let mut frm  : Form = Form::new(href);

        match id {
            None => (),
            Some(x) => frm.set_operation(*x),
        }

        ppa.add_form(frm);

        let ppo = EventObject::new(name,pa.clone(), &mut*self);

        self.events.insert(name.to_string(),ppo);

        let td: &mut Box<dyn ThingDescription >= &mut Arc::get_mut(&mut self.td).unwrap();
        td.add_event(name,pa.clone());


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
        name    : &String, 
        desc    : &Option<String>,
        href    : &Url,
        id      : &Option<FormOperationType>,
        handler : Arc<Box< dyn ActionHandlerTrait>>
    ) {
        let mut pa : Arc<Box<dyn ActionAffordance>> = Arc::new(ActionAffordanceFactory::new());
        let mut ppa: &mut Box<dyn ActionAffordance >= &mut Arc::get_mut(&mut pa).unwrap();
        let mut s_ref: &mut dyn ActionAffordance = coerce(&mut ppa);

        super::super::affordances::interaction_affordance::InteractionAffordance::set_title(s_ref,&Some(name.to_string()));
        super::super::affordances::interaction_affordance::InteractionAffordance::set_description( s_ref, desc);

        let mut frm  : Form = Form::new(href);

        match id {
            None => (),
            Some(x) => frm.set_operation(*x),
        }

        ppa.add_form(frm);

        let ppo = ActionObject::new(name,pa.clone(),&mut *self,handler);

        self.actions.insert(name.to_string(),ppo);

        let td: &mut Box<dyn ThingDescription >= &mut Arc::get_mut(&mut self.td).unwrap();
        td.add_action(name,pa.clone());


    }
    ///1
    pub fn  remove_action(&mut self, k : &String) {
        self.actions.remove(k);
        let td: &mut Box<dyn ThingDescription >= &mut Arc::get_mut(&mut self.td).unwrap();
        td.remove_action(k);
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

