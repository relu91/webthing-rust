use std::collections::BTreeSet;
use super::super::affordances::property_affordance::PropertyAffordance;
use super::observable_object::ObservableObject;

///used for base property implementation 


pub struct PropertyObject  {
    def : Box<dyn PropertyAffordance>,
    val : Option<serde_json::Value>,
    name: String ,
    subs: BTreeSet<String>
}


impl PropertyObject {
    pub fn new(n: &String, pa : Box<dyn PropertyAffordance>) -> Self {
        PropertyObject {
            def : pa,
            val : None,
            name : n.clone(),
            subs : BTreeSet::new()
        }
    }

    pub fn  get_value(&self) -> &Option<serde_json::Value> {
        &self.val
    }
    pub fn set_value(&mut self, v: &Option<serde_json::Value>) {
        self.val = v.clone();

        
        for ws_id in self.subs.iter() {

        }

    }

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