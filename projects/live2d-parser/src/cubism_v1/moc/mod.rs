mod affines;
mod deformers;
mod objects;
mod params;
mod parts;
mod pivots;
mod meshes;
mod string_id;

use self::parts::Part;
use crate::{
    cubism_v1::moc::{
        affines::Affine,
        deformers::{CurvedSurfaceDeformer, RotationDeformer},
        params::ParameterDefinition,
        pivots::{Pivot, PivotManager},
    },
    helpers::MocVersion,
    L2Error,
};
use integer_encoding::VarInt;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, ops::AddAssign, slice::SliceIndex};
use tracing::debug;
use crate::cubism_v1::moc::meshes::Mesh;

#[derive(Serialize, Deserialize)]
pub struct Moc {
    /// The version of the moc file
    pub version: MocVersion,
    /// Parameter list
    pub parameters: Vec<ParameterDefinition>,
    /// Parts list
    pub parts: Vec<Part>,
    /// Canvas width
    pub canvas_width: i32,
    /// Canvas height
    pub canvas_height: i32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum ObjectData {
    #[default]
    Null,
    DrawDataName,
    BaseDataName,
    Parameter(ParameterDefinition),
    Part(Box<Part>),
    RotationDeformer(RotationDeformer),
    CurvedSurfaceDeformer(CurvedSurfaceDeformer),
    PivotManager(PivotManager),
    Pivot(Pivot),
    Texture(Box<Mesh>),
    Affine(Affine),
    I32Array(Vec<i32>),
    F32Array(Vec<f32>),
    ObjectArray(Vec<ObjectData>),
    ObjectReference(i32),
    Unknown60,
    Unknown134,
    Unknown { type_id: u64 },
}


impl Moc {
    /// Parse moc data from a byte array
    ///
    /// ## Safety
    /// The input data must be a valid moc file
    pub unsafe fn new(data: &[u8]) -> Result<Moc, L2Error> {
        let reader = MocReader { moc: data, ptr: RefCell::new(0) };
        if reader.moc.get_unchecked(..3) == b"moc" {
            reader.advance(8);
        }
        else {
            return Err(L2Error::UnknownError {});
        }
        let version = reader.read()?;
        let parameters: ObjectData = reader.read()?;
        let parts: ObjectData = reader.read()?;
        let canvas_width = reader.read()?;
        let canvas_height = reader.read()?;
        Ok(Self {
            version,
            //
            parameters: parameters.as_parameters(),
            //
            parts: parts.as_parts(),
            canvas_width,
            canvas_height,
        })
    }
}

struct MocReader<'i> {
    moc: &'i [u8],
    ptr: RefCell<usize>,
}

trait MocObject {
    unsafe fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized;
}

impl<'i> MocReader<'i> {
    pub unsafe fn new(moc: &'i [u8]) -> Self {
        Self { moc, ptr: RefCell::new(0) }
    }
    pub unsafe fn version(&self) -> u8 {
        *self.moc.get_unchecked(3)
    }
    pub unsafe fn rest(&self) -> &[u8] {
        let offset = self.ptr.borrow();
        self.moc.get_unchecked(*offset..)
    }
    pub unsafe fn view(&self, slice: impl SliceIndex<[u8], Output = [u8]>) -> &[u8] {
        self.rest().get_unchecked(slice)
    }
    pub fn advance(&self, n: usize) {
        self.ptr.borrow_mut().add_assign(n)
    }
    // pub unsafe fn read_var(&self) -> Result<usize, L2Error> {
    //     match usize::decode_var(self.rest()) {
    //         Some((s, delta)) => {
    //             self.advance(delta);
    //             Ok(s)
    //         }
    //         None => Err(L2Error::UnknownError {}),
    //     }
    // }
    pub unsafe fn read_var(&self) -> Result<i32, L2Error> {
        let b1: u8 = self.read()?;
        if (b1 & 0b10000000) == 0 {
            return Ok(b1 as i32);
        }

        let b2: u8 = self.read()?;
        if (b2 & 0b10000000) == 0 {
            return Ok(((b1 & 0b01111111) as i32) << 7 | (b2 & 0b01111111) as i32);
        }

        let b3: u8 = self.read()?;
        if (b3 & 0b10000000) == 0 {
            return Ok(((b1 & 0b01111111) as i32) << 14 | ((b2 & 0b01111111) as i32) << 7 | (b3 as i32));
        }

        let b4: u8 = self.read()?;
        if (b4 & 0b10000000) != 0 {
            return Err(L2Error::UnknownError {});
        }

        Ok(((b1 & 0b01111111) as i32) << 21 | ((b2 & 0b01111111) as i32) << 14 | ((b3 & 0b01111111) as i32) << 7 | (b4 as i32))
    }
    #[track_caller]
    pub unsafe fn read<T: MocObject>(&self) -> Result<T, L2Error> {
        T::read_object(self)
    }
}

