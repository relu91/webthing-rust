use std::collections::BTreeSet;
///trait for object that can be observed
pub trait ObservableObject {
    ///1
    fn add_subscriber(&mut self,ws_id: &String);
    ///1
    fn remove_subscriber(&mut self,ws_id: &String);
    ///1
    fn get_subscribers(&self) -> &BTreeSet<String>;

    ///1
    fn get_subscribers_mut(&mut self) -> &mut BTreeSet<String>;

}