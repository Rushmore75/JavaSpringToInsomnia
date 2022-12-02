use std::collections::HashMap;

use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
/// Environments hold variables, which can then be swapped with a quick change of environment.
/// This would be used to change base url easily
pub struct Environment<'a> {
    _id: String,
    #[serde(rename(deserialize = "parentId", serialize="parentId"))]
    parent_id: String,
    modified: u64,
    created: u64,
    name: &'a str,
    data: HashMap<&'a str, &'a str>,
    #[serde(rename(deserialize = "dataPropertyOrder", serialize="dataPropertyOrder"))]
    data_property_order: DataPropertyOrder<'a>,
    color: Option<&'a str>,
    #[serde(rename(deserialize = "isPrivate", serialize="isPrivate"))]
    is_private: bool,
    #[serde(rename(deserialize = "metaSortKey", serialize="metaSortKey"))]
    meta_sort_key: i32,
    /// "environment"
    _type: &'a str, 
}

impl<'a> Environment<'a> {

    /// Environment's ids seem to be decoupled from everything else
    fn new(parent_id: &String, name: &'a str) -> Self {
        let time_since_epoch = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
        let time = time_since_epoch.as_millis() as u64;
        let id = Uuid::new_v4();


        Self {
            _id: id.to_string(),
            parent_id: parent_id.to_string(),
            modified: time,
            created: time,
            name,
            data: HashMap::new(),
            data_property_order: DataPropertyOrder { name: Vec::new() },
            color: None,
            is_private: false,
            meta_sort_key: 0, 
            _type: "environment"
        } 
    }

    pub (super) fn new_base_environment(workspace_id: &String) -> Self {
        // base is allowed to see this but outside this folder it's private
        Environment::new(workspace_id, "Base Environment")        
    }

    pub fn new_sub_environment(parent_id: &String, name: &'a str) -> Self {
        Environment::new(parent_id, name)
    }

    /// This data will be accessible for use with {{ key }}
    pub fn add_environment_data(&mut self, key: &'a str, value: &'a str) {
        self.data.insert(key, value);
        self.data_property_order.name.push(key);
    }

    pub fn get_id(&self) -> &String {
        &self._id
    }

}

#[derive(Serialize)]
struct DataPropertyOrder<'a> {
    #[serde(rename(deserialize = "&", serialize="&"))]
    name: Vec<&'a str>
}