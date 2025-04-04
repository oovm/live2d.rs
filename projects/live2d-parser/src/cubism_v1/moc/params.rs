use crate::{
    cubism_v1::moc::{parts::Part, MocObject, MocReader, ObjectData},
    L2Error,
};
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

#[derive(Debug)]
pub struct ParameterList {
    items: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterDefinition {
    /// Parameter name
    pub id: String,
    /// Minimum value
    pub min_value: f32,
    /// Maximum value
    pub max_value: f32,
    /// Default value
    pub default_value: f32,
}

impl MocObject for ParameterDefinition {
    fn read_object(r: &MocReader) -> Result<ParameterDefinition, L2Error>
    where
        Self: Sized,
    {
        let max_value = r.read()?;
        let min_value = r.read()?;
        let default_value = r.read()?;
        let name = r.read()?;
        Ok(ParameterDefinition { id: name, min_value, max_value, default_value })
    }
}

impl ObjectData {
    pub fn as_parameters(self) -> Vec<ParameterDefinition> {
        match self {
            ObjectData::Parameter(p) => vec![p],
            ObjectData::ObjectArray(v) => v.into_iter().map(|o| o.as_parameters()).flatten().collect(),
            s => {
                warn!("ObjectData::as_parameters() called on non-pivot object {s:?}");
                vec![]
            }
        }
    }
}
