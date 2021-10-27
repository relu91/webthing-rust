


#[cfg(test)]
mod test {
    use super::super::form::Form;
    use super::super::json_object::JSonObject;
    use super::super::form::FormOperationType;
    //use url::String;
    
     #[test]
     
     pub fn test_form() {
        let href = "/pippo/1.2.3.4".to_string() ;
        let content_type = "application/json".to_string();
        let content_coding = "pippo".to_string();
        let op = FormOperationType::ReadProperty;

        let ref mut f = Form::new(&href);

        assert_eq!(f.get_href(), &href);

        

        f.set_content_coding(&Some(content_coding.to_string()));
        f.set_operation(op);

        f.set_content_type(&Some(content_type.to_string()));
        let ref z_content_type = f.get_content_type().clone().unwrap();
        assert_eq!(z_content_type, &content_type);

        f.set_content_coding(&Some(content_coding.to_string()));
        let ref z_content_coding = f.get_content_coding().clone().unwrap();
        assert_eq!(z_content_coding, &content_coding);

        //assert_eq!(f.get_content_coding().to_string(), content_coding);
        let ref zop = f.get_operation_list();
        assert_eq!(zop.len(),1);
        assert_eq!(zop.contains(op),true);

        f.set_operation(FormOperationType::ReadProperty);


        let ss  = serde_json::json!(f.to_json()).to_string();

        println!("Output : {}",ss);

    }

}