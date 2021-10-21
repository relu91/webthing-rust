///define interface for objects which can be notified
pub trait NotifiableObject {
    ///1
    fn notify_event(&mut self);
    ///1
    fn add_notification(&mut self,ws_id: &String, msg : &String);
    ///1
    fn get_notifications(&mut self,ws_id: &String) -> Option<&mut Vec<String>>;    
}