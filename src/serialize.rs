use std::result;
use std::fmt;
use serde::{de, ser};
use std_prelude::*;

use super::*;

// ##################################################
// # V2_0

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
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(V2_0Visitor)
    }
}

// ##################################################
// # ERROR CODE

impl ser::Serialize for ErrorCode {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let value = match *self {
            ErrorCode::ParseError => -32700,
            ErrorCode::InvalidRequest => -32600,
            ErrorCode::MethodNotFound => -32601,
            ErrorCode::InvalidParams => -32602,
            ErrorCode::InternalError => -32603,
            ErrorCode::ServerError(value) => value,
        };
        serializer.serialize_i64(value)
    }
}

impl de::Expected for ErrorCode {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self)
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
        Ok(ErrorCode::from(value))
    }
}

impl<'de> de::Deserialize<'de> for ErrorCode {
    fn deserialize<D>(deserializer: D) -> result::Result<ErrorCode, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_i64(ErrorCodeVisitor)
    }
}
