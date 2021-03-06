extern crate serde_json;
#[macro_use] extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedObj {
    pub timestamp: Option<String>,
    pub message: String,
}
