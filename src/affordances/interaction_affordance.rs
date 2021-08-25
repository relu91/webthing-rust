use std::fmt::Debug;
use super::form;
use super::descriptive_data;
use super::w3c_list::W3CList;
use super::json_object::JSonObject;
use super::json_object::JSonSerializer;




///1
pub trait InteractionAffordance :  Debug + JSonObject  {
    ///1
    fn get_description(&self) -> Option<String>;
    ///1
    fn set_description(&mut self, v : Option<String>);
    ///1
    fn get_title(&self) -> Option<String>;
    ///1
    fn set_title(&mut self, v : Option<String>);
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


}
///1
pub struct InteractionAffordanceFactory {
}
///1
impl InteractionAffordanceFactory {
    ///1
    pub fn make() -> Box<dyn InteractionAffordance> {
        return Box::new(BaseInteractionAffordance::new());
    }
}

#[derive(Debug)]
struct BaseInteractionAffordance {
    desc_data : descriptive_data::DescriptiveData,
    forms   : Vec<form::Form>,
}

impl BaseInteractionAffordance {
    fn new() -> Self {
        Self {
            desc_data : descriptive_data::DescriptiveData::new(),
            forms : Vec::new()
        }    
    
    }
    
}
impl JSonObject for BaseInteractionAffordance {
    fn to_json(&self) -> serde_json::Map<String, serde_json::Value> {
        let mut ret  = serde_json::Map::new();

        self.desc_data.copy("".to_string(), &mut ret);

        ret
    }
}
impl InteractionAffordance for BaseInteractionAffordance {
    // from base Descriptive Data 
    ///1
    fn get_description(&self) -> Option<String> {
        self.desc_data.description.clone()
    }
    ///1
    fn set_description(&mut self, v : Option<String>) {
        self.desc_data.description = v.clone();
    }

    ///1
    fn get_title(&self) -> Option<String> {
        self.desc_data.title.clone()
    }
    ///1
    fn set_title(&mut self, v : Option<String>) {
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
        self.forms.retain(|x| *x.get_href() != f.get_href());
    }
    fn get_forms(&self) -> &std::vec::Vec<form::Form> { 
        return &self.forms;
    }

    fn clear_forms(&mut self) {
        self.forms.clear();
    }

}