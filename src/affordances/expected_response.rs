use super::json_object::JSonObject;
use super::json_object::JSonSerializer;
use std::string::String;
///Contains data of Expected response

#[derive(Debug,Clone)]
pub struct ExpectedResponse {
    ///Expected response content type
    content_type: String,
}
impl JSonObject for ExpectedResponse {
    fn to_json(&self) ->  serde_json::Map<String, serde_json::Value> {
        let mut m  = serde_json::Map::new();
        self.content_type.copy("contentType".to_string(),&mut m);
        m
    }    
}
impl ExpectedResponse {
    ///Getresponse content type
    pub fn get_content_type(&self) -> String {
         return self.content_type.clone();
   }
   ///Set response content type
   pub fn set_content_type(&mut self, v: String) {
        self.content_type = v;
   }

   ///constructor
   pub fn new(v : String ) -> ExpectedResponse {
     return ExpectedResponse{content_type : v};
   }

}

