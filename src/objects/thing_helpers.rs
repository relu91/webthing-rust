use std::sync::{Arc,RwLock };
use super::thing_object::ThingObject;
use super::property_object::PropertyObject;
use super::action_object::{ActionHandlerTraits, ActionObject};
use super::event_object::{EventObject,EventHandlerTraits};
use super::super::affordances::thing_description::{ ThingDescription,ThingDescriptionFactory};
use super::super::affordances::property_affordance::{ PropertyAffordance, PropertyAffordanceFactory};
use super::super::affordances::event_affordance::{ EventAffordance, EventAffordanceFactory};
use super::super::affordances::action_affordance::{ ActionAffordance, ActionAffordanceFactory};
use super::super::affordances::form::{Form,FormOperationType };
///1
pub  struct ThingHelpers {

}

fn coerce<S: ?Sized>(r: &mut Box<S>) -> &mut S {
    r
}

impl ThingHelpers {
    ///1
    pub fn add_event(
        thing   : Arc<RwLock<ThingObject>>,
        name    : &String, 
        desc    : &Option<String>,
        href    : &String,
        id      : &Option<FormOperationType>,
        handl : Arc<Box<dyn EventHandlerTraits >>
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


        let ppo = EventObject::new(name,pa.clone(), handl.clone(), thing.clone());

        let mut  t_thing = thing.write().unwrap();
        t_thing.add_event(name,ppo);

        
        let mut ta: Arc<RwLock<Box<dyn ThingDescription>>> = t_thing.get_description_mut();
        let td: &mut Box<dyn ThingDescription >= &mut ta.write().unwrap();

        td.add_event(name,pa.clone());



    }


    ///1
    pub fn add_action(
        thing   : Arc<RwLock<ThingObject>>,
        name    : &String, 
        desc    : &Option<String>,
        href    : &String,
        id      : &Option<FormOperationType>,
        handler : Arc<Box< dyn ActionHandlerTraits>>
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

        let ppo = ActionObject::new(name,pa.clone(),thing.clone(),handler);

        let mut t_thing = thing.write().unwrap();
        t_thing.add_action(name.to_string(),ppo);

        let mut ta: Arc<RwLock<Box<dyn ThingDescription>>> = t_thing.get_description_mut();
        let td: &mut Box<dyn ThingDescription >= &mut ta.write().unwrap();

        td.add_action(name,pa.clone());


    }

    ///1
    pub fn add_readonly_property(
        thing   : Arc<RwLock<ThingObject>>,
        name    : &String, 
        desc    : &Option<String>,
        href    : &String,
        init_val: &Option<serde_json::Value>
    ) {
        let mut pa : Arc<Box<dyn PropertyAffordance>> = Arc::new(PropertyAffordanceFactory::new());
        let mut ppa: &mut Box<dyn PropertyAffordance >= &mut Arc::get_mut(&mut pa).unwrap();
        let mut s_ref: &mut dyn PropertyAffordance = coerce(&mut ppa);

        PropertyAffordance::set_description(&mut *s_ref,desc);
        PropertyAffordance::set_title(&mut *s_ref,&Some(name.clone()));


        let mut frm  : Form = Form::new(href);

        frm.set_operation(FormOperationType::ReadProperty);

        s_ref.add_form(frm);
        s_ref.set_readonly(Some(true));

        let mut ppo = PropertyObject::new(name,thing.clone(),pa.clone());
        ppo.set_value(init_val);

        let mut t_thing = thing.write().unwrap();
        t_thing.add_property(name,ppo);

        let mut ta: Arc<RwLock<Box<dyn ThingDescription>>> = t_thing.get_description_mut();
        let td: &mut Box<dyn ThingDescription >= &mut ta.write().unwrap();

        td.add_property(name,pa.clone());

/*
        let td: &mut Box<dyn ThingDescription >= &mut Arc::get_mut(&mut ta).unwrap();
        td.add_property(name,pa.clone());
*/

    }
    ///1
    pub fn add_writeonly_property(
        thing   : Arc<RwLock<ThingObject>>,
        name    : &String, 
        desc    : &Option<String>,
        href    : &String,
        init_val: &Option<serde_json::Value>
    ) {
        let mut pa : Arc<Box<dyn PropertyAffordance>> = Arc::new(PropertyAffordanceFactory::new());
        let mut ppa: &mut Box<dyn PropertyAffordance >= &mut Arc::get_mut(&mut pa).unwrap();
        let mut s_ref: &mut dyn PropertyAffordance = coerce(&mut ppa);

        PropertyAffordance::set_description(&mut *s_ref,desc);
        PropertyAffordance::set_title(&mut *s_ref,&Some(name.clone()));


        let mut frm  : Form = Form::new(href);

        frm.set_operation(FormOperationType::WriteProperty);

        s_ref.add_form(frm);
        s_ref.set_writeonly(Some(true));

        let mut ppo = PropertyObject::new(name,thing.clone(),pa.clone());
        ppo.set_value(init_val);

        let mut t_thing = thing.write().unwrap();
        t_thing.add_property(name,ppo);

        let mut ta: Arc<RwLock<Box<dyn ThingDescription>>> = t_thing.get_description_mut();
        let td: &mut Box<dyn ThingDescription >= &mut ta.write().unwrap();


        td.add_property(name,pa.clone());


    }
    ///1
    pub fn add_std_property(
        thing   : Arc<RwLock<ThingObject>>,
        name    : &String, 
        desc    : &Option<String>,
        href    : &String,
        init_val: &Option<serde_json::Value>
    ) {
        let mut pa : Arc<Box<dyn PropertyAffordance>> = Arc::new(PropertyAffordanceFactory::new());
        let mut ppa: &mut Box<dyn PropertyAffordance >= &mut Arc::get_mut(&mut pa).unwrap();
        let  s_ref: &mut dyn PropertyAffordance = coerce(&mut ppa);

        PropertyAffordance::set_description(&mut *s_ref,desc);
        PropertyAffordance::set_title(&mut *s_ref,&Some(name.clone()));


        let mut frm  : Form = Form::new(href);
        frm.add_operation(FormOperationType::WriteProperty);
        frm.add_operation(FormOperationType::ReadProperty);
        s_ref.add_form(frm);
        



        let mut ppo = PropertyObject::new(name,thing.clone(),pa.clone());
        ppo.set_value(init_val);

        let mut t_thing = thing.write().unwrap();
        t_thing.add_property(name,ppo);

        let mut ta: Arc<RwLock<Box<dyn ThingDescription>>> = t_thing.get_description_mut();
        let td: &mut Box<dyn ThingDescription >= &mut ta.write().unwrap();
        
        td.add_property(name,pa.clone());


    }

}