use std::fmt::Debug;
use super::w3c_list::W3CList;
use url::Url;
use std::collections::btree_map::BTreeMap;
use super::json_object::JSonObject;
use super::json_object::JSonSerializer;


#[derive(Debug,Clone)] 
///1
pub enum SecuritySchemeId {
    ///1
    Nosec, 
    ///1
    Basic,
    ///1 
    Digest, 
    ///1
    Bearer, 
    ///1
    PSK,
    ///1 
    OAuth2,
    ///1
    ApiKey    
}


///1
#[derive(Debug,Clone)] 
pub enum SecuritySchemeInLocation {
    ///1
    Header,
    ///1
    Query,
    ///1
    Body,
    ///1
    Cookie
}

///1
pub trait SecurityScheme :  Debug +  JSonObject  {
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

///1


#[derive(Debug)]
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
        self.description.copy("description".to_string(), &mut m);
        self.descriptions.copy("descriptions".to_string(),&mut m);
        self.stype.copy("@type".to_string(), &mut m);
        self.id.copy("id".to_string(), &mut m);
        self.proxy.copy("proxy".to_string(), &mut m);
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

///specific security schema traits factory
pub struct SecuritySchemeFactory {
    
}
///Scheme for HTTP basic authentication
pub trait BasicSecurityScheme : SecurityScheme {
    ///1
    fn get_in(&self) -> &SecuritySchemeInLocation;
    ///1
    fn set_in(&mut self, v : &SecuritySchemeInLocation);
    ///1
    fn get_name(&self) -> &Option<String>;
    ///1
    fn set_name(&mut self, v: &Option<String>);
}

///1
#[derive(Debug,Clone)] 
///2
pub enum SecuritySchemeDigestQOP {
    ///2
    Auth,
    ///2 
    AuthInt
}
///1
pub trait DigestSecurityScheme : BasicSecurityScheme {
    ///1
    fn get_qop(&self) -> SecuritySchemeDigestQOP;
    ///1
    fn set_qop(&mut self, v : SecuritySchemeDigestQOP);
}
#[derive(Debug)] 
struct BaseBasicSecurityScheme {
    base        : BaseSecurityScheme,
    inLocation  : SecuritySchemeInLocation,
    name        : Option<String>

}

impl JSonObject for BaseBasicSecurityScheme {
    
    fn to_json(&self) -> serde_json::Map<String, serde_json::Value> {
        let mut m  = self.base.to_json();
        self.name.copy("name".to_string(), &mut m);
        self.inLocation.copy("in".to_string(), &mut m);

        m
    }
}
impl SecurityScheme for BaseBasicSecurityScheme {
    ///1
    fn get_description(&self) -> &Option<String> {
        &self.base.description
    }
    ///1
    fn set_description(&mut self, v : &Option<String>) {
        self.base.description = v.clone();
    }
    ///1
    fn get_i18n_description(&self, k: &String) -> Option<&String> {
        self.base.descriptions.get(k)
    }
    ///1
    fn set_i18n_description(&mut self, k : &String , v: &Option<String>) {
        match v {
            None => self.base.descriptions.remove(k),
            Some(x) => self.base.descriptions.insert(k.to_string(),x.to_string())
        };

    }
    ///1 
    fn get_type(&self) -> &W3CList<String> {
        &self.base.stype
    }
    ///1
    fn set_type(&mut self, v : &W3CList<String>) {
        self.base.stype = v.clone();
    }
    ///1
    fn get_id(&self) -> SecuritySchemeId {
        self.base.id.clone()
    }
    ///1
    fn get_proxy(&self) -> &Option<Url> {
        &self.base.proxy
    }
    ///1
    fn set_proxy(&mut self, v : &Option<Url>) {
        self.base.proxy = v.clone();
    }

}

impl BasicSecurityScheme for BaseBasicSecurityScheme {
    ///1
    fn get_in(&self) -> &SecuritySchemeInLocation {
        &self.inLocation
    }
    ///1
    fn set_in(&mut self, v : &SecuritySchemeInLocation) {
        &self.inLocation;
    }
    ///1
    fn get_name(&self) -> &Option<String> {
        &self.name
    }
    ///1
    fn set_name(&mut self, v: &Option<String>) {
        self.name = v.clone();
    }

}

impl BaseBasicSecurityScheme {
    pub fn new() -> Self {
        BaseBasicSecurityScheme {
            base : BaseSecurityScheme::new(SecuritySchemeId::Basic),
            inLocation : SecuritySchemeInLocation::Header,
            name : None
        }
    }
}


///Digest security Scheme implementation
#[derive(Debug)]
struct BaseDigestSecurityScheme {
    base        : BaseSecurityScheme,
    inLocation  : SecuritySchemeInLocation,
    name        : Option<String>,
    qop         : SecuritySchemeDigestQOP 
}


impl JSonObject for BaseDigestSecurityScheme {
    
    fn to_json(&self) -> serde_json::Map<String, serde_json::Value> {
        let mut m  = self.base.to_json();
        self.name.copy("name".to_string(), &mut m);
        self.inLocation.copy("in".to_string(), &mut m);
        self.qop.copy("qop".to_string(),&mut m);
        m
    }
}
impl SecurityScheme for BaseDigestSecurityScheme {
    ///1
    fn get_description(&self) -> &Option<String> {
        &self.base.description
    }
    ///1
    fn set_description(&mut self, v : &Option<String>) {
        self.base.description = v.clone();
    }
    ///1
    fn get_i18n_description(&self, k: &String) -> Option<&String> {
        self.base.descriptions.get(k)
    }
    ///1
    fn set_i18n_description(&mut self, k : &String , v: &Option<String>) {
        match v {
            None => self.base.descriptions.remove(k),
            Some(x) => self.base.descriptions.insert(k.to_string(),x.to_string())
        };

    }
    ///1 
    fn get_type(&self) -> &W3CList<String> {
        &self.base.stype
    }
    ///1
    fn set_type(&mut self, v : &W3CList<String>) {
        self.base.stype = v.clone();
    }
    ///1
    fn get_id(&self) -> SecuritySchemeId {
        self.base.id.clone()
    }
    ///1
    fn get_proxy(&self) -> &Option<Url> {
        &self.base.proxy
    }
    ///1
    fn set_proxy(&mut self, v : &Option<Url>) {
        self.base.proxy = v.clone();
    }

}

impl BasicSecurityScheme for BaseDigestSecurityScheme {
    ///1
    fn get_in(&self) -> &SecuritySchemeInLocation {
        &self.inLocation
    }
    ///1
    fn set_in(&mut self, v : &SecuritySchemeInLocation) {
        &self.inLocation;
    }
    ///1
    fn get_name(&self) -> &Option<String> {
        &self.name
    }
    ///1
    fn set_name(&mut self, v: &Option<String>) {
        self.name = v.clone();
    }

}

impl DigestSecurityScheme for BaseDigestSecurityScheme {
    ///1
    fn get_qop(&self) -> SecuritySchemeDigestQOP {
        self.qop.clone()
    }
    ///1
    fn set_qop(&mut self, v : SecuritySchemeDigestQOP) {
        self.qop = v;
    }

}

impl BaseDigestSecurityScheme {
    pub fn new() -> Self {
        BaseDigestSecurityScheme {
            base : BaseSecurityScheme::new(SecuritySchemeId::Digest),
            inLocation : SecuritySchemeInLocation::Header,
            name : None,
            qop : SecuritySchemeDigestQOP::Auth,
        }
    }
}


impl SecuritySchemeFactory {
    ///1
    pub fn new(id : SecuritySchemeId) -> Box<dyn SecurityScheme> {
        match id {
            SecuritySchemeId::Nosec => Box::new(BaseSecurityScheme::new(id)),
            SecuritySchemeId::Basic => Box::new(BaseBasicSecurityScheme::new()),
            SecuritySchemeId::Digest => Box::new(BaseDigestSecurityScheme::new()),
            _ => Box::new(BaseSecurityScheme::new(id)),
        }
    }
}