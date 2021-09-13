use std::fmt::Debug;
use super::form;
use super::descriptive_data;
use super::w3c_list::W3CList;
use super::json_object::JSonObject;
use super::json_object::JSonSerializer;
use super::data_schema::DataSchemaFactory;
use super::data_schema::DataSchema;
use std::collections::btree_map::BTreeMap;




///1
pub trait InteractionAffordance :  Debug + JSonObject  {
    ///1
    fn get_description(&self) -> &Option<String>;
    ///1
    fn set_description(&mut self, v : &Option<String>);
    ///1
    fn get_title(&self) -> &Option<String>;
    ///1
    fn set_title(&mut self, v : &Option<String>);
    ///1
    fn get_i18n_title(&self, k: String) -> Option<&String>;
    ///1
    fn set_i18n_title(&mut self, k : String , v: Option<String>);
    ///1
    fn get_i18n_description(&self, k: String) -> Option<&String>;
    ///1
    fn set_i18n_description(&mut self, k : String , v: Option<String>);
    ///1 
    fn get_type(&self) -> &W3CList<String>;
    ///1
    fn set_type(&mut self, v : &W3CList<String>);

    ///1
    fn add_form(&mut self,f : form::Form);
    ///1
    fn remove_form(&mut self, f: form::Form);
    ///1
    fn get_forms(&self) -> &Vec<form::Form>;
    ///1
    fn clear_forms(&mut self);


    ///1 
    fn add_uri_variable(&mut self, n : String, d : Box<dyn DataSchema> );
    ///1 
    fn remove_uri_variable(&mut self, n : String);
    ///1 
    fn get_uri_variables(&self) -> &BTreeMap<String, Box<dyn DataSchema> >;
    ///1
    fn clear_uri_variables(&mut self);



}
///1
pub struct InteractionAffordanceFactory {
}
///1
impl InteractionAffordanceFactory {
    ///1
    pub fn new() -> Box<dyn InteractionAffordance> {
        return Box::new(BaseInteractionAffordance::new());
    }
}

#[derive(Debug)]
struct BaseInteractionAffordance {
    desc_data : descriptive_data::DescriptiveData,
    forms   : Vec<form::Form>,
    uri_variables : BTreeMap<String, Box<dyn DataSchema> >,
}

impl BaseInteractionAffordance {
    fn new() -> Self {
        Self {
            desc_data : descriptive_data::DescriptiveData::new(),
            forms : Vec::new(),
            uri_variables : BTreeMap::new()
        }    
    
    }
    
}
impl JSonObject for BaseInteractionAffordance {
    fn to_json(&self) -> serde_json::Map<String, serde_json::Value> {
        let mut ret  = serde_json::Map::new();

        self.desc_data.copy("".to_string(), &mut ret);
        self.forms.copy("forms".to_string(),&mut ret);

        
        self.uri_variables.copy("uriVariables".to_string(),&mut ret);
        

        ret
    }
}
impl InteractionAffordance for BaseInteractionAffordance {
    // from base Descriptive Data 
    ///1
    fn get_description(&self) -> &Option<String> {
        &self.desc_data.description
    }
    ///1
    fn set_description(&mut self, v : &Option<String>) {
        self.desc_data.description = v.clone();
    }

    ///1
    fn get_title(&self) -> &Option<String> {
        &self.desc_data.title
    }
    ///1
    fn set_title(&mut self, v : &Option<String>) {
        self.desc_data.title = v.clone();
    }
    ///1
    fn get_i18n_title(&self, k: String) -> Option<&String> {
        self.desc_data.titles.get(&k)
    }
    ///1
    fn set_i18n_title(&mut self, k : String , v: Option<String>) {
        match v {
            None => self.desc_data.titles.remove(&k),
            Some(x) => self.desc_data.titles.insert(k,x)
        };
        
    }
    ///1
    fn get_i18n_description(&self, k: String) -> Option<&String> {
        self.desc_data.descriptions.get(&k)
    }
    ///1
    fn set_i18n_description(&mut self, k : String , v: Option<String>) {
        match v {
            None => self.desc_data.descriptions.remove(&k),
            Some(x) => self.desc_data.descriptions.insert(k,x)
        };

    }
    ///1 
    fn get_type(&self) -> &W3CList<String> {
        &self.desc_data.stype
    }
    ///1
    fn set_type(&mut self, v : &W3CList<String>) {
        self.desc_data.stype = v.clone();
    }

    // specific interaction affordance data
    fn add_form(&mut self, f : form::Form) { 
        self.forms.push(f);
    }


    fn remove_form(&mut self, f : form::Form) { 
        self.forms.retain(|x| x.href != f.href);
    }
    fn get_forms(&self) -> &std::vec::Vec<form::Form> { 
        return &self.forms;
    }

    fn clear_forms(&mut self) {
        self.forms.clear();
    }
    ///1 
    fn add_uri_variable(&mut self, n : String, d : Box<dyn DataSchema> ) {
        self.uri_variables.insert(n,d);
    }
    ///1 
    fn remove_uri_variable(&mut self, n : String) {
        self.uri_variables.remove(&n);
    }
    ///1 
    fn get_uri_variables(&self) -> &BTreeMap<String, Box<dyn DataSchema> > {
        &self.uri_variables
    }
    ///1
    fn clear_uri_variables(&mut self) {
        self.uri_variables.clear();
    }

}