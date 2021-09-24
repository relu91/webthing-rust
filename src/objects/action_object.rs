use super::super::affordances::action_affordance::ActionAffordance;
use std::sync::Arc;
use super::thing_object::ThingObject;


pub struct ActionObject {
  ///1
  def : Arc<Box<dyn ActionAffordance>>,
  ///1
  name: String ,

  ///1
  owner : * mut ThingObject,
    
  ///1
  handler : Arc<Box< dyn ActionHandlerTrait>>
}

impl ActionObject {
    pub fn new(n: &String, pa : Arc<Box<dyn ActionAffordance>>, o: *mut  ThingObject, h :  Arc<Box< dyn ActionHandlerTrait>>) -> Self {
        ActionObject{
            def : pa,
            name : n.to_string(),
            owner: o,
            handler : h
        }
    }
}
///1
pub trait ActionHandlerTrait {
    fn handle(&self, a: &mut ActionObject);
}