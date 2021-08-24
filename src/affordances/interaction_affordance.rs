
use super::form;
use super::descriptive_data;
use super::w3c_list::W3CList;


use serde::{Serialize, Deserialize};

///1
pub trait InteractionAffordance {
    ///1
    fn get_title(&self) -> String;
    ///1
    fn set_title(&mut self, v : String);
    ///1
    fn get_i18n_title(&self, k: String) -> String;
    ///1
    fn set_i18n_title(&mut self, k : String , v: String);
    ///1
    fn get_i18n_description(&self, k: String) -> String;
    ///1
    fn set_i18n_description(&mut self, k : String , v: String);
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


#[derive(Serialize,Debug)]
#[serde(rename_all = "camelCase")] 
pub (crate) struct BaseInteractionAffordance {
    #[serde(flatten)]
    desc_data : descriptive_data::DescriptiveData,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
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
impl InteractionAffordance for BaseInteractionAffordance {
    // from base Descriptive Data 
    fn get_title(&self) -> std::string::String { 
        self.desc_data.title.clone()
    }
    fn set_title(&mut self, v : std::string::String) { 
        self.desc_data.title = v.clone();

    }
    fn get_i18n_title(&self, k :  std::string::String) -> std::string::String { 
        if self.desc_data.titles.contains_key(&k) {
            let z  =  self.desc_data.titles.get(&k);
            if z.is_some() {
                return z.unwrap().clone();
            }
        } 

        "".to_string()
    }
    fn set_i18n_title(&mut self, k :  std::string::String, v :  std::string::String) { 
        self.desc_data.titles.insert(k,v);
    }
    fn get_i18n_description(&self, k :  std::string::String) -> std::string::String { 
        if self.desc_data.descriptions.contains_key(&k) {
            let z  =  self.desc_data.descriptions.get(&k);
            if z.is_some() {
                return z.unwrap().clone();
            }
        } 

        "".to_string()
    }
    fn set_i18n_description(&mut self, k :  std::string::String, v :  std::string::String) { 
        self.desc_data.descriptions.insert(k,v);
    }
    fn get_type(&self) -> &W3CList<String> { 
        &self.desc_data.stype
    }
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