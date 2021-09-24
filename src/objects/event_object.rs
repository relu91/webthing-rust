
use super::notifiable_object::NotifiableObject;
use super::observable_object::ObservableObject;
use super::super::affordances::event_affordance::EventAffordance;
use std::collections::BTreeSet;
use std::sync::Arc;
use super::thing_object::ThingObject;



///1

pub struct EventObject {
  ///1
  def : Arc<Box<dyn EventAffordance>>,
  ///1
  name: String ,
  ///1
  subs: BTreeSet<String>,
  ///1
  owner : * mut ThingObject,
}

impl EventObject {
    ///1
    pub fn new(n: &String, pa : Arc<Box<dyn EventAffordance>>, o: *mut  ThingObject) -> Self {
        EventObject{
            def : pa,
            name : n.to_string(),
            subs : BTreeSet::new(),
            owner: o
        }
    }
}

impl NotifiableObject for EventObject {
    fn notify_event(&self, message : &String) {

    }
}

impl ObservableObject for EventObject {
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