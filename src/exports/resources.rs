use serde::{Serialize, ser::SerializeSeq};

use super::{workspace::Workspace, request::Request, environment::Environment};

pub struct Resources<'a> {
    pub workspace: Workspace<'a>,
    pub methods: Vec<Request<'a>>,
    pub environments: Vec<Environment<'a>>
}


impl Serialize for Resources<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {

            let size = self.environments.len() + self.methods.len();
            let mut resources = serializer.serialize_seq(Some(size))?;
            
            self.environments.iter().for_each(|f| { resources.serialize_element(f).unwrap(); });
            self.methods.iter().for_each(|f| { resources.serialize_element(f).unwrap(); });
            resources.serialize_element(&self.workspace)?;


            resources.end()
    }
}
