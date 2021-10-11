use super::interaction_affordance::InteractionAffordance;
use super::interaction_affordance::InteractionAffordanceFactory;
use super::data_schema::DataSchemaFactory;
use super::data_schema::DataSchema;
use super::data_schema::DataSchemaId;
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
    fn set_observable(&mut self, v :  Option<bool>);

    ///1 
    fn set_data_schema(&mut self, v: Box<dyn DataSchema>);

    ///1
    fn set_interaction(&mut self, v: Box<dyn InteractionAffordance>);

    ///1
    fn get_description(&self) -> &Option<String>;
    ///1
    fn set_description(&mut self, v : &Option<String>);
    ///1
    fn get_title(&self) -> &Option<String>;
    ///1
    fn set_title(&mut self, v : &Option<String>);
    
    
}

#[derive(Debug)]
struct BasePropertyAffordance {
    base_interaction : Box<dyn InteractionAffordance>,
    base_dataschema : Box<dyn DataSchema>,
    observable : Option<bool>
}

impl JSonObject for BasePropertyAffordance {
    fn to_json(&self) -> serde_json::Map<String, serde_json::Value> {
        let mut m  = self.base_interaction.to_json();
        self.observable.copy("observable".to_string(),&mut m);
        //now, copy from base data schema everything except descriptive data
        let m2 = self.base_dataschema.to_json();

        for (key, value) in m2 {
            match key.as_ref() {
                "title"  => (),
                "titles" => (),
                "description" => (),
                "descriptions" => (),
                "@type" => (),
                _ =>  { m.insert(key,value); },
            }
        }
        m
    }
}

impl InteractionAffordance for BasePropertyAffordance {
    fn get_description(&self) -> &Option<String> {
        self.base_interaction.get_description()
    }
    ///1
    fn set_description(&mut self, v : &Option<String>) {
        self.base_interaction.set_description(v);
    }

    ///1
    fn get_title(&self) -> &Option<String> {
        self.base_interaction.get_title()
    }
    ///1
    fn set_title(&mut self, v : &Option<String>) {
        self.base_interaction.set_title(v)
    }
    ///1
    fn get_i18n_title(&self, k: String) -> Option<&String> {
        self.base_interaction.get_i18n_title(k)
    }
    ///1
    fn set_i18n_title(&mut self, k : String , v: Option<String>) {
        self.base_interaction.set_i18n_title(k,v);
    }
    ///1
    fn get_i18n_description(&self, k: String) -> Option<&String> {
        self.base_interaction.get_i18n_description(k)
    }
    ///1
    fn set_i18n_description(&mut self, k : String , v: Option<String>) {
        self.base_interaction.set_i18n_description(k,v);
    }
    ///1 
    fn get_type(&self) -> &W3CList<String> {
        self.base_interaction.get_type()
    }
    ///1
    fn set_type(&mut self, v : &W3CList<String>) {
        self.base_interaction.set_type(v);
    }

    // specific interaction affordance data
    fn add_form(&mut self, f : form::Form) { 
        self.base_interaction.add_form(f);
    }


    fn remove_form(&mut self, f : form::Form) { 
        self.base_interaction.remove_form(f);
    }
    fn get_forms(&self) -> &std::vec::Vec<form::Form> { 
        self.base_interaction.get_forms()
    }

    fn clear_forms(&mut self) {
        self.base_interaction.clear_forms();
    }
    ///1 
    fn add_uri_variable(&mut self, n : String, d : Box<dyn DataSchema> ) {
        self.base_interaction.add_uri_variable(n,d);
    }
    ///1 
    fn remove_uri_variable(&mut self, n : String) {
        self.base_interaction.remove_uri_variable(n);
    }
    ///1 
    fn get_uri_variables(&self) -> &BTreeMap<String, Box<dyn DataSchema> > {
        self.base_interaction.get_uri_variables()
    }
    ///1
    fn clear_uri_variables(&mut self) {
        self.base_interaction.clear_uri_variables();
    }

}


impl DataSchema for BasePropertyAffordance {
    fn get_description(&self) -> Option<String> {
        self.base_dataschema.get_description()
    }
    ///1
    fn set_description(&mut self, v : Option<String>) {
        self.base_dataschema.set_description(v);
    }

    ///1
    fn get_title(&self) -> Option<String> {
        self.base_dataschema.get_title()
    }
    ///1
    fn set_title(&mut self, v : Option<String>) {
        self.base_dataschema.set_title(v);
    }
    ///1
    fn get_i18n_title(&self, k: String) -> Option<&String> {
        self.base_dataschema.get_i18n_title(k)
    }
    ///1
    fn set_i18n_title(&mut self, k : String , v: Option<String>) {
        self.base_dataschema.set_i18n_title(k,v);
        
    }
    ///1
    fn get_i18n_description(&self, k: String) -> Option<&String> {
        self.base_dataschema.get_i18n_description(k)
    }
    ///1
    fn set_i18n_description(&mut self, k : String , v: Option<String>) {
        self.base_dataschema.set_i18n_description(k,v);
    }
    ///1 
    fn get_type(&self) -> &W3CList<String> {
        self.base_dataschema.get_type()
    }
    ///1
    fn set_type(&mut self, v : &W3CList<String>) {
        self.base_dataschema.set_type(v);
    }
    ///1
    fn get_schema_type(&self) -> Option<DataSchemaId> {
        self.base_dataschema.get_schema_type()
    }
    ///1
    fn get_unit(&self) -> Option<String> {
        self.base_dataschema.get_unit()
    }
    ///1
    fn set_unit(&mut self , v : Option<String>) {
        self.base_dataschema.set_unit(v);
    }
    ///1
    fn add_oneof(&mut self, v: Box<dyn DataSchema>) {
        self.base_dataschema.add_oneof(v);
    }
    ///1
    fn get_oneof_list(&self) -> &Vec<Box<dyn DataSchema>> {
        self.base_dataschema.get_oneof_list()
    }
    ///1
    fn remove_oneof(&mut self, i : i32) {
        self.base_dataschema.remove_oneof(i);
    }
    ///1
    fn get_format(&self) -> Option<String> {
        self.base_dataschema.get_format()
    }
    ///1
    fn set_format(&mut self , v : Option<String>) {
        self.base_dataschema.set_format(v);
    }
    ///1
    fn get_readonly(&self) -> Option<bool> {
        self.base_dataschema.get_readonly()
    }
    ///1
    fn set_readonly(&mut self, v : Option<bool>) {
        self.base_dataschema.set_readonly(v);
    }
    ///1
    fn get_writeonly(&self) ->Option< bool> {
        self.base_dataschema.get_writeonly()
    }
    ///1
    fn set_writeonly(&mut self, v : Option<bool>) {
        self.base_dataschema.set_writeonly(v);
    }

}
impl PropertyAffordance for BasePropertyAffordance {
    ///1
    fn get_observable(&self) -> Option<bool> {
        self.observable
    }
    ///1
    fn set_observable(&mut self, v :  Option<bool>) {
        self.observable = v.clone();
    }

    fn set_data_schema(&mut self, v: Box<dyn DataSchema>) {
        self.base_dataschema = v;
    }
    fn set_interaction(&mut self, v: Box<dyn InteractionAffordance>) {
        self.base_interaction = v;
    }
    fn get_description(&self) -> &Option<String> {
        self.base_interaction.get_description()
    }
    ///1
    fn set_description(&mut self, v : &Option<String>) {
        self.base_interaction.set_description(v);
    }

    ///1
    fn get_title(&self) -> &Option<String> {
        self.base_interaction.get_title()
    }
    ///1
    fn set_title(&mut self, v : &Option<String>) {
        self.base_interaction.set_title(v)
    }

}


///1
pub struct PropertyAffordanceFactory {

}

///1
impl  PropertyAffordanceFactory {
    ///1
    pub fn new() ->  Box<dyn PropertyAffordance>  {
        let b = BasePropertyAffordance { 
            base_interaction : InteractionAffordanceFactory::new(),
            base_dataschema : DataSchemaFactory::new(None),
            observable : None
        };
        

     
        let ret = Box::new(b);
        ret

    }
}
