use std::collections::BTreeSet;
use super::super::affordances::property_affordance::PropertyAffordance;
use super::observable_object::ObservableObject;
use std::sync::Arc;

///used for base property implementation 
pub struct PropertyObject  {
    ///1
    def : Arc<Box<dyn PropertyAffordance>>,
    ///1
    val : Option<serde_json::Value>,
    ///1
    name: String ,
    ///1
    subs: BTreeSet<String>
}


impl PropertyObject {
    ///1
    pub fn new(n: &String, pa : Arc<Box<dyn PropertyAffordance>>) -> Self {
        PropertyObject {
            def : pa,
            val : None,
            name : n.clone(),
            subs : BTreeSet::new()
        }
    }
    ///1
    pub fn  get_value(&self) -> &Option<serde_json::Value> {
        &self.val
    }
    ///1
    pub fn set_value(&mut self, v: &Option<serde_json::Value>) {
        self.val = v.clone();

        
        for ws_id in self.subs.iter() {

        }

    }
    ///1
    pub fn get_definition(&self) -> &Box<dyn PropertyAffordance> {
        &self.def
    }
}

impl ObservableObject for PropertyObject {
    fn remove_subscriber(&mut self,ws_id: String) {
        self.subs.remove(&ws_id);
    }
    fn add_subscriber(&mut self,ws_id: String) {
        self.subs.insert(ws_id);

    }
    fn get_subscribers(&self) -> &BTreeSet<String> {
        &self.subs
    }

}