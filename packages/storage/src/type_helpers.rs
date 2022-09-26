use serde::de::DeserializeOwned;
use std::any::type_name;
#[cfg(feature = "iterator")]
use cosmwasm_std::Record;
use cosmwasm_std::{from_slice, StdError, StdResult};
/// may_deserialize parses json bytes from storage (Option), returning Ok(None) if no data present
///
/// value is an odd type, but this is meant to be easy to use with output from storage.get (Option<Vec<u8>>)
/// and value.map(|s| s.as_slice()) seems trickier than &value
pub(crate) fn may_deserialize<T: DeserializeOwned>(
    value: &Option<Vec<u8>>,
) -> StdResult<Option<T>> {
    match value {
        Some(data) => Ok(Some(from_slice(data)?)),
        None => Ok(None),
    }
}
/// must_deserialize parses json bytes from storage (Option), returning NotFound error if no data present
pub(crate) fn must_deserialize<T: DeserializeOwned>(value: &Option<Vec<u8>>) -> StdResult<T> {
    match value {
        Some(data) => from_slice(data),
        None => Err(StdError::not_found(type_name::<T>())),
    }
}
#[cfg(feature = "iterator")]
pub(crate) fn deserialize_kv<T: DeserializeOwned>(kv: Record<Vec<u8>>) -> StdResult<Record<T>> {
    let (k, v) = kv;
    let t = from_slice::<T>(&v)?;
    Ok((k, t))
}
