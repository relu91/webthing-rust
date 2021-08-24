
use std::collections::BTreeSet;
use enumset::EnumSet;
use serde::{Serialize, Deserialize};

use super::expected_response::{ExpectedResponse};
use std::fmt;

///Enum for all avaiable form operation types
#[derive(enumset::EnumSetType, Debug,Serialize,Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FormOperationType {
    ///reads a property
    ReadProperty,
    ///writes a property
    WriteProperty, 
    ///observes a property 
    ObserveProperty, 
    ///stop observing a property
    UnobserveProperty, 
    ///executes an action
    InvokeAction, 
    ///subscribe for a certain event
    SubscribeEvent, 
    ///unsubscribe for a certain event
    UnsubscribeEvent, 
    ///reads all properties
    ReadAllProperties, 
    ///write all properties
    WriteAllProperties, 
    ///read multiple properties
    ReadMultiPleproperties,
    ///writes multiple properties
    WriteMultiPleproperties
}
impl fmt::Display for FormOperationType  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
///Base form definition
#[derive(Serialize,Debug)]
#[serde(rename_all = "camelCase")] 
pub struct Form {
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    #[serde(default)]
    security : BTreeSet<String>,
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    #[serde(default)]
    scopes : BTreeSet<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    method_name : String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    subprotocol: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    content_type : String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    content_coding : String,
    #[serde(skip_serializing_if = "EnumSet::is_empty", with = "oplist_serde")] 
    #[serde(default)]   
    op : EnumSet<FormOperationType>,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    href : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    response: Option<ExpectedResponse>,

}

impl Form {
    ///Main constructor
    pub fn new(h : String ) -> Self {
        Self {
            href : h,
            security : BTreeSet::new(),
            scopes :  BTreeSet::new(),
            op : EnumSet::new(),
            method_name : String::from(""),
            subprotocol : String::from(""),
            content_type : String::from(""),
            content_coding : String::from(""),
            response : None
        }
    }
    ///Gets list of allowd operations 
    pub fn get_operation_list(&self) -> EnumSet<FormOperationType> {
        return self.op;
    }
    ///Sets list of allowd operations
    pub fn set_operation_list(&mut self, ops: EnumSet<FormOperationType>) {
        self.op = ops;
    }
    ///Sets an operation as the only allowed operation
    pub fn set_operation(&mut self, op: FormOperationType) {
        self.op.clear();
        self.op.insert(op);
    }
    ///adds an operation
    pub fn add_operation(&mut self, op: FormOperationType) {
        self.op.insert(op);
    }
    ///removes an operation
    pub fn remove_operation(&mut self, op: FormOperationType) {
        self.op.remove(op);
    }

    ///Get form content type
    pub fn get_content_type(&self) -> String {
        return self.content_type.clone();
    }
    ///Set form content type
    pub  fn set_content_type(&mut self, v: String ) {
        self.content_type = v;
    }
    ///Get form content coding
    pub fn get_content_coding(&self) -> String {
        return self.content_coding.clone();
    }
    ///Set form content coding
    pub fn set_content_coding(&mut self, v: String ) {
        self.content_coding = v;
    }

    ///Get form subprotocol
    pub fn get_subprotocol(&self) -> String {
        return self.subprotocol.clone();
    }
    ///Set form subprotocol
    pub fn set_subprotocol(&mut self, v: String ) {
        self.subprotocol = v;
    }

    ///Get Method name
    pub fn get_method_name(&self) -> String {
        return self.method_name.clone();
    }
    ///Set method name
    pub fn set_method_name(&mut self, v: String ) {
        self.method_name = v;
    }

    ///Get form scopes
    pub fn get_scopes(&self) -> &BTreeSet<String> {
        return &self.scopes;
    }
    ///Clear all scopdes
    pub fn clear_scopes(&mut self) {
        self.scopes.clear();
    }
    ///Add a form scope
    pub fn add_scope(&mut self, v: String) {
        self.scopes.insert(v);
    }

    ///Remove a form scope
    pub fn remove_scope(&mut self, v: &str) {
        self.scopes.remove(v);
    }
    

    ///Get all security schema names
    pub fn get_securities(&self) -> &BTreeSet<String> {
        return &self.security;
    }
    ///Removes all security schema names
    pub fn clear_securities(&mut self) {
        self.security.clear();
    }
    ///Adds security schema name
    pub fn add_security(&mut self, v: String) {
        self.security.insert(v);
    }
    ///Remove security schema name
    pub fn remove_security(&mut self, v: &str) {
        self.security.remove(v);
    }

    ///Get Expected Result
    pub fn get_expected_response(&self)  -> &Option<ExpectedResponse> {
        return &self.response;
    }
    ///Sets Expected Result
    pub fn set_expected_response(&mut self, o : Option<ExpectedResponse>) {
        self.response = o;
    }

    ///Gets href
    pub fn get_href(&self) -> String {
        return self.href.clone();
    }
    ///Sets href
    pub fn set_href(&mut self, h : String) {
        self.href = h;
    }

}

mod oplist_serde {
    pub fn serialize<S>(op: &enumset::EnumSet<super::FormOperationType>, s: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        match op.len() {
            0 => {
                 return s.serialize_none();
            }
            1=> { 
                //return op.iter().next().unwrap().clone().serialize(s);
                let f : super::FormOperationType = op.iter().next().unwrap().clone();
                let x : String  = f.to_string();
                return s.serialize_str(&x.to_lowercase());
            }
            _ => {
                return s.serialize_none();
            }
        }
  
    }

    pub fn deserialize<'de, D>(deserializer: D)
        -> Result<enumset::EnumSet<super::FormOperationType>, D::Error>
        where D: serde::Deserializer<'de> {
        //let s: Option<String> = Option::deserialize(deserializer)?;
/*        
        if let Some(s) = s {
            return Ok(Some(NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)?))
        }
*/
        Ok(enumset::EnumSet::new())
    }    
}



