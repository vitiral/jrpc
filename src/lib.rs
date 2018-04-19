//! Crate to define the jsonrpc spec datatypes using serde -- that is it.
//!
//! This crate never touches the network, filesystem, etc.

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate std_prelude;

use std_prelude::*;
use std::fmt;
use serde::{de, ser};
use serde::ser::Serialize;
use serde::de::Deserialize;

pub struct Rpc2_0;

impl ser::Serialize for Rpc2_0 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str("2.0")
    }
}

impl fmt::Debug for Rpc2_0 {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("\"2.0\"")
    }
}

impl de::Expected for Rpc2_0 {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("\"2.0\"")
    }
}

struct Rpc2_0Visitor;

impl<'de> de::Visitor<'de> for Rpc2_0Visitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("exactly \"2.0\"")
    }

    fn visit_str<E>(self, value: &str) -> Result<String, E>
    where
        E: de::Error,
    {
        if value == "2.0" {
            Ok("".into())
        } else {
            Err(de::Error::invalid_value(de::Unexpected::Str(value), &Rpc2_0))
        }
    }
}

impl<'de> de::Deserialize<'de> for Rpc2_0 {
    fn deserialize<D>(deserializer: D) -> Result<Rpc2_0, D::Error>
        where D: de::Deserializer<'de>
    {
        let _ = deserializer.deserialize_str(Rpc2_0Visitor)?;
        Ok(Rpc2_0)
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    jsonrpc: Rpc2_0,
}
