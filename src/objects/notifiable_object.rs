///define interface for objects which can be notified
pub trait NotifiableObject {
    ///1
    fn notify_event(&self, message : &String);
}