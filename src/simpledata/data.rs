use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SimplePackageData {
    pub id: i64,
    pub name: String,
    pub source: String,
    pub description: Option<String>,
    pub installation: Option<String>,
}

impl SimplePackageData {
    pub fn new(
        name: String,
        source: String,
        description: Option<String>,
        installation: Option<String>,
    ) -> Self {
        Self {
            id: 0,
            name,
            source,
            description,
            installation,
        }
    }
}

impl std::fmt::Display for SimplePackageData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}\nsource: {}\ndescription: {}\ninstallation: {}",
            self.name,
            self.source,
            match &self.description {
                Some(desc) => desc.as_str(),
                None => "None",
            },
            match &self.installation {
                Some(inst) => inst.as_str(),
                None => "None",
            }
        )
    }
}

#[derive(Debug)]
pub struct LongDisplayableSimpleDataVec<'a>(&'a Vec<SimplePackageData>);

impl<'a> From<&'a Vec<SimplePackageData>> for LongDisplayableSimpleDataVec<'a> {
    fn from(value: &'a Vec<SimplePackageData>) -> Self {
        Self(&value)
    }
}

impl<'a> std::fmt::Display for LongDisplayableSimpleDataVec<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in self.0.iter() {
            write!(f, "{}\n\n", item)?;
        }
        Ok(())
    }
}
