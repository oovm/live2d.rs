use super::*;


impl MocObject for String {
    #[track_caller]
    unsafe fn read_object(r: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let caller = std::panic::Location::caller();
        // #[cfg(test)]
        let _ = match r.read_var()? {
            0 => return Ok(String::new()),
            50 => ObjectData::DrawDataName,
            51 => ObjectData::BaseDataName,
            60 => ObjectData::Unknown60,
            134 => ObjectData::Unknown134,
            s => panic!("unknown string type: {s}\n    {caller:?}"),
        };
        let length = r.read_var()? as usize;
        // tracing::trace!("String Length: {length}");
        let str = String::from_utf8_lossy(r.view(..length));
        // warn!("String: {str}\n    {caller:?}");
        r.advance(length);
        Ok(str.to_string())
    }
}