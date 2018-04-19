//! Crate to define the jsonrpc spec datatypes using serde -- that is it.
//!
//! This crate never touches the network, filesystem, etc.

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate std_prelude;

use std_prelude::*;
use std::result;
use std::fmt;
use serde::{de, ser};
use serde::ser::Serialize;
use serde::de::Deserialize;

pub struct V2_0;

impl ser::Serialize for V2_0 {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str("2.0")
    }
}

impl fmt::Debug for V2_0 {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("\"2.0\"")
    }
}

impl de::Expected for V2_0 {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("\"2.0\"")
    }
}

struct V2_0Visitor;

impl<'de> de::Visitor<'de> for V2_0Visitor {
    type Value = V2_0;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("exactly \"2.0\"")
    }

    fn visit_str<E>(self, value: &str) -> result::Result<V2_0, E>
    where
        E: de::Error,
    {
        if value == "2.0" {
            Ok(V2_0)
        } else {
            Err(de::Error::invalid_value(de::Unexpected::Str(value), &V2_0))
        }
    }
}

impl<'de> de::Deserialize<'de> for V2_0 {
    fn deserialize<D>(deserializer: D) -> result::Result<V2_0, D::Error>
        where D: de::Deserializer<'de>
    {
        deserializer.deserialize_str(V2_0Visitor)
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum Id {
    String(String),
    Int(u64),
    Null,
}

impl From<String> for Id {
    fn from(s: String) -> Self {
        Id::String(s)
    }
}

impl From<u64> for Id {
    fn from(v: u64) -> Self {
        Id::Int(v)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request<T> {
    pub jsonrpc: V2_0,
    pub method: String,
    pub params: Option<T>,
    pub id: Id,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Result<T> {
    pub jsonrpc: V2_0,
    pub result: T,
    pub id: Id,
}

// #[derive(Debug, Serialize, Deserialize)]
pub struct Error<T> {
    pub jsonrpc: V2_0,
    pub error: ErrorObject<T>,
    pub id: Id,
}


// #[derive(Debug, Serialize, Deserialize)]
pub struct ErrorObject<T> {
    pub code: ErrorCode,
    pub message: String,
    pub data: T,
}

#[derive(Debug)]
pub enum ErrorCode {
    /// -32700   Parse error   Invalid JSON was received by the server.
    /// An error occurred on the server while parsing the JSON text.
    ParseError,
    /// -32600   Invalid Request   The JSON sent is not a valid Request object.
    InvalidRequest,
    /// -32601   Method not found   The method does not exist / is not available.
    MethodNotFound,
    /// -32602   Invalid params   Invalid method parameter(s).
    InvalidParams,
    /// -32603   Internal error   Internal JSON-RPC error.
    InternalError,
    /// -32000 to -32099 	Server error 	Reserved for implementation-defined server-errors.
    ServerError(i64),
    /// Unknown Error Code
    Unknown(i64),
}

impl ser::Serialize for ErrorCode {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str("2.0")
    }
}

impl de::Expected for ErrorCode {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("\"2.0\"")
    }
}

struct ErrorCodeVisitor;

impl<'de> de::Visitor<'de> for ErrorCodeVisitor {
    type Value = ErrorCode;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A valid json-rpc error code")
    }

    fn visit_i64<E>(self, value: i64) -> result::Result<ErrorCode, E>
    where
        E: de::Error,
    {
        let out = match value {
            -32700 => ErrorCode::ParseError,
            -32600 => ErrorCode::InvalidRequest,
            -32601 => ErrorCode::MethodNotFound,
            -32602 => ErrorCode::InvalidParams,
            -32603 => ErrorCode::InternalError,
            -32100...-32000=> ErrorCode::ServerError(value),
            _ => ErrorCode::Unknown(value)
        };
        Ok(out)
    }
}

impl<'de> de::Deserialize<'de> for ErrorCode {
    fn deserialize<D>(deserializer: D) -> result::Result<ErrorCode, D::Error>
        where D: de::Deserializer<'de>
    {
        deserializer.deserialize_i64(ErrorCodeVisitor)
    }
}
