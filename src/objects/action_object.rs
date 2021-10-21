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
  owner : RefCell<Weak<RwLock<ThingObject>>>,
    
  ///1
  handler : Arc<Box< dyn ActionHandlerTrait>>
}

impl ActionObject {
    ///1
    pub fn new(n: &String, pa : Arc<Box<dyn ActionAffordance>>, o: Arc<RwLock<ThingObject>>, h :  Arc<Box< dyn ActionHandlerTrait>>) -> Self {
        let ret = ActionObject{
            def : pa,
            name : n.to_string(),
            owner: RefCell::new(Weak::new()),
            handler : h
        };

        *ret.owner.borrow_mut() = Arc::downgrade(&o);

        ret
    }
    ///1
    pub fn handle(&mut self) {
       
        self.handler.clone().handle(&mut *self);
    }

    ///1
    pub fn get_definition(&self) -> &Box<dyn ActionAffordance> {
        &self.def
    }

}
///1
pub trait ActionHandlerTrait {
    ///1
    fn handle(&self, a: *mut ActionObject);
}