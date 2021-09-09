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
///Module for actions affordances
pub mod action_affordance;
///Module for actions affordances
pub mod event_affordance;
///Module for properties affordances
pub mod property_affordance;
///Module for security scheme
pub mod security_scheme;
///1
pub mod version_info;
///1
pub mod link;
///1
pub mod thing;
////1
pub use expected_response::ExpectedResponse;

/*
#[test]
pub fn doTest() {
    assert_eq!(1,2,"Error");
}
*/