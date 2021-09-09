use super::w3c_list::W3CList;
use url::Url;
use chrono::DateTime;
use chrono::Utc;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use super::action_affordance::ActionAffordance;
use super::event_affordance::EventAffordance;
use super::property_affordance::PropertyAffordance;
use super::form::Form;
use super::link::Link;
use super::json_object::JSonObject;
use super::json_object::JSonSerializer;
use super::security_scheme::SecurityScheme;
use super::w3c_list::W3CList;


///1
pub trait Thing {
    ///1
    fn get_context(&self) -> &W3CList<Url>;
    ///1
    fn set_context(&mut self, v : &Url);
    ///1
    fn set_context_list(&mut self, v : &W3CList<Url>);
    ///1
    fn add_context(&mut self, v: &Url);
    ///1
    fn clear_context(&mut self);
    ///1
    fn get_type(&self) -> &W3CList<String>;
    ///1
    fn set_type(&mut self, v: &String);
    ///2
    fn set_type_list(&mut self, v : &W3CList<String>);
    ///1
    fn add_type(&mut self, v : &String);
    ///1
    fn clear_type(&mut self);
    

    ///1
    fn get_id(&self) -> &Option<Url>;
    ///1
    fn set_id(&mut self, v : &Option<Url>);


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
    fn get_modified(&self) -> &Option<DateTime<Utc>>;
    ///1
    fn set_modified(&mut self, v : &Option<DateTime<Utc>>);

    ///1
    fn get_support(&self) -> &Option<Url>;
    ///1
    fn set_support(&mut self, v : &Option<Url>);

    ///1
    fn get_base(&self) -> &Option<Url>;
    ///1
    fn set_base(&mut self, v : &Option<Url>);

    ///1
    fn get_properties(&self) -> &BTreeMap<String, Box<dyn PropertyAffordance>>;
    ///1
    fn set_properties(&mut self, v: &BTreeMap<String, Box<dyn PropertyAffordance>>);
    ///1
    fn clear_properties(&mut self);
    ///1
    fn add_property(&mut self, k: &String, v : &Box<dyn PropertyAffordance>);
    ///1
    fn remove_property(&mut self, v : &String);
    ///1
    fn get_property(&self, k : &String) -> Option<&Box<dyn PropertyAffordance>>;


    ///1
    fn get_events(&self) -> &BTreeMap<String, Box<dyn EventAffordance>>;
    ///1
    fn set_events(&mut self, v: &BTreeMap<String, Box<dyn EventAffordance>>);
    ///1
    fn clear_events(&mut self);
    ///1
    fn add_event(&mut self, k: &String v : &Box<dyn EventAffordance>);
    ///1
    fn remove_event(&mut self, v : &String);
    ///1
    fn get_event(&self, k : &String) -> Option<&Box<dyn EventAffordance>>;



    ///1
    fn get_actions(&self) -> &BTreeMap<String, Box<dyn ActionAffordance>>;
    ///1
    fn set_actions(&mut self, v: &BTreeMap<String, Box<dyn ActionAffordance>>);
    ///1
    fn clear_actions(&mut self);
    ///1
    fn add_action(&mut self, k: &String, v : &Box<dyn ActionAffordance>);
    ///1
    fn remove_action(&mut self, v : &String);
    ///1
    fn get_action(&self, k : &String) -> Option<&Box<dyn ActionAffordance>>;


    ///1
    fn get_links(&self) -> &BTreeSet<Box < dyn Link>>;
    ///2
    fn set_links(&mut self, v : &BTreeSet<Box < dyn Link>>);
    ///1
    fn clear_links(&mut self);
    ///1
    fn add_link(&mut self,v  : &Box < dyn Link> );
    ///1
    fn remove_link(&mut self, k : &Url);
    ///1
    fn get_link(&self, k : &Url) -> Option<&Box< dyn Link>>;


    ///1
    fn get_forms(&self) -> &BTreeSet<Form>;
    ///2
    fn set_forms(&mut self, v : &BTreeSet<Form>);
    ///1
    fn clear_forms(&mut self);
    ///1
    fn add_form(&mut self,v  : &Form );
    ///1
    fn remove_form(&mut self, k : &Url);
    ///1
    fn get_form(&self, k : &Url) -> Option<&Form>;


    ///1
    fn get_security(&self) -> &W3CList<String>;
    ///1
    fn set_security(&mut self, v : &W3CList<String>);

    //1
    fn get_security_definitions(&self) -> &BTreeMap<String, Box<dyn SecurityScheme>>;
    //1
    fn set_security_definitions(&mut self, v :  &BTreeMap<String, Box<dyn SecurityScheme>>);
    ///1
    fn clear_security_definitions(&mut self);
    ///1
    fn add_security_definition(&mut self, k: &String, v : &Box<dyn SecurityScheme>);
    ///2
    fn remove_security_definition(&mut self, k : &String);
    ///1
    fn get_security_definition(&self, k: &String) -> Option<&Box<dyn SecurityScheme>>;
    
    


}