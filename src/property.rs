use serde_json;
use std::marker::{Send, Sync};
use super::affordances::property_affordance::PropertyAffordance;



/// Used to forward a new property value to the physical/virtual device.
pub trait ValueForwarder: Send + Sync {
    /// Set the new value of the property.
    fn set_value(&mut self, value: serde_json::Value) -> Result<serde_json::Value, &'static str>;
}

/// High-level Property trait.
pub trait Property: Send + Sync {
    /// Validate new property value before setting it.
    ///
    /// Returns a result indicating validity.
    fn validate_value(&self, value: &serde_json::Value) -> Result<(), &'static str> {
        let mut affordance = self.get_affordance();

        match affordance.get_readonly() {
            None  => (),
            Some(x) => {
                if x {
                    return Err("Read-only property");
                }
            }
        }
        Ok(())

    }

    /// Get the property description.
    ///
    /// Returns a JSON value describing the property.
    /*/// 
    fn as_property_description(&self) -> serde_json::Map<String, serde_json::Value> {
        let mut description = self.get_metadata().clone();
        let link = json!(
            {
                "rel": "property",
                "href": self.get_href(),
            }
        );

        if description.contains_key("links") {
            let links = description
                .get_mut("links")
                .unwrap()
                .as_array_mut()
                .unwrap();
            links.push(link);
        } else {
            description.insert("links".to_string(), json!([link]));
        }
        description
    }
*/

    /// Get the href of this property.
    ///fn get_href(&self) -> String;

    /// Get the current property value.
    fn get_value(&self) -> serde_json::Value;

    /// Set the current value of the property with the value forwarder.
    fn set_value(&mut self, value: serde_json::Value) -> Result<(), &'static str>;

    /// Set the cached value of the property.
    fn set_cached_value(&mut self, value: serde_json::Value) -> Result<(), &'static str>;

    /// Get the name of this property.
    fn get_name(&self) -> String;

    // Get the metadata associated with this property.
    //fn get_metadata(&self) -> serde_json::Map<String, serde_json::Value>;

    fn get_affordance(&self) -> &Box<dyn PropertyAffordance>;
}

/// Basic property implementation.
///
/// A Property represents an individual state value of a thing.
///
/// This can easily be used by other properties to handle most of the boring work.
pub struct BaseProperty {
    name: String,
    last_value: serde_json::Value,
    value_forwarder: Option<Box<dyn ValueForwarder>>,
    data           : Box<dyn PropertyAffordance>
}

impl BaseProperty {
    /// Create a new BaseProperty.
    ///
    /// # Arguments
    ///
    /// * `name` - name of the property
    /// * `initial_value` - initial property value
    /// * `value_forwarder` - optional value forwarder; property will be read-only if None
    /// * `metadata` - property metadata, i.e. type, description, unit, etc., as a JSON map
    pub fn new(
        name: String,
        initial_value: serde_json::Value,
        value_forwarder: Option<Box<dyn ValueForwarder>>,
        data : Box<dyn PropertyAffordance>
    ) -> BaseProperty {

        

        BaseProperty {
            name: name,
            last_value: initial_value,
            value_forwarder: value_forwarder,
            data : data
        }
    }
}

impl Property for BaseProperty {

    /// Get the current property value.
    fn get_value(&self) -> serde_json::Value {
        self.last_value.clone()
    }

    /// Set the current value of the property.
    fn set_value(&mut self, value: serde_json::Value) -> Result<(), &'static str> {
        let result = self.validate_value(&value);
        if result.is_err() {
            return result;
        }

        match self.value_forwarder {
            Some(ref mut vf) => match vf.set_value(value) {
                Ok(v) => {
                    self.last_value = v;
                    Ok(())
                }
                Err(e) => Err(e),
            },
            None => {
                self.last_value = value;
                Ok(())
            }
        }
    }

    /// Set the cached value of the property.
    fn set_cached_value(&mut self, value: serde_json::Value) -> Result<(), &'static str> {
        self.last_value = value;
        Ok(())
    }

    /// Get the name of this property.
    fn get_name(&self) -> String {
        self.name.clone()

    }

    fn get_affordance(&self) -> &Box<dyn PropertyAffordance> {
        &self.data
    }
}
