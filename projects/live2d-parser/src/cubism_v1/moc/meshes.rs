use crate::{
    cubism_v1::moc::{MocObject, MocReader, ObjectData},
    L2Error,
};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, trace, warn};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Mesh {
    pub id: String,
    pub target_id: String,
    pub average_draw_order: i32,
    pub pivot_draw_order: Vec<i32>,
    pub pivot_opacity: Vec<f32>,
    pub clip_id: Vec<String>,
    pub values: ObjectData,
    pub texture_id: i32,
    pub point_count: i32,
    pub polygon_count: i32,
    pub index_array: ObjectData,
    pub pivot_points: ObjectData,
    pub uv_maps: ObjectData,
    pub mesh_flags: i32,
    pub color_composition_type: i32,
    pub color_group_id: i32,
    pub culling: bool,
}

impl MocObject for Vec<Mesh> {
    fn read_object(reader: &MocReader) -> Result<Self, L2Error>
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
    fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let mut output = Mesh::default();
        output.id = reader.read()?;
        output.target_id = reader.read()?;
        output.values = reader.read()?;
        output.average_draw_order = reader.read()?;
        output.pivot_draw_order = reader.read()?;
        output.pivot_opacity = reader.read()?;
        if reader.version() >= 11 {
            let draw_id: String = reader.read()?;
            if draw_id.is_empty() {} else if draw_id.contains(",") {
                output.clip_id = draw_id.split(',').map(|s| s.to_string()).collect();
            } else {
                output.clip_id.push(draw_id)
            }
        }
        output.texture_id = reader.read()?;
        output.point_count = reader.read()?;
        output.polygon_count = reader.read()?;
        output.index_array = reader.read()?;
        output.pivot_points = reader.read()?;
        output.uv_maps = reader.read()?;
        if reader.version() >= 8 {
            output.mesh_flags = reader.read()?;
            if output.mesh_flags != 0 {
                if (output.mesh_flags & 1) != 0 {
                    output.color_group_id = reader.read()?;
                }
                output.color_composition_type = if (output.mesh_flags & 30) != 0 { (output.mesh_flags & 30) >> 1 } else { 0 };
                if (output.mesh_flags & 1 << 5) != 0 {
                    output.culling = false;
                }
            }
        }
        Ok(output)
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
