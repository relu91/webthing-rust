


#[cfg(test)]
mod test {
    use super::super::form::Form;
    use super::super::json_object::JSonObject;
    use super::super::form::FormOperationType;
    use url::Url;
    
     #[test]
     
     pub fn test_form() {
        let hhref = "/pippo/1.2.3.4" ;
        let href = &Url::parse(hhref).ok().unwrap();
        let content_type = "application/json";
        let content_coding = "pippo";
        let op = FormOperationType::ReadProperty;

        let ref mut f = Form::new( href);

        assert_eq!(f.get_href(), href);

        

        f.set_content_coding(content_coding.to_string());
        f.set_operation(op);

        f.set_content_type(content_type.to_string());
        let ref z_content_type = f.get_content_type();
        assert_eq!(z_content_type, content_type);

        f.set_content_coding(content_coding.to_string());
        let ref z_content_coding = f.get_content_coding();
        assert_eq!(z_content_coding, content_coding);

        //assert_eq!(f.get_content_coding().to_string(), content_coding);
        let ref zop = f.get_operation_list();
        assert_eq!(zop.len(),1);
        assert_eq!(zop.contains(op),true);

        f.set_operation(FormOperationType::ReadProperty);


        let ss  = serde_json::json!(f.to_json()).to_string();

        println!("Output : {}",ss);

    }

}