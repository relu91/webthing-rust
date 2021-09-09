use super::interaction_affordance::InteractionAffordance;
use super::interaction_affordance::InteractionAffordanceFactory;
use super::data_schema::DataSchema;
use super::json_object::JSonObject;
use super::json_object::JSonSerializer;
use super::w3c_list::W3CList;
use super::form;
use std::collections::btree_map::BTreeMap;

///1
pub trait ActionAffordance : InteractionAffordance {
    ///1
    fn get_input(&self) -> &Option<Box<dyn DataSchema>>;
    ///1
    fn set_input(&mut self, v : Option<Box<dyn DataSchema>>);
    ///1
    fn get_output(&self) -> &Option<Box<dyn DataSchema>>;
    ///1
    fn set_output(&mut self, v :Option<Box<dyn DataSchema>>);
    ///1
    fn get_idempotent(&self) -> Option<bool>;
    ///1
    fn set_idempotent(&mut self, v :  Option<bool>);
    ///1
    fn get_safe(&self) -> Option<bool>;
    ///1
    fn set_safe(&mut self, v :  Option<bool>);
    
}

#[derive(Debug)]
struct BaseActionAffordance {
    base : Box<dyn InteractionAffordance>,
    input : Option<Box<dyn DataSchema>>,
    output : Option<Box<dyn DataSchema>>,
    safe : Option<bool>,
    idempotent : Option<bool>
}

impl JSonObject for BaseActionAffordance {
    fn to_json(&self) -> serde_json::Map<String, serde_json::Value> {
        let mut m  = self.base.to_json();
        self.input.copy("input".to_string(),&mut m);
        self.input.copy("output".to_string(),&mut m);
        self.safe.copy("safe".to_string(),&mut m);
        self.idempotent.copy("idempotent".to_string(),&mut m);
        m
    }
}

impl InteractionAffordance for BaseActionAffordance {
    fn get_description(&self) -> &Option<String> {
        self.base.get_description()
    }
    ///1
    fn set_description(&mut self, v : &Option<String>) {
        self.base.set_description(v);
    }

    ///1
    fn get_title(&self) -> &Option<String> {
        self.base.get_title()
    }
    ///1
    fn set_title(&mut self, v : &Option<String>) {
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

impl ActionAffordance for BaseActionAffordance {
    ///1
    fn get_input(&self) -> &Option<Box<dyn DataSchema>> {
        &self.input
    }
    ///1
    fn set_input(&mut self, v : Option<Box<dyn DataSchema>>) {
        self.input = v;
    }
    ///1
    fn get_output(&self) -> &Option<Box<dyn DataSchema>> {
        &self.output
    }
    ///1
    fn set_output(&mut self, v :Option<Box<dyn DataSchema>>) {
        self.output = v;
    }
    ///1
    fn get_idempotent(&self) -> Option<bool> {
        self.idempotent
    }
    ///1
    fn set_idempotent(&mut self, v :  Option<bool>) {
        self.idempotent = v.clone();
    }
    ///1
    fn get_safe(&self) -> Option<bool> {
        self.safe
    }
    ///1
    fn set_safe(&mut self, v :  Option<bool>) {
        self.safe = v.clone();
    }

}


///1
pub struct ActionAffordanceFactory {

}

///1
impl  ActionAffordanceFactory {
    ///1
    pub fn new() ->  Box<dyn ActionAffordance>  {
        let b = BaseActionAffordance { 
            base : InteractionAffordanceFactory::new(),
            input : None,
            output : None,
            safe : None,
            idempotent : None
        };
        

     
        let ret = Box::new(b);
        ret

    }
}
