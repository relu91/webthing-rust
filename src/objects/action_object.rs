use super::super::affordances::action_affordance::ActionAffordance;
use std::sync::{Arc,Weak,RwLock};
use std::cell::RefCell;
use super::thing_object::ThingObject;


///1
pub struct ActionObject {
  ///1
  def : Arc<Box<dyn ActionAffordance>>,
  ///1
  name: String ,
///1
  handler : Arc<Box< dyn ActionHandlerTraits>>
}

impl ActionObject {
    ///1
    pub fn new(n: &String, pa : Arc<Box<dyn ActionAffordance>>, o: Arc<RwLock<ThingObject>>, h :  Arc<Box< dyn ActionHandlerTraits>>) -> Self {
        let ret = ActionObject{
            def : pa,
            name : n.to_string(),
            handler : h
        };

       

        ret
    }
    ///1
    pub fn handle(&self, to : &ThingObject) {
        let zz :&ActionObject = self;
        zz.handler.handle(&to);
    }

    ///1
    pub fn get_definition(&self) -> &Box<dyn ActionAffordance> {
        &self.def
    }

}
///1
pub trait ActionHandlerTraits {
    ///1
    fn handle(&self, a: &ThingObject );
}