use std::collections::BTreeSet;
use super::super::affordances::property_affordance::PropertyAffordance;
use super::observable_object::ObservableObject;
use std::sync::{Arc,Weak,RwLock};
use std::cell::RefCell;
use super::thing_object::ThingObject;
use std::collections::BTreeMap;

///used for base property implementation 
pub struct PropertyObject  {
    ///1
    def : Arc<Box<dyn PropertyAffordance>>,
    ///1
    val : Option<serde_json::Value>,
    ///1
    name: String ,
    ///1
    subs: BTreeSet<String>,
    ///1
    owner : RefCell<Weak<RwLock<ThingObject>>>,
    ///1
    messages : BTreeMap<String, Vec<String>>
}


impl PropertyObject {
    ///1
    pub fn new(n: &String,o: Arc<RwLock<ThingObject>>, pa : Arc<Box<dyn PropertyAffordance>>) -> Self {
        let ret = PropertyObject {
            def : pa,
            val : None,
            name : n.clone(),
            subs : BTreeSet::new(),
            owner : RefCell::new(Weak::new()),
            messages : BTreeMap::new()
        };

        *ret.owner.borrow_mut() = Arc::downgrade(&o);

        ret
    }
    ///1
    pub fn  get_value(&self) -> &Option<serde_json::Value> {
        &self.val
    }
    ///1
    pub fn set_value(&mut self, v: &Option<serde_json::Value>) {
        self.val = v.clone();

        

    }
    ///1
    pub fn get_definition(&self) -> &Box<dyn PropertyAffordance> {
        &self.def
    }
}

impl ObservableObject for PropertyObject {
    fn remove_subscriber(&mut self,ws_id: &String) {
        self.subs.remove(ws_id);
    }
    fn add_subscriber(&mut self,ws_id: &String) {
        self.subs.insert(ws_id.clone());

    }
    fn get_subscribers(&self) -> &BTreeSet<String> {
        &self.subs
    }
    fn get_subscribers_mut(&mut self) -> &mut BTreeSet<String> {
        &mut self.subs
    }

}