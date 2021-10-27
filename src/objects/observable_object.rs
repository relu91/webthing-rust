use std::collections::BTreeSet;
use std::cell::RefCell;

///trait for object that can be observed
pub trait ObservableObject {
    ///1
    fn add_subscriber(&self,ws_id: &String);
    ///1
    fn remove_subscriber(&self,ws_id: &String);
    ///1
    fn get_subscribers(&self) -> RefCell<BTreeSet<String>>;


}