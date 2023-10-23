use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct ProConfig {
    pub project: Project,
    pub profile: Profile
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Project {
    pub name: String
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Profile {
    pub cxx: Option<String>,
    pub src: Vec<String>,
    pub inc: Option<Vec<String>>
}
