use crate::{
    cubism_v1::moc::{MocObject, MocReader, ObjectData},
    L2Error,
};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, trace, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct PivotManager {
    pub items: Vec<Pivot>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pivot {
    pub id: String,
    pub count: u32,
    pub values: Vec<f32>,
}

impl MocObject for PivotManager {
    fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let o: ObjectData = reader.read()?;
        Ok(Self { items: o.as_pivots() })
    }
}

impl MocObject for Vec<Pivot> {
    fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let count = reader.read_var()?;
        let mut pivots = Vec::with_capacity(count as usize);
        for _ in 0..count {
            pivots.push(reader.read()?);
        }
        Ok(pivots)
    }
}

impl MocObject for Pivot {
    fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let id = reader.read()?;
        let count: i32 = reader.read()?;
        let values: ObjectData = reader.read()?;
        Ok(Self {
            id,
            count: count as u32,
            // 似乎总是 f32[3], 暂未发现反例
            values: values.as_f32_array(),
        })
    }
}

impl ObjectData {
    pub fn as_pivots(self) -> Vec<Pivot> {
        match self {
            ObjectData::Null => Vec::new(),
            ObjectData::ObjectArray(o) => o.into_iter().map(|x| x.as_pivots()).flatten().collect(),
            ObjectData::Pivot(v) => vec![v],
            ObjectData::PivotManager(v) => v.items,
            s => {
                warn!("ObjectData::as_pivots() called on non-pivot object {s:?}");
                vec![]
            }
        }
    }
}
