use super::w3c_list::W3CList;
use url::Url;
///1
pub trait Thing {
    ///1
    fn get_context(&self) -> &W3CList<Url>;
    ///1
    fn set_context(&mut self, v : &Url);
    ///1
    fn set_context_list(&mut self, v : &W3CList<Url>);
    ///1
    fn add_context(&mut self, v: &Url);
    ///1
    fn clear_context(&mut self);
    ///1
    fn get_type(&self) -> &W3CList<String>;
    ///1
    fn set_type(&mut self, v: &String);
    ///2
    fn set_type_list(&mut self, v : &W3CList<String>);
    ///1
    fn add_type(&mut self, v : &String);
    ///1
    fn clear_type(&mut self);
    
}
