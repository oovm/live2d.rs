use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
#[derive(Serialize, Deserialize)]
pub enum MocVersion {
    /// `moc 0x6`, initial version,
    V1_6_INTIAL,
    /// `moc 0x6`, opacity support
    V1_7_OPACITY,
    /// `moc 0x8`, texture option support,
    V1_8_TEX_OPTION,
    /// `moc 0x9`, Avatar parts support,
    V1_9_AVATAR_PARTS,
    /// `moc 0xA`, SDK 2.0,
    V1_10_SDK2_0,
    /// `moc 0xB`, SDK 2.1,
    V1_11_SDK2_1,
    /// `moc3 0x1`
    V3_1_SDK3_0,
    /// `moc3 0x2`
    V3_2_SDK3_3,
    /// `moc3 0x3`
    V3_3_SDK4_0,
    /// `moc3 0x4`
    V3_4_SDK4_2,
    /// `moc3 0x5`
    V3_5_SDK5_0,
}
