use std::collections::BTreeSet;
use super::super::affordances::property_affordance::PropertyAffordance;
use super::observable_object::ObservableObject;
use super::notifiable_object::NotifiableObject;
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
    _name: String ,
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
            _name : n.clone(),
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

        self.notify_event();

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

impl NotifiableObject for PropertyObject {
    ///1
    fn notify_event(&self) {
        
        let jmsg = self.get_value().clone().unwrap();
        let msg = jmsg.to_string();

        let so = self.subs.borrow_mut();

        for ws_id  in so.iter() {
            let s_ws_id = ws_id.clone();
            let mut s_this_msgs = self.messages.borrow_mut();

            let v : &mut Vec<String> = match s_this_msgs.get_mut(&s_ws_id) {
                Some(x) => x,
                None => return
            };
    
            v.push(msg.clone());
    
        }
    }

    ///1
    fn add_notification(&self,ws_id: &String, msg : &String) {
        let s_ws_id = ws_id.clone();
        let mut s_this_msgs = self.messages.borrow_mut();

        let v : &mut Vec<String> = match s_this_msgs.get_mut(&s_ws_id) {
            Some(x) => x,
            None => return
        };

        v.push(msg.clone());
}
    ///1
    fn get_notifications(&mut self,ws_id: &String, tgt : &mut Vec<String>) {

        let mmap = &mut self.messages.get_mut();

        let data = mmap.get_mut(ws_id);

        if data.is_some() {
            let v = data.unwrap();
            tgt.append(v);
        }

        
    }
}
