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
    subs: RefCell<BTreeSet<String>>,
    ///1
    owner : RefCell<Weak<RwLock<ThingObject>>>,
    ///1
    messages : RefCell<BTreeMap<String, Vec<String>>>
}


impl PropertyObject {
    ///1
    pub fn new(n: &String,o: Arc<RwLock<ThingObject>>, pa : Arc<Box<dyn PropertyAffordance>>) -> Self {
        let ret = PropertyObject {
            def : pa,
            val : None,
            name : n.clone(),
            subs : RefCell::new(BTreeSet::new()),
            owner : RefCell::new(Weak::new()),
            messages : RefCell::new(BTreeMap::new())
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
    fn remove_subscriber(&self,ws_id: &String) {
        self.subs.borrow_mut().remove(ws_id);
    }
    fn add_subscriber(&self,ws_id: &String) {
        self.subs.borrow_mut().insert(ws_id.clone());

    }
    fn get_subscribers(&self) -> RefCell<BTreeSet<String>> {
        self.subs.clone()
    }


}