///define interface for objects which can be notified
pub trait NotifiableObject {
    ///1
    fn notify_event(&self);
    ///1
    fn add_notification(&self,ws_id: &String, msg : &String);
    ///1
    fn get_notifications(&mut self,ws_id: &String,tgt : &mut Vec<String>) ;
}