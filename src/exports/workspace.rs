use serde::Serialize;
use uuid::Uuid;


#[derive(Serialize)]
/// Holds everything, think "project"
pub struct Workspace<'a> {
    _id: String,
    #[serde(rename(deserialize = "parentId", serialize="parentId"))]
    parent_id: Option<&'a str>, // no parent, it is the parent
    modified: u64, // millis since epoch
    created: u64,
    name: &'a str,
    description: &'a str,
    /// "collection"
    scope: &'a str,
    /// "workspace"
    _type: &'a str,
}

impl<'a> Workspace<'a> {

    /// Workspace's id is the parent id for methods
    pub fn new(name: &'a str, description: &'a str) -> Self {

        let time_since_epoch = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
        let time = time_since_epoch.as_millis() as u64;
        let id = Uuid::new_v4();


        Self {
            _id: id.to_string(),
            parent_id: None,
            modified: time,
            created: time,
            name,
            description,
            scope: "collection",
            _type: "workspace"
        }
    }

    pub fn get_id(&self) -> &String {
        return &self._id
    }
}
