use std::collections::BTreeSet;
///trait for object that can be observed
pub trait ObservableObject {
    fn add_subscriber(&mut self,ws_id: String);
    fn remove_subscriber(&mut self,ws_id: String);
    fn get_subscribers(&self) -> &BTreeSet<String>;
}