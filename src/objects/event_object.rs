
use super::notifiable_object::NotifiableObject;
use super::observable_object::ObservableObject;
use super::super::affordances::event_affordance::EventAffordance;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use super::thing_object::ThingObject;
use std::sync::{Arc,Weak,RwLock};
use std::cell::RefCell;


///1
pub struct EventObject {
  ///1
  def : Arc<Box<dyn EventAffordance>>,
  ///1
  _name: String ,
  ///1
  subs: RefCell<BTreeSet<String>>,
  ///1
  owner : RefCell<Weak<RwLock<ThingObject>>>,
  ///1
  handler : Arc<Box<dyn EventHandlerTraits >>,
  ///1
  messages : RefCell<BTreeMap<String, Vec<String>>>
}

///1
pub trait EventHandlerTraits {
    ///1
    fn make_event_data(&self, owner:  RefCell<Weak<RwLock<ThingObject>>>) -> serde_json::Value ;
}

impl EventObject {
    ///1
    pub fn new( 
        n       :   &String, 
        pa      :   Arc<Box<dyn EventAffordance>>, 
        handl   :   Arc<Box<dyn EventHandlerTraits >>,
        o       :   Arc<RwLock<ThingObject>>
    ) -> Self {
        let ret = EventObject{
            def : pa,
            _name : n.to_string(),
            subs : RefCell::new(BTreeSet::new()),
            owner: RefCell::new(Weak::new()),
            handler : handl,
            messages : RefCell::new(BTreeMap::new())
        };

        *ret.owner.borrow_mut() = Arc::downgrade(&o);
        ret
    }
    ///1
    pub fn get_definition(&self) -> &Box<dyn EventAffordance> {
        &self.def
    }

}

impl NotifiableObject for EventObject {
    ///1
    fn notify_event(&self) {
        
        let jmsg = self.handler.make_event_data(self.owner.clone());
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

impl ObservableObject for EventObject {
    fn remove_subscriber(&self,ws_id: &String) {
        self.subs.borrow_mut().remove(ws_id);
        self.messages.borrow_mut().remove(ws_id);
        
    }
    fn add_subscriber(&self,ws_id: &String) {
        self.subs.borrow_mut().insert(ws_id.clone());
        self.messages.borrow_mut().remove(ws_id);
        self.messages.borrow_mut().insert(ws_id.clone(),Vec::new());

    }
    fn get_subscribers(&self) -> RefCell<BTreeSet<String>>{
        self.subs.clone()
    }


}