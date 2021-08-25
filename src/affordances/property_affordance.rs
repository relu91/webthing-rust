use super::interaction_affordance::InteractionAffordance;
use super::interaction_affordance::InteractionAffordanceFactory;
use super::data_schema::DataSchema;
use super::json_object::JSonObject;
use super::json_object::JSonSerializer;
use super::w3c_list::W3CList;
use super::form;
use std::collections::btree_map::BTreeMap;

///1
pub trait PropertyAffordance : InteractionAffordance + DataSchema {
    ///1
    fn get_observable(&self) -> Option<bool>;
    ///1
    fn set_observable(&mut self, v :  Option<bool>>);
    
}

#[derive(Debug)]
struct BasePropertyAffordance {
    base_interaction : Box<dyn InteractionAffordance>,
    base_dataschema : Box<dyn DataSchema>,
    observable : Option<bool>
}

impl JSonObject for BasePropertyAffordance {
    fn to_json(&self) -> serde_json::Map<String, serde_json::Value> {
        let mut m  = self.baseInteraction.to_json();
        self.observable.copy("observable".to_string(),m);
        
        self.subscription.copy("subscription".to_string(),&mut m);
        self.subscription.copy("data".to_string(),&mut m);
        self.cancellation.copy("cancellation".to_string(),&mut m);
        m
    }
}

impl InteractionAffordance for BaseEventAffordance {
    fn get_description(&self) -> Option<String> {
        self.base.get_description()
    }
    ///1
    fn set_description(&mut self, v : Option<String>) {
        self.base.set_description(v);
    }

    ///1
    fn get_title(&self) -> Option<String> {
        self.base.get_title()
    }
    ///1
    fn set_title(&mut self, v : Option<String>) {
        self.base.set_title(v)
    }
    ///1
    fn get_i18n_title(&self, k: String) -> Option<&String> {
        self.base.get_i18n_title(k)
    }
    ///1
    fn set_i18n_title(&mut self, k : String , v: Option<String>) {
        self.base.set_i18n_title(k,v);
    }
    ///1
    fn get_i18n_description(&self, k: String) -> Option<&String> {
        self.base.get_i18n_description(k)
    }
    ///1
    fn set_i18n_description(&mut self, k : String , v: Option<String>) {
        self.base.set_i18n_description(k,v);
    }
    ///1 
    fn get_type(&self) -> &W3CList<String> {
        self.base.get_type()
    }
    ///1
    fn set_type(&mut self, v : &W3CList<String>) {
        self.base.set_type(v);
    }

    // specific interaction affordance data
    fn add_form(&mut self, f : form::Form) { 
        self.base.add_form(f);
    }


    fn remove_form(&mut self, f : form::Form) { 
        self.base.remove_form(f);
    }
    fn get_forms(&self) -> &std::vec::Vec<form::Form> { 
        self.base.get_forms()
    }

    fn clear_forms(&mut self) {
        self.base.clear_forms();
    }
    ///1 
    fn add_uri_variable(&mut self, n : String, d : Box<dyn DataSchema> ) {
        self.base.add_uri_variable(n,d);
    }
    ///1 
    fn remove_uri_variable(&mut self, n : String) {
        self.base.remove_uri_variable(n);
    }
    ///1 
    fn get_uri_variables(&self) -> &BTreeMap<String, Box<dyn DataSchema> > {
        self.get_uri_variables()
    }
    ///1
    fn clear_uri_variables(&mut self) {
        self.clear_uri_variables();
    }

}

impl EventAffordance for BaseEventAffordance {
    ///1
    fn get_subscription(&self) -> &Option<Box<dyn DataSchema>> {
        &self.subscription
    }
    ///1
    fn set_subscription(&mut self, v : Option<Box<dyn DataSchema>>) {
        self.subscription = v;
    }
    ///1
    fn get_data(&self) -> &Option<Box<dyn DataSchema>> {
        &self.data
    }
    ///1
    fn set_data(&mut self, v :Option<Box<dyn DataSchema>>) {
        self.data = v;
    }
    ///1
    fn get_cancellation(&self) -> &Option<Box<dyn DataSchema>> {
        &self.cancellation
    }
    ///1
    fn set_cancellation(&mut self, v :  Option<Box<dyn DataSchema>>) {
        self.cancellation = v;
    }

}


///1
pub struct EventAffordanceFactory {

}

///1
impl  EventAffordanceFactory {
    ///1
    pub fn new() ->  Box<dyn EventAffordance>  {
        let b = BaseEventAffordance { 
            base : InteractionAffordanceFactory::new(),
            subscription : None,
            data : None,
            cancellation : None
        };
        

     
        let ret = Box::new(b);
        ret

    }
}
