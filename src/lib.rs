//! # jrpc: an ultra lightweight crate for types from the jsonrpc spec
//!
//! This crate defines the datatypes for the jsonrpc spec... and that is IT.
//!
//! This crate never touches the network, filesystem, etc. It simply uses serde
//! to easily construct, serialize and deserialize Request, Result and Error
//! data types.
//!
//! http://www.jsonrpc.org/specification_v2

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate std_prelude;

mod serialize;

use std_prelude::*;
use serde::ser::Serialize;
use serde::de::{Deserialize, DeserializeOwned};

/// The `jsonrpc` version. Will serialize/deserialize to/from `"2.0"`.
pub struct V2_0;

/// An identifier established by the Client that MUST contain a String, Number, or NULL value if
/// included. If it is not included it is assumed to be a notification. The value SHOULD normally
/// not be Null and Numbers SHOULD NOT contain fractional parts
///
/// The Server MUST reply with the same value in the Response object if included. This member is
/// used to correlate the context between the two objects.
///
/// ## TODO: Notification
///
/// A Notification is a Request object without an "id" member. A Request object that is a
/// Notification signifies the Client's lack of interest in the corresponding Response object, and
/// as such no Response object needs to be returned to the client. The Server MUST NOT reply to a
/// Notification, including those that are within a batch request.
///
/// Notifications are not confirmable by definition, since they do not have a Response object to be
/// returned. As such, the Client would not be aware of any errors (like e.g. "Invalid
/// params","Internal error").
///
/// # Examples
///
/// ```rust
/// # extern crate jrpc;
/// extern crate serde_json;
/// use jrpc::Id;
///
/// # fn main() {
/// assert_eq!(Id::from(4), Id::Int(4));
/// assert_eq!(
///     serde_json::from_str::<Id>("4").unwrap(),
///     Id::Int(4),
/// );
/// assert_eq!(
///     serde_json::from_str::<Id>("\"foo\"").unwrap(),
///     Id::String("foo".into()),
/// );
/// assert_eq!(
///     serde_json::from_str::<Id>("null").unwrap(),
///     Id::Null,
/// );
/// # }
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Id {
    String(String),
    Int(i64),
    Null,
    // FIXME: handle Notification type. If id doesn't exist then it is notification.
}

impl From<String> for Id {
    fn from(s: String) -> Self {
        Id::String(s)
    }
}

impl From<i64> for Id {
    fn from(v: i64) -> Self {
        Id::Int(v)
    }
}

/// The jsonrpc Request object.
///
/// # Examples
///
/// ```rust
/// # extern crate jrpc;
/// extern crate serde_json;
/// use jrpc::{Id, Request, V2_0};
///
/// # fn main() {
/// let value: Vec<u32> = vec![1, 2, 3];
/// let request = Request::with_params(
///     Id::from(4),
///     "CreateFoo".into(),
///     Some(value.clone()),
/// );
/// let json = r#"
/// {
///     "jsonrpc": "2.0",
///     "method": "CreateFoo",
///     "params": [1,2,3],
///     "id": 4
/// }
/// "#;
/// let json = json.replace("\n", "").replace(" ", "");
/// let result = serde_json::to_string(&request).unwrap();
/// assert_eq!(json, result);
/// # }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Request<T> {
    /// A String specifying the version of the JSON-RPC protocol. MUST be exactly "2.0".
    pub jsonrpc: V2_0,
    /// A String containing the name of the method to be invoked.
    ///
    /// Method names that begin with the word rpc followed by a period character (U+002E or ASCII
    /// 46) are reserved for rpc-internal methods and extensions and MUST NOT be used for anything
    /// else.
    pub method: String,
    /// A Structured value that holds the parameter values to be used during the invocation of the
    /// method.
    pub params: Option<T>,
    /// The `id`. See [`Id`](enum.Id.html)
    pub id: Id,
}

impl<T: Serialize+DeserializeOwned> Request<T> {
    pub fn new(id: Id, method: String) -> Self {
        Self {
            jsonrpc: V2_0,
            method: method,
            params: None,
            id: id,
        }
    }

    pub fn with_params(id: Id, method: String, params: T) -> Self
    {
        Self {
            jsonrpc: V2_0,
            method: method,
            params: Some(params),
            id: id,
        }
    }
}

/// The jsonrpc Result response, indicating a successful result.
///
/// # Examples
///
/// ```rust
/// # extern crate jrpc;
/// extern crate serde_json;
/// use jrpc::{Id, Result};
///
/// # fn main() {
/// let data: Vec<u32> = vec![1, 2, 3];
/// let example = Result::new(Id::from(4), data.clone());
/// let json = r#"
/// {
///     "jsonrpc": "2.0",
///     "result": [1,2,3],
///     "id": 4
/// }
/// "#;
/// let json = json.replace("\n", "").replace(" ", "");
/// let result = serde_json::to_string(&example).unwrap();
/// assert_eq!(json, result);
/// # }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Result<T> {
    pub jsonrpc: V2_0,
    pub result: T,
    pub id: Id,
}

impl<T: Serialize+DeserializeOwned> Result<T> {
    pub fn new(id: Id, result: T) -> Self {
        Self {
            jsonrpc: V2_0,
            result: result,
            id: id,
        }
    }
}

/// The jsonrpc Error response, indicating an error.
///
/// Since the `T` in the `ErrorObject` will _at least_ be based on the `ErrorCode` it is
/// recommended that you deserialize this type as `T=Value` first.
///
/// # Examples
///
/// ```rust
/// # extern crate jrpc;
/// extern crate serde_json;
/// use jrpc::{Id, Error, ErrorCode, ErrorObject, V2_0};
///
/// # fn main() {
/// let data: Vec<u32> = vec![1, 2, 3];
/// let example = Error {
///     jsonrpc: V2_0,
///     error: ErrorObject {
///         code: ErrorCode::from(-32000),
///         message: "BadIndexes".into(),
///         data: Some(data.clone()),
///     },
///     id: Id::from(4),
/// };
///
/// let json = r#"
/// {
///     "jsonrpc": "2.0",
///     "error": {
///         "code": -32000,
///         "message": "BadIndexes",
///         "data": [1,2,3]
///     },
///     "id": 4
/// }
/// "#;
/// let json = json.replace("\n", "").replace(" ", "");
/// let result = serde_json::to_string(&example).unwrap();
/// assert_eq!(json, result);
///
/// // This is how it is recommended you deserialize:
/// let error: Error<serde_json::Value> =
///     serde_json::from_str(&json).unwrap();
/// if error.error.code != ErrorCode::ServerError(-32000) {
///     panic!("unexpected error");
/// }
/// let result: Vec<u32> = serde_json::from_value(
///     error.error.data.unwrap()
/// ).unwrap();
/// assert_eq!(data, result);
/// # }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Error<T> {
    pub jsonrpc: V2_0,
    pub error: ErrorObject<T>,
    pub id: Id,
}


/// The jsonrpc Error object, with details of the error.
///
/// Typically you may want to deserialze this with `T == serde_json::Value`
/// to first inspect the value of the `ErrorCode`.
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorObject<T> {
    pub code: ErrorCode,
    pub message: String,
    pub data: Option<T>,
}

/// An error code.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    /// - `-32700`: Parse error. Invalid JSON was received by the server.
    ///   An error occurred on the server while parsing the JSON text.
    ParseError,
    /// - `-32600`: Invalid Request. The JSON sent is not a valid Request object.
    InvalidRequest,
    /// - `-32601`: Method not found. The method does not exist / is not available.
    MethodNotFound,
    /// - `-32602`: Invalid params. Invalid method parameter(s).
    InvalidParams,
    /// - `-32603`: Internal error. Internal JSON-RPC error.
    InternalError,
    /// - `-32000 to -32099`: Server error. Reserved for implementation-defined server-errors.
    ServerError(i64),
}

impl ErrorCode {
    /// Return whether the ErrorCode is correct.
    ///
    /// This will only return `false` if this is ServerError and is outside of the range of -32000
    /// to -32099.
    fn is_valid(&self) -> bool {
        match *self {
            ErrorCode::ServerError(value) => {
                if (-32099 <= value) && (value <= -32000) {
                    true
                } else {
                    false
                }
            }
            _ => true,
        }
    }
}

impl From<i64> for ErrorCode {
    fn from(v: i64) -> ErrorCode {
        match v {
            -32700 => ErrorCode::ParseError,
            -32600 => ErrorCode::InvalidRequest,
            -32601 => ErrorCode::MethodNotFound,
            -32602 => ErrorCode::InvalidParams,
            -32603 => ErrorCode::InternalError,
            _ => ErrorCode::ServerError(v),
        }
    }
}
