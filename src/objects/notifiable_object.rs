///define interface for objects which can be notified
pub trait NotifiableObject {
    fn notify_event(&self, message : &String);
}