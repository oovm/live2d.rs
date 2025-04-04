use crate::{
    cubism_v1::moc::{MocObject, MocReader, ObjectData},
    L2Error,
};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, trace, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct Mesh {
    pub id: String,
    pub target_id: String,
    pub average_draw_order: i32,
    pub pivot_draw_order: Vec<i32>,
    pub pivot_opacity: Vec<f32>,
    pub clip_id: Vec<String>,
}

impl MocObject for Vec<Mesh> {
    unsafe fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let count = reader.read_var()?;
        let mut pivots = Vec::with_capacity(count as usize);
        debug!("Find texutre: {}", count);
        for _ in 0..count {
            pivots.push(reader.read()?);
        }
        Ok(pivots)
    }
}

impl MocObject for Mesh {
    unsafe fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let id = reader.read()?;
        let target_id = reader.read()?;

        let values: ObjectData = reader.read()?;
        let average_draw_order: i32 = reader.read()?;
        let pivot_draw_order: Vec<i32> = reader.read()?;
        let pivot_opacity: Vec<f32> = reader.read()?;
        let clip_id = if reader.version() >= 11 {
            let draw_id: String = reader.read()?;
            println!("Texture draw_id: {:?}", draw_id);
            if draw_id.is_empty() {
                vec![]
            }
            else if draw_id.contains(",") {
                let clip_ids: Vec<String> = draw_id.split(',').map(|s| s.to_string()).collect();
                println!("Texture clip_ids: {:?}", clip_ids);
                clip_ids
            }
            else {
                vec![draw_id]
            }
        }
        else {
            vec![]
        };
        let texture_id: i32 = reader.read()?;
        println!("Texture id: {:?}", texture_id);
        Ok(Self {
            id,
            target_id,
            // count: target as u32,
            average_draw_order,
            pivot_draw_order,
            pivot_opacity,
            clip_id,
        })
    }
}

impl ObjectData {
    pub fn as_texture(self) -> Vec<Mesh> {
        match self {
            ObjectData::Null => Vec::new(),
            ObjectData::ObjectArray(o) => o.into_iter().map(|x| x.as_texture()).flatten().collect(),
            // ObjectData::Pivot(v) => vec![v],
            // ObjectData::PivotManager(v) => v.items,
            s => {
                warn!("ObjectData::as_texture() called on non-pivot object {s:?}");
                vec![]
            }
        }
    }
}
