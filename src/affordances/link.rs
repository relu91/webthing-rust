//use url::String;
use std::fmt::Debug;
use super::json_object::JSonObject;
use super::json_object::JSonSerializer;
use std::cmp::Ord;
use std::cmp::PartialOrd;
use std::cmp::PartialEq;
use std::cmp::Ordering;



///1
#[derive(Debug,Clone)]
pub struct Link {
    ///1
    pub href    : String,
    ///1
    pub stype   : Option<String>,
    ///1
    pub rel     : Option<String>,
    ///1
    pub anchor  : Option<String>
}

impl Ord for Link {
    fn cmp(&self, other: &Self) -> Ordering {
        let s1  = &self.href;
        let s2 = &other.href;
        s1.cmp(&s2)
    }

}
impl PartialOrd for Link {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let s1  = &self.href;
        let s2 = &other.href;
        Some(s1.cmp(&s2))

    }

}
impl PartialEq for Link {
    fn eq(&self, other: &Self) -> bool {
        let s1  = &self.href;
        let s2 = &other.href;
        s1 == s2
    }

}
impl Eq for Link {

}

impl JSonObject for Link {
    
    fn to_json(&self) -> serde_json::Map<std::string::String, serde_json::Value> { 
        let mut ret = serde_json::Map::new();
        self.href.copy("href".to_string(), &mut ret);
        self.anchor.copy("anchor".to_string(), &mut ret);
        self.stype.copy("type".to_string(), &mut ret);
        self.rel.copy("rel".to_string(), &mut ret);

        ret
    }
}




impl Link {
    ///1
    pub fn new(h : &String) -> Self {
        Self {
            href : h.clone(),
            stype : None,
            rel : None,
            anchor : None
        }
    }
}

