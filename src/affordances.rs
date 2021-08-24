extern crate std;
///Expected response data
pub mod expected_response;
///Test of expected response
pub mod expected_response_test;
///Module for forms
pub mod form;
///Module for formts test
pub mod form_test;
///Base module for affordances and data schema
pub mod descriptive_data;
///Module for base interaction definition
pub mod interaction_affordance;
///Module for json helpers
mod w3c_list;
///Module for data schemas
pub mod data_schema;
///Module for base json
pub mod json_object;

pub use expected_response::ExpectedResponse;

/*
#[test]
pub fn doTest() {
    assert_eq!(1,2,"Error");
}
*/