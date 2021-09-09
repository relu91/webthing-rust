use url::Url;
use std::fmt::Debug;
use super::json_object::JSonObject;
use super::json_object::JSonSerializer;

///1
pub trait Link: Debug + JSonObject {
    ///1
    fn get_href(&self) -> &Url;
    ///1
    fn set_href(&mut self, v : &Url);

    ///1
    fn get_type(&self) -> &Option<String>;
    ///1
    fn set_type(&mut self, v: &Option<String>);

    ///1
    fn get_rel(&self) -> &Option<String>;
    ///1
    fn set_rel(&mut self, v: &Option<String>);

    ///1
    fn get_anchor(&self) -> &Option<Url>;
    ///1
    fn set_anchor(&mut self, v : &Option<Url>);

}


#[derive(Debug,Clone)]
struct BaseLink {
    href    : Url,
    stype   : Option<String>,
    rel     : Option<String>,
    anchor  : Option<Url>
}

impl JSonObject for BaseLink {
    
    fn to_json(&self) -> serde_json::Map<std::string::String, serde_json::Value> { 
        let mut ret = serde_json::Map::new();
        self.href.copy("href".to_string(), &mut ret);
        self.anchor.copy("anchor".to_string(), &mut ret);
        self.stype.copy("type".to_string(), &mut ret);
        self.rel.copy("rel".to_string(), &mut ret);

        ret
    }
}

impl Link for BaseLink {
    fn get_href(&self) -> &Url {
        &self.href
    }
    fn set_href(&mut self, v : &Url) {
        self.href = v.clone();
    }

    fn get_type(&self) -> &Option<String> {
        &self.stype
    }
    fn set_type(&mut self, v: &Option<String>) {
        self.stype = v.clone();
    }

    fn get_rel(&self) -> &Option<String> {
        &self.rel
    }
    fn set_rel(&mut self, v: &Option<String>) {
        self.rel = v.clone();
    }

    fn get_anchor(&self) -> &Option<Url> {
        &self.anchor
    }
    fn set_anchor(&mut self, v : &Option<Url>) {
        self.anchor = v.clone();
    }

}

impl BaseLink {
    pub fn new(h : &Url) -> Self {
        Self {
            href : h.clone(),
            stype : None,
            rel : None,
            anchor : None
        }
    }
}

///1
pub struct LinkFactory {

}

impl LinkFactory {
    ///1
    pub fn new(h : &Url ) -> Box<dyn Link> {
        Box::new(
            BaseLink::new(h)
        )
    }
}