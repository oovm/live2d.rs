use serde::{Deserialize, Serialize};
use crate::{
    cubism_v1::moc::{MocObject, MocReader},
    L2Error,
};
use tracing::debug;
use crate::cubism_v1::moc::ObjectData;

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

impl MocObject for Vec<ParameterDefinition> {
    unsafe fn read_object(r: &MocReader) -> Result<Vec<ParameterDefinition>, L2Error>
    where
        Self: Sized,
    {
        let count = r.read_var()?;
        let mut params = Vec::with_capacity(count as usize);
        debug!("Find parameters: {}", count);
        for _ in 0..count {
            params.push(r.read()?)
        }
        Ok(params)
    }
}

impl MocObject for ParameterDefinition {
    unsafe fn read_object(r: &MocReader) -> Result<ParameterDefinition, L2Error>
    where
        Self: Sized,
    {
        let align: i32 = r.read_var()?;
        assert_eq!(align, 131, "unknown object");
        let max_value = r.read()?;
        let min_value = r.read()?;
        let default_value = r.read()?;
        let name = r.read()?;
        Ok(ParameterDefinition { id: name, min_value, max_value, default_value })
    }
}

