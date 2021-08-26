use super::w3c_list::W3CList;
use url::Url;
use std::collections::btree_map::BTreeMap;
use super::json_object::JSonObject;

#[derive(Debug,Clone)] 
///1
pub enum SecuritySchemeId {
    ///1
    nosec, 
    ///1
    basic,
    ///1 
    digest, 
    ///1
    bearer, 
    ///1
    psk,
    ///1 
    oauth2,
    ///1
    apikey    
}

///1
pub trait SecurityScheme : Debug + JSonObject  {
    ///1
    fn get_description(&self) -> &Option<String>;
    ///1
    fn set_description(&mut self, v : &Option<String>);
    ///1
    fn get_i18n_description(&self, k: &String) -> Option<&String>;
    ///1
    fn set_i18n_description(&mut self, k : &String , v: &Option<String>);
    ///1 
    fn get_type(&self) -> &W3CList<String>;
    ///1
    fn set_type(&mut self, v : &W3CList<String>);
    ///1
    fn get_id(&self) -> SecuritySchemeId;
    ///1
    fn get_proxy(&self) -> &Option<Url>;
    ///1
    fn set_proxy(&mut self, v : &Option<Url>);
}

struct BaseSecurityScheme {
    description  : Option<String>,
    descriptions : BTreeMap<String,String> ,
    stype        : W3CList<String>,
    id           : SecuritySchemeId,
    proxy        : Option<Url>

}

impl JSonObject for BaseSecurityScheme {
    fn to_json(&self) -> serde_json::Map<String, serde_json::Value> {
        let mut m = serde_json::Map::new();
        self.description.copy("description".to_string(), m);
        self.descriptions.copy("descriptions".to_string(), m);
        self.stype.copy("@type".to_string(), m);
        m
    }
}
impl SecurityScheme for BaseSecurityScheme {
    ///1
    fn get_description(&self) -> &Option<String> {
        &self.description
    }
    ///1
    fn set_description(&mut self, v : &Option<String>) {
        self.description = v.clone();
    }
    ///1
    fn get_i18n_description(&self, k: &String) -> Option<&String> {
        self.descriptions.get(k)
    }
    ///1
    fn set_i18n_description(&mut self, k : &String , v: &Option<String>) {
        match v {
            None => self.descriptions.remove(k),
            Some(x) => self.descriptions.insert(k.to_string(),x.to_string())
        };

    }
    ///1 
    fn get_type(&self) -> &W3CList<String> {
        &self.stype
    }
    ///1
    fn set_type(&mut self, v : &W3CList<String>) {
        self.stype = v.clone();
    }
    ///1
    fn get_id(&self) -> SecuritySchemeId {
        self.id.clone()
    }
    ///1
    fn get_proxy(&self) -> &Option<Url> {
        &self.proxy
    }
    ///1
    fn set_proxy(&mut self, v : &Option<Url>) {
        self.proxy = v.clone();
    }

}
impl BaseSecurityScheme {
    pub fn new(i : SecuritySchemeId) -> Self {
        BaseSecurityScheme  { 
            description : None, 
            descriptions : BTreeMap::new(), 
            stype :  W3CList::None,
            id : i,
            proxy : None
        }
    }
}


pub struct SecuritySchemeFactory {
    
}
//specific security schema traits

pub trait BasicSecurityScheme : SecurityScheme {
    
}
impl SecuritySchemeFactory {
    pub fn new(id : SecuritySchemeId) -> Box<dyn SecurityScheme> {
        match id {
            SecuritySchemeId::nosec => Box::new(BaseSecurityScheme::new(id)),

        }
    }
}