use serde::{Serialize, Deserialize};
use std::string::String;
///Contains data of Expected response

#[derive(Serialize,Debug)]
#[serde(rename_all = "camelCase")] 
pub struct ExpectedResponse {
    ///Expected response content type
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    content_type: String,
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
/*
impl Serialize for ExpectedResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        // Any implementation of Serialize.
    }
}

impl Deserialize for ExpectedResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        // Any implementation of Deserialize.
    }
}
*/

