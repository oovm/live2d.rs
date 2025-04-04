use super::*;
use crate::cubism_v1::moc::ObjectData::ObjectReference;
use tracing::{error, trace, warn};

impl MocObject for Vec<ObjectData> {
    #[track_caller]
    fn read_object(r: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let count = r.read_var()?;
        let mut objects = Vec::with_capacity(count as usize);
        trace!("Find objects: {}", count);
        for _ in 0..count {
            objects.push(r.read()?);
        }
        Ok(objects)
    }
}

impl MocObject for ObjectData {
    #[track_caller]
    fn read_object(r: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let caller = std::panic::Location::caller();
        let type_id = r.read_var()?;
        // trace!("preview: {type_id}@{:?}\n    {:?}", r.view(..8), caller);
        let data = match type_id {
            0 => ObjectData::Null,
            15 => ObjectData::ObjectArray(r.read()?),
            25 => ObjectData::I32Array(r.read()?),
            27 => ObjectData::F32Array(r.read()?),
            33 => {
                let object_id: i32 = r.read()?;
                error!("ObjectData::read_object() called on non-pivot object {object_id}");
                return Ok(ObjectReference(object_id));
            }
            65 => ObjectData::CurvedSurfaceDeformer(r.read()?),
            66 => ObjectData::PivotManager(r.read()?),
            67 => ObjectData::Pivot(r.read()?),
            68 => ObjectData::RotationDeformer(r.read()?),
            69 => ObjectData::Affine(r.read()?),
            70 => ObjectData::Texture(Box::new(r.read()?)),
            131 => ObjectData::Parameter(r.read()?),
            133 => ObjectData::Part(Box::new(r.read()?)),
            // _ => Err(L2Error::UnknownType { type_id: type_id as u32 })?,
            _ => panic!("unknown type: {type_id}"),
        };
        Ok(data)
    }
}

impl<const N: usize> MocObject for [u8; N] {
    fn read_object(r: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        if r.rest().len() < N {
            return Err(L2Error::OutOfBounds { rest: r.rest().len(), request: N });
        }
        let array = unsafe {
            std::ptr::read(r.rest().as_ptr() as *const [u8; N])
        };
        r.advance(N);
        Ok(array)
    }
}

impl MocObject for Vec<i32> {
    fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let count = reader.read_var()?;
        let mut values = Vec::with_capacity(count as usize);
        for _ in 0..count {
            values.push(reader.read()?);
        }
        Ok(values)
    }
}

impl MocObject for i32 {
    fn read_object(r: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        Ok(i32::from_be_bytes(r.read()?))
    }
}

impl MocObject for Vec<f32> {
    fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let count = reader.read_var()?;
        let mut values = Vec::with_capacity(count as usize);
        for _ in 0..count {
            values.push(reader.read()?);
        }
        Ok(values)
    }
}

impl MocObject for f32 {
    fn read_object(r: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        Ok(f32::from_be_bytes(r.read()?))
    }
}

impl MocObject for u8 {
    fn read_object(r: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let float = unsafe {
            std::ptr::read(r.rest().as_ptr())
        };
        r.advance(1);
        Ok(float)
    }
}
impl MocObject for bool {
    fn read_object(r: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        Ok(u8::read_object(r)? != 0)
    }
}

impl ObjectData {
    pub fn as_f32_array(self) -> Vec<f32> {
        match self {
            ObjectData::Null => Vec::new(),
            // ObjectData::ObjectArray(o) => o.into_iter().map(|x| x.as_f32_array()).flatten().collect(),
            ObjectData::F32Array(v) => v,
            s => {
                warn!("ObjectData::as_f32_array() called on non-pivot object {s:?}");
                vec![]
            }
        }
    }
}
impl MocObject for MocVersion {
    fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let v = match reader.version() {
            6 => MocVersion::V1_6_INTIAL,
            7 => MocVersion::V1_7_OPACITY,
            8 => MocVersion::V1_8_TEX_OPTION,
            9 => MocVersion::V1_9_AVATAR_PARTS,
            10 => MocVersion::V1_10_SDK2_0,
            11 => MocVersion::V1_11_SDK2_1,
            _ => Err(L2Error::UnknownError {})?,
        };
        Ok(v)
    }
}
