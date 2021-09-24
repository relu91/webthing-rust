
use super::notifiable_object::NotifiableObject;
use super::observable_object::ObservableObject;
use super::super::affordances::event_affordance::EventAffordance;
use std::collections::BTreeSet;
use std::sync::Arc;

///1

pub struct EventObject {
  def : Arc<Box<dyn EventAffordance>>,
  name: String ,
  subs: BTreeSet<String>

}

impl EventObject {
    ///1
    pub fn new(n: &String, pa : Arc<Box<dyn EventAffordance>>) -> Self {
        EventObject {
            def : pa,
            name : n.to_string(),
            subs : BTreeSet::new()
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