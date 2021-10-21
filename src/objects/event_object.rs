
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
  name: String ,
  ///1
  subs: BTreeSet<String>,
  ///1
  owner : RefCell<Weak<RwLock<ThingObject>>>,
  ///1
  handler : Arc<Box<dyn EventHandlerTraits >>,
  ///1
  messages : BTreeMap<String, Vec<String>>
}

///1
pub trait EventHandlerTraits {
    ///1
    fn make_event_data(&self) -> serde_json::Value ;
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
            name : n.to_string(),
            subs : BTreeSet::new(),
            owner: RefCell::new(Weak::new()),
            handler : handl,
            messages : BTreeMap::new()
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
    fn notify_event(&mut self) {
        let zz :&mut EventObject = self;
        let jmsg = zz.handler.make_event_data();
        let msg = jmsg.to_string();

        let subs : &BTreeSet<String> = &mut zz.subs;

        for ws_id  in subs {
            let v : &mut Vec<String> = match zz.messages.get_mut(ws_id) {
                Some(x) => x,
                None => return
            };
    
            v.push(msg.clone());
    
        }
    }

    ///1
    fn add_notification(&mut self,ws_id: &String, msg : &String) {
        let mut v : &mut Vec<String> = match self.messages.get_mut(ws_id) {
            Some(x) => x,
            None => return
        };

        v.push(msg.clone());
    }
    ///1
    fn get_notifications(&mut self,ws_id: &String) ->  Option<&mut Vec<String>> {
        self.messages.get_mut(ws_id)

    }
}

impl ObservableObject for EventObject {
    fn remove_subscriber(&mut self,ws_id: &String) {
        self.subs.remove(ws_id);
        self.messages.remove(ws_id);
        
    }
    fn add_subscriber(&mut self,ws_id: &String) {
        self.subs.insert(ws_id.clone());
        self.messages.remove(ws_id);
        self.messages.insert(ws_id.clone(),Vec::new());

    }
    fn get_subscribers(&self) -> &BTreeSet<String>{
        &self.subs
    }

    fn get_subscribers_mut(&mut self) -> &mut BTreeSet<String> {
        &mut self.subs
    }
}