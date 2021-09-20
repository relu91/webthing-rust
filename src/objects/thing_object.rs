use std::collections::{ BTreeMap,BTreeSet };
use super::property_object::PropertyObject;
use super::action_object::ActionObject;
use super::event_object::EventObject;
use super::super::affordances::thing_description::{ ThingDescription,ThingDescriptionFactory};
use super::super::affordances::property_affordance::{ PropertyAffordance, PropertyAffordanceFactory};
use super::super::affordances::event_affordance::{ EventAffordance, EventAffordanceFactory};
use super::super::affordances::action_affordance::{ ActionAffordance, ActionAffordanceFactory};
use url::Url;
use super::super::affordances::form::{Form,FormOperationType };
use std::boxed::Box;
use std::sync::Arc;




pub struct ThingObject {
    td                  : Arc<Box<dyn ThingDescription>>,
    props               : BTreeMap<String, PropertyObject>,
    actions             : BTreeMap<String, ActionObject>,
    events              : BTreeMap<String, EventObject>

}

impl ThingObject {
    pub fn new(ctx : &Url) -> Self {
        let ret  = ThingObject {
            td : Arc::new(ThingDescriptionFactory::new(ctx)),
            props : BTreeMap::new(),
            actions : BTreeMap::new(),
            events : BTreeMap::new()

        };

        ret
    }

    pub fn add_property(
        &mut self, 
        name    : &String, 
        desc    : &Option<String>,
        href    : &Url,
        id      : &Option<FormOperationType>
    ) {
        let mut pa : Box<dyn PropertyAffordance> = PropertyAffordanceFactory::new();
        let mut ppa = pa.as_mut();

        super::super::affordances::interaction_affordance::InteractionAffordance::set_title(ppa,&Some(name.to_string()));
        super::super::affordances::interaction_affordance::InteractionAffordance::set_description(ppa, desc);

        let frm  : Form = Form::new(href);

        match id {
            None => (),
            Some(x) => frm.set_operation(*x),
        }

        ppa.add_form(frm);

        let ppo = PropertyObject::new(name,pa);

        self.props.insert(name.to_string(),ppo);

        self.td.add_property(name,&pa);


    }

    pub fn  remove_property(&mut self, k : &String) {
        self.props.remove(k);
        self.td.remove_property(k);
    }

    pub fn add_event(
        &mut self, 
        name    : &String, 
        desc    : &Option<String>,
        href    : &Url,
        id      : &Option<FormOperationType>
    ) {
        let mut pa : Box<dyn EventAffordance> = EventAffordanceFactory::new();
        let mut ppa = pa.as_mut();

        super::super::affordances::interaction_affordance::InteractionAffordance::set_title(ppa,&Some(name.to_string()));
        super::super::affordances::interaction_affordance::InteractionAffordance::set_description(ppa, desc);

        let frm  : Form = Form::new(href);

        match id {
            None => (),
            Some(x) => frm.set_operation(*x),
        }

        ppa.add_form(frm);

        let ppo = EventObject::new(name,pa);

        self.events.insert(name.to_string(),ppo);

        self.td.add_event(name,&pa);


    }

    pub fn  remove_event(&mut self, k : &String) {
        self.events.remove(k);
        self.td.remove_event(k);
    }



    pub fn get_thing_description(&self) -> Arc<Box<dyn ThingDescription>>{
        self.td
    }
}