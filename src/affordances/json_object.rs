

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
    use enumset::EnumSet;

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

    impl super::JSonSerializer for Vec<String > {
        fn copy(&self,n : String, tgt:&mut  serde_json::Map<String, serde_json::Value>) {
            if self.is_empty() == false {
               tgt.insert(n, serde_json::json!(self));
            }
        }
    } 

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
                    DSIObject => s = "object".to_string(),
                    DSIArray => s = "array".to_string(),
                    DSIString => s = "string".to_string(),
                    DSINumber => s = "number".to_string(),
                    DSIInteger => s = "integer".to_string(),
                    DSIBoolean => s = "boolean".to_string(),
                    DSINull => s = "null".to_string(),
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

}
