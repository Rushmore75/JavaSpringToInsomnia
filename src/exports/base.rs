use serde::{Serialize, ser::SerializeStruct};

use super::{resources::Resources, workspace::Workspace, request::Request, environment::Environment};




pub struct Base<'a> {
    /// 4
    __export_format: u8, // 4
    __export_date: String, // 2022-11-30T22:04:02.194Z
    __export_source: &'a str, // Arbitrary
    
    resources: Resources<'a>,
}

impl Serialize for Base<'_> {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {

            let mut state = serializer.serialize_struct("Base", 5)?;
            state.serialize_field("__export_format", &self.__export_format)?;
            state.serialize_field("__export_date", &self.__export_date)?;
            state.serialize_field("__export_source", &self.__export_source)?;
            state.serialize_field("resources", &self.resources)?;

            state.end()
    }
}


impl<'a> Base<'a> {

    /// Creates a new base to work from, also creates the base environment for you
    pub fn new(workspace: Workspace<'a>) -> Self {
        let time_since_epoch = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
    
        let resources: Resources = Resources { 
            workspace, 
            methods: Vec::new(), 
            environments: Vec::new(),
        };

        let mut this = Self {
            __export_format: 4,
            __export_date: time_since_epoch.as_millis().to_string(),
            __export_source: "Java Spring to Insomnia by Oliver Atkinson",
            resources,
        };

        // this *could* go in the Resources declaration. But I prefer it to go thru a 
        // universal method.
        this.add_environment(Environment::new_base_environment(this.get_workspace_id()));
        
        this
    }

    pub fn add_method(&mut self, method: Request<'a>) {
        self.resources.methods.push(method);
    }

    pub fn add_environment(&mut self, environment: Environment<'a>) {
        self.resources.environments.push(environment);
    }

    pub fn get_workspace_id(&self) -> &String {
        &self.resources.workspace.get_id()
    }

    pub fn get_base_environment_id(&self) -> &String {
        &self.resources.environments[0].get_id()
    }


}