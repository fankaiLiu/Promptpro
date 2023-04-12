use std::collections::HashMap;


#[derive(Debug, serde::Deserialize)]
pub struct Configs {
    pub must_appear:Vec<String>,
    pub may_appear:HashMap<String,u32>,
    pub pick_one:Vec<HashMap<String,u32>>,
}




