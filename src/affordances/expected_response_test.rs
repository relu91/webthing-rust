#[cfg(test)]
mod test {
    use super::super::*;
     #[test]
     pub fn test_expected_response() {
        let mut  v  = ExpectedResponse::new(String::from(""));
        let mut  s = v.get_content_type();
        assert_eq!(s.len(),0);
        v.set_content_type("appl/json".to_string());
        s = v.get_content_type();
        println!("Content Type : {}" , s );
        assert_eq!(s, "appl/json");
        
    }

}