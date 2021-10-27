
use super::json_object::JSonObject;
use std::fmt::Debug;
///1
pub trait VersionInfo : Debug + JSonObject  {
    ///1
    fn get_instance(&self) -> &String;
    ///1
    fn set_instance(&mut self, v: &String);
}

#[derive(Debug,Clone)]
struct BaseVersionInfo {
    instance : String,
}

impl BaseVersionInfo {
    pub fn new(inst : &String) -> Self {
        Self { 
            instance : inst.clone()
        }
    }
}


impl JSonObject for BaseVersionInfo {

    fn to_json(&self) -> serde_json::Map<std::string::String, serde_json::Value> { 
        let mut ret = serde_json::Map::new();
        ret.insert("instance".to_string(), serde_json::Value::String(self.instance.clone()));

        ret
    }
}

impl VersionInfo for BaseVersionInfo {
    fn get_instance(&self) -> &String {
        &self.instance
    }
    fn set_instance(&mut self, v: &String) {
        self.instance = v.clone();
    }

}
///1
pub struct VersionInfoFactory {

}

impl VersionInfoFactory {
    ///1
    pub fn new(inst : &String) -> Box<dyn VersionInfo> {
        Box::new(
            BaseVersionInfo::new(inst)
        )
    }
}