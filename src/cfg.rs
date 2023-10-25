use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct ProConfig {
    pub package: Package,
    pub profile: Profile
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Package {
    pub name: String
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Profile {
    pub cxx: Option<String>,
    pub src: Vec<String>,
    pub inc: Option<Vec<String>>
}
