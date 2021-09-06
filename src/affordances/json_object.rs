

///1
pub trait JSonObject {
    ///1
    fn to_json(&self) ->  serde_json::Map<String, serde_json::Value>;
}

pub (crate) trait JSonSerializer {
    fn copy(&self,n : String, tgt:&mut  serde_json::Map<String, serde_json::Value>);
} 

pub (crate) mod JSONSerialzerHelpers {
    use std::collections::btree_map::BTreeMap;
    use std::collections::btree_set::BTreeSet;
    use super::super::w3c_list::W3CList;
    use super::super::descriptive_data::DescriptiveData;
    use super::super::data_schema::DataSchemaId;
    use super::super::data_schema::DataSchema;
    use super::super::form::FormOperationType;
    use super::super::security_scheme::SecuritySchemeId ;
    use super::super::security_scheme::SecuritySchemeInLocation  ;
    use super::super::security_scheme::SecuritySchemeDigestQOP;
    use enumset::EnumSet;
    use url::Url;


    impl super::JSonSerializer for Option<f64> {
        fn copy(&self,n : String, tgt:&mut  serde_json::Map<String, serde_json::Value>) {
            if self.is_some() {
                let s : f64  = self.unwrap();
                tgt.insert(n, serde_json::json!(s));

            }
        }
    }

    impl super::JSonSerializer for Option<i32> {
        fn copy(&self,n : String, tgt:&mut  serde_json::Map<String, serde_json::Value>) {
            if self.is_some() {
                let s : i32  = self.unwrap();
                tgt.insert(n, serde_json::json!(s));

            }
        }
    }

    impl super::JSonSerializer for Option<Url>  {
        fn copy(&self,n : String, tgt:&mut  serde_json::Map<String, serde_json::Value>) {
            if self.is_some() {
                let u : &Url  = self.as_ref().unwrap();
                let s  = u.as_str();
                tgt.insert(n,serde_json::Value::String(s.to_string()));

            }
        }
    }

    impl super::JSonSerializer for Option<bool> {
        fn copy(&self,n : String, tgt:&mut  serde_json::Map<String, serde_json::Value>) {
            if self.is_some() {
                let s : bool = self.unwrap();
                tgt.insert(n, serde_json::Value::Bool(s));

            }
        }
    }

    impl super::JSonSerializer for Option<String> {
        fn copy(&self,n : String, tgt:&mut  serde_json::Map<String, serde_json::Value>) {
            if self.is_some() {
                let s  = self.as_ref().unwrap();
                tgt.insert(n, serde_json::Value::String(s.to_string()));

            }
            
        }
    }

    impl super::JSonSerializer for BTreeMap<String, String > {
        fn copy(&self,n : String, tgt:&mut  serde_json::Map<String, serde_json::Value>) {
            if self.is_empty() == false {
                let mut m = serde_json::Map::new();
                for (key, value) in self.into_iter() {
                    m.insert(key.clone(),  serde_json::Value::String(value.clone()));
                }

                tgt.insert(n, serde_json::Value::Object(m));
            }
        }
    } 
    impl super::JSonSerializer for BTreeSet<String> {
        fn copy(&self,n : String, tgt:&mut  serde_json::Map<String, serde_json::Value>) {
            if self.is_empty() == false {
                tgt.insert(n,  serde_json::json!(self));
            }
        }
    } 
/*
    impl super::JSonSerializer for Vec<String > {
        fn copy(&self,n : String, tgt:&mut  serde_json::Map<String, serde_json::Value>) {
            if self.is_empty() == false {
               tgt.insert(n, serde_json::json!(self));
            }
        }
    } 
*/
    impl super::JSonSerializer for String {
        fn copy(&self,n : String, tgt:&mut  serde_json::Map<String, serde_json::Value>) {
            if self.is_empty() == false {
                tgt.insert(n, serde_json::Value::String(self.clone()));
            }
        }
    }

    impl super::JSonSerializer for W3CList<String> {
        fn copy(&self,n : String, tgt:&mut  serde_json::Map<String, serde_json::Value>) {
            match self {
                W3CList::List(ref list) => {
                    tgt.insert(n, serde_json::json!(list));
                }
                W3CList::Single(ref single) => {
                    tgt.insert(n, serde_json::Value::String(single.clone()));
                }
                W3CList::None => (),
            }
    
        }
    }

    impl super::JSonSerializer for DescriptiveData {
        fn copy(&self,_ : String, tgt:&mut  serde_json::Map<String, serde_json::Value>) {
            self.stype.copy("@type".to_string(), tgt);
            self.title.copy("title".to_string(), tgt);
            self.titles.copy("titles".to_string(), tgt);
            self.description.copy("description".to_string(), tgt);
            self.descriptions.copy("descriptions".to_string(), tgt);

        }
    }

    impl super::JSonSerializer for Option<DataSchemaId> {
        fn copy(&self,n : String, tgt:&mut  serde_json::Map<String, serde_json::Value>) {
            if self.is_some() {
                let v  =  self.unwrap();
                let s  : String;

                match v {
                    DataSchemaId::DSIObject => s = "object".to_string(),
                    DataSchemaId::DSIArray => s = "array".to_string(),
                    DataSchemaId::DSIString => s = "string".to_string(),
                    DataSchemaId::DSINumber => s = "number".to_string(),
                    DataSchemaId::DSIInteger => s = "integer".to_string(),
                    DataSchemaId::DSIBoolean => s = "boolean".to_string(),
                    DataSchemaId::DSINull => s = "null".to_string(),
                }

                tgt.insert(n, serde_json::Value::String(s));
            }
        }
    }

    impl super::JSonSerializer for Vec<Box<dyn DataSchema>>{
        fn copy(&self,n : String, tgt:&mut  serde_json::Map<String, serde_json::Value>) {
            if self.is_empty() == false {
                let mut v : Vec<serde_json::Map<String, serde_json::Value>> = Vec::new();
                
                for e in self {
                    let  m  = e.to_json();
                    v.push(m);
    
                }

                tgt.insert(n,serde_json::json!(v));

            }
        }
    }



 

    impl super::JSonSerializer for EnumSet<FormOperationType> {
        fn copy(&self,n : String, tgt:&mut  serde_json::Map<String, serde_json::Value>) {
            if self.is_empty()  == false {
                let mut v : Vec<String> = Vec::new();

                for i in self.into_iter() {
                    let s : String;
                    match i {
                        FormOperationType::ReadProperty => s = "readproperty".to_string(),
                        FormOperationType::WriteProperty => s = "writeproperty".to_string(),
                        FormOperationType::ObserveProperty => s = "observeproperty".to_string(),
                        FormOperationType::UnobserveProperty => s =  "unobserveproperty".to_string(),
                        FormOperationType::InvokeAction => s = "invokeaction".to_string(),
                        FormOperationType::SubscribeEvent => s = "subscribeevent".to_string(),
                        FormOperationType::UnsubscribeEvent => s = "unsubscribeevent".to_string(),
                        FormOperationType::ReadAllProperties => s = "readallproperties".to_string(),
                        FormOperationType::WriteAllProperties => s = "writeallproperties".to_string(),
                        FormOperationType::ReadMultiPleproperties => s = "readmultipleproperties".to_string(),
                        FormOperationType::WriteMultiPleproperties => s = "writemultipleproperties".to_string(),
                    
                    }
                    
                    v.push(s);
                }

                tgt.insert(n, serde_json::json!(v));
            }
        }
    }


    impl<T> super::JSonSerializer for Vec<T> where T: super::JSonObject {
        fn copy(&self,n : String, tgt:&mut  serde_json::Map<String, serde_json::Value>) {
            if self.is_empty() == false {
                let mut v : Vec<serde_json::Map<String, serde_json::Value>> =  Vec::new();
                for i in self {
                    let m = i.to_json();
                    v.push(m);

                }

                tgt.insert(n, serde_json::json!(v));
            }
        }
    }
    impl super::JSonSerializer for Vec<Box<dyn super::JSonObject>>{
        fn copy(&self,n : String, tgt:&mut  serde_json::Map<String, serde_json::Value>) {
            if self.is_empty() == false {
                let mut v : Vec<serde_json::Map<String, serde_json::Value>> = Vec::new();
                
                for e in self {
                    let  m  = e.to_json();
                    v.push(m);
    
                }

                tgt.insert(n,serde_json::json!(v));

            }
        }
    }

    impl super::JSonSerializer for BTreeMap<String, Box<dyn DataSchema >>  {
        fn copy(&self,n : String, tgt:&mut  serde_json::Map<String, serde_json::Value>) {
            if self.is_empty() == false {
                let mut z : serde_json::Map<String, serde_json::Value> =   serde_json::Map::new();
                for (key, value) in self.into_iter() {
                    let m = value.to_json();
                    z.insert(key.clone(),serde_json::Value::Object(m));
                }

                tgt.insert(n, serde_json::json!(z));
            }
        }
    }

    impl super::JSonSerializer for Option<Box<dyn DataSchema >>  {
        fn copy(&self,n : String, tgt:&mut  serde_json::Map<String, serde_json::Value>) {
            match &*self {
                None => { return; }
                Some(x) => { 
                    let m = x.to_json();
                    tgt.insert(n, serde_json::json!(m));
                }
            }

        }
    }

    impl super::JSonSerializer for SecuritySchemeId {
        fn copy(&self,n : String, tgt:&mut  serde_json::Map<String, serde_json::Value>) {
            let s : String;
            match self {
                SecuritySchemeId::Nosec => s = "nosec".to_string(),
                SecuritySchemeId::Basic => s = "basic".to_string(),
                SecuritySchemeId::Digest => s = "digest".to_string(),
                SecuritySchemeId::Bearer => s = "bearer".to_string(),
                SecuritySchemeId::PSK => s = "psk".to_string(),
                SecuritySchemeId::OAuth2 => s = "oauth2".to_string(),
                SecuritySchemeId::ApiKey => s = "apikey".to_string(),
            
            }

            tgt.insert(n,serde_json::Value::String(s));
        }
    }
    impl super::JSonSerializer for SecuritySchemeInLocation  {
        fn copy(&self,n : String, tgt:&mut  serde_json::Map<String, serde_json::Value>) {
            let s : String;
            match self {
                SecuritySchemeInLocation::Header => s = "header".to_string(),
                SecuritySchemeInLocation::Query => s = "query".to_string(),
                SecuritySchemeInLocation::Body => s = "body".to_string(),
                SecuritySchemeInLocation::Cookie => s = "cookie".to_string(),
            
            }

            tgt.insert(n,serde_json::Value::String(s));
        }
    }
    impl super::JSonSerializer for SecuritySchemeDigestQOP{
        fn copy(&self,n : String, tgt:&mut  serde_json::Map<String, serde_json::Value>) {
            let s : String;
            match self  {
                SecuritySchemeDigestQOP::Auth => s = "auth".to_string(),
                SecuritySchemeDigestQOP::AuthInt => s = "auth-int".to_string(),

            }
            tgt.insert(n,serde_json::Value::String(s));
        }
    }

}
