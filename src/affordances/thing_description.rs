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
use std::fmt::Debug;


///1
pub trait ThingDescription : Debug + JSonObject{
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
    fn get_i18n_title(&self, k: &String) -> Option<&String>;
    ///1
    fn set_i18n_title(&mut self, k : &String , v: Option<String>);
    ///1
    fn get_i18n_description(&self, k: &String) -> Option<&String>;
    ///1
    fn set_i18n_description(&mut self, k : &String , v: Option<String>);
    
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
    fn add_event(&mut self, k: &String, v : &Box<dyn EventAffordance>);
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
    fn get_links(&self) -> &BTreeSet<Link>;
    ///2
    fn set_links(&mut self, v : &BTreeSet<Link>);
    ///1
    fn clear_links(&mut self);
    ///1
    fn add_link(&mut self,v  : &Link );
    ///1
    fn remove_link(&mut self, k : &Url);
    ///1
    fn get_link(&self, k : &Url) -> Option<&Link>;


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


#[derive(Debug)]
struct BaseThingDescription {
    ctx     : W3CList<Url>,
    types   : W3CList<String>,
    id      : Option<Url>,
    title   : Option<String>,
    titles  : BTreeMap<String,String>,
    desc    : Option<String>,
    descs   : BTreeMap<String,String>,
    modified: Option<DateTime<Utc>>,
    base    : Option<Url>,
    support : Option<Url>,
    props   : BTreeMap<String, Box<dyn PropertyAffordance>>,
    evts    : BTreeMap<String, Box<dyn EventAffordance>>,
    acts    : BTreeMap<String, Box<dyn ActionAffordance>>,
    forms   : BTreeSet<Form>,
    links   : BTreeSet<Link>,
    sec     : W3CList<String>,
    sec_defs: BTreeMap<String, Box<dyn SecurityScheme>>
}

impl BaseThingDescription {
    pub fn new( u : &Url) -> Self {
        BaseThingDescription {
            ctx : W3CList::Single(u.clone()),
            types : W3CList::None, 
            id : None,
            title : None,
            titles : BTreeMap::new(),
            desc : None,
            descs : BTreeMap::new(),
            modified : None,
            base : None,
            support : None,
            props : BTreeMap::new(),
            evts : BTreeMap::new(),
            acts : BTreeMap::new(),
            forms : BTreeSet::new(),
            links : BTreeSet::new(),
            sec : W3CList::None,
            sec_defs : BTreeMap::new()

        }
    }
}


///1
/// 
impl JSonObject for BaseThingDescription {

    fn to_json(&self) -> serde_json::Map<std::string::String, serde_json::Value> { 
        let mut ret = serde_json::Map::new();
        self.ctx.copy("@context".to_string(), &mut ret );
        self.types.copy("@type".to_string(), &mut ret);
        self.id.copy("id".to_string(), &mut ret);
        self.title.copy("title".to_string(), &mut ret);
        self.titles.copy("titles".to_string(), &mut ret);
        self.desc.copy("description".to_string(), &mut ret);
        self.descs.copy("descriptions".to_string(), &mut ret);
        self.modified.copy("modified".to_string(), &mut ret);
        self.base.copy("base".to_string(), &mut ret);
        self.support.copy("support".to_string(), &mut ret);



        ret
    }
}

impl ThingDescription for BaseThingDescription {
    ///1
    fn get_context(&self) -> &W3CList<Url> {
        &self.ctx
    }
    ///1
    fn set_context(&mut self, v : &Url) {
        self.ctx = W3CList::Single(v.clone());
    }
    ///1
    fn set_context_list(&mut self, v : &W3CList<Url>) {
        self.ctx = v.clone();
    }
    ///1
    fn add_context(&mut self, v: &Url) {
/*        
        match self.ctx {
            W3CList::List (&mut val) => val.add(v),
            _ => return,
        }
*/        
    }
    ///1
    fn clear_context(&mut self) {
        self.ctx = W3CList::None;
    }
    ///1
    fn get_type(&self) -> &W3CList<String> {
        &self.types
    }
    ///1
    fn set_type(&mut self, v: &String) {
        self.types = W3CList::Single(v.clone());
    }
    ///2
    fn set_type_list(&mut self, v : &W3CList<String>) {
        self.types = v.clone();
    }
    ///1
    fn add_type(&mut self, v : &String) {
        
    }
    ///1
    fn clear_type(&mut self){
        self.types = W3CList::None;
    }
    

    ///1
    fn get_id(&self) -> &Option<Url> {
        return &self.id;
    }
    ///1
    fn set_id(&mut self, v : &Option<Url>) {
        self.id = v.clone();
    }


    ///1
    fn get_description(&self) -> &Option<String> {
        &self.desc
    }
    ///1
    fn set_description(&mut self, v : &Option<String>) {
        self.desc = v.clone();
    }
    ///1
    fn get_title(&self) -> &Option<String> {
        &self.title
    }
    ///1
    fn set_title(&mut self, v : &Option<String>) {
        self.title = v.clone();
    }
    ///1
    fn get_i18n_title(&self, k: &String) -> Option<&String> {
        self.titles.get(k)
    }
    ///1
    fn set_i18n_title(&mut self, k : &String , v: Option<String>) {
        match v {
            None => self.titles.remove(k),
            Some(x) => self.titles.insert(k.clone(), v.unwrap())
        };
        
    }
    ///1
    fn get_i18n_description(&self, k: &String) -> Option<&String> {
        self.descs.get(k)
    }
    ///1
    fn set_i18n_description(&mut self, k : &String , v: Option<String>) {
        match v {
            None => self.descs.remove(k),
            Some(x) => self.descs.insert(k.clone(), v.unwrap())
        };

    }
    
    ///1
    fn get_modified(&self) -> &Option<DateTime<Utc>> {
        &self.modified
    }
    ///1
    fn set_modified(&mut self, v : &Option<DateTime<Utc>>) {
        self.modified = v.clone();
    }

    ///1
    fn get_support(&self) -> &Option<Url> {
        &self.support
    }
    ///1
    fn set_support(&mut self, v : &Option<Url>) {
        self.support = v.clone();
    }

    ///1
    fn get_base(&self) -> &Option<Url> {
        &self.base
    }
    ///1
    fn set_base(&mut self, v : &Option<Url>) {
        self.base  = v.clone();
    }

    ///1
    fn get_properties(&self) -> &BTreeMap<String, Box<dyn PropertyAffordance>> {
        &self.props
    }
    fn set_properties(&mut self, v: &BTreeMap<String, Box<dyn PropertyAffordance>>) {
        self.props = *v.clone();
    }
    ///1
    ///1
    fn clear_properties(&mut self) {
        self.props.clear();
    }
    ///1
    fn add_property(&mut self, k: &String, v : &Box<dyn PropertyAffordance>) {
        self.props.insert(k.clone(),*v.clone());  
    }
    ///1
    fn remove_property(&mut self, v : &String) {
        self.props.remove(v);
    }
    ///1
    fn get_property(&self, k : &String) -> Option<&Box<dyn PropertyAffordance>> {
        self.props.get(k)
    }


    ///1
    fn get_events(&self) -> &BTreeMap<String, Box<dyn EventAffordance>> {
        &self.evts
    }
    ///1
    fn set_events(&mut self, v: &BTreeMap<String, Box<dyn EventAffordance>>) {
        self.evts = *v.clone();
    }
    ///1
    fn clear_events(&mut self) {
        self.evts.clear();
    }
    ///1
    fn add_event(&mut self, k: &String, v : &Box<dyn EventAffordance>) {
        self.evts.insert(k.clone(), *v.clone());
    }
    ///1
    fn remove_event(&mut self, v : &String) {
        self.evts.remove(v);
    }
    ///1
    fn get_event(&self, k : &String) -> Option<&Box<dyn EventAffordance>> {
        self.evts.get(k)
    }



    ///1
    fn get_actions(&self) -> &BTreeMap<String, Box<dyn ActionAffordance>> {
        &self.acts
    }
    ///1
    fn set_actions(&mut self, v: &BTreeMap<String, Box<dyn ActionAffordance>>) {
        self.acts = *v.clone();
    }
    ///1
    fn clear_actions(&mut self) {
        self.acts.clear();
    }
    ///1
    fn add_action(&mut self, k: &String, v : &Box<dyn ActionAffordance>) {
        self.acts.insert(k.clone(), *v.clone());
    }
    ///1
    fn remove_action(&mut self, v : &String) {
        self.acts.remove(v);
    }
    ///1
    fn get_action(&self, k : &String) -> Option<&Box<dyn ActionAffordance>> {
        self.acts.get(k)
    }


    ///1
    fn get_links(&self) -> &BTreeSet<Link> {
        &self.links
    }
    ///2
    fn set_links(&mut self, v : &BTreeSet<Link>) {
        self.links = v.clone();
    }
    ///1
    fn clear_links(&mut self) {
        self.links.clear();
    }
    ///1
    fn add_link(&mut self,v  : &Link ) {
        self.links.insert(v.clone());
    }
    ///1
    fn remove_link(&mut self, k : &Url) {
        self.links.remove(&Link::new(k));
    }
    ///1
    fn get_link(&self, k : &Url) -> Option<&Link> {
        self.links.get(&Link::new(k))
    }


    ///1
    fn get_forms(&self) -> &BTreeSet<Form> {
        &self.forms
    }
    ///2
    fn set_forms(&mut self, v : &BTreeSet<Form>) {
        self.forms = *v.clone();
    }
    ///1
    fn clear_forms(&mut self) {
        self.forms.clear();
    }
    ///1
    fn add_form(&mut self,v  : &Form ) {
        self.forms.insert(*v.clone());
    }
    ///1
    fn remove_form(&mut self, k : &Url) {
        self.forms.remove(&Form::new(k));
    }
    ///1
    fn get_form(&self, k : &Url) -> Option<&Form> {
        self.forms.get(&Form::new(k))
    }


    ///1
    fn get_security(&self) -> &W3CList<String> {
        &self.sec
    }
    ///1
    fn set_security(&mut self, v : &W3CList<String>) {
        self.sec = v.clone();
    }

    //1
    fn get_security_definitions(&self) -> &BTreeMap<String, Box<dyn SecurityScheme>> {
        &self.sec_defs
    }
    //1
    fn set_security_definitions(&mut self, v :  &BTreeMap<String, Box<dyn SecurityScheme>>) {
        self.sec_defs = *v.clone();
    }
    ///1
    fn clear_security_definitions(&mut self) {
        self.sec_defs.clear();
    }
    ///1
    fn add_security_definition(&mut self, k: &String, v : &Box<dyn SecurityScheme>) {
        self.sec_defs.insert(k.clone(), *v.clone());
    }
    ///2
    fn remove_security_definition(&mut self, k : &String) {
        self.sec_defs.remove(k);
    }
    ///1
    fn get_security_definition(&self, k: &String) -> Option<&Box<dyn SecurityScheme>> {
        self.sec_defs.get(k)
    }
    
}
pub struct ThingDescriptionFactory {

}

impl ThingDescriptionFactory {
    ///1
    pub fn new(ctx : &Url ) -> Box<dyn ThingDescription> {
        Box::new(BaseThingDescription::new(ctx))
    }
}