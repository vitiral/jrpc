//! # jrpc: an ultra lightweight crate for types from the jsonrpc spec
//!
//! This crate defines the datatypes for the jsonrpc spec... and that is IT.
//!
//! This crate never touches the network, filesystem, etc. It simply uses serde
//! to easily construct, serialize and deserialize Request and Response data types.
//!
//! # Specification
//!
//! The below is directly copy/pasted from: [http://www.jsonrpc.org/specification][spec]
//!
//! The types try to correctly copy the relevant documentation snippets in their docstring.
//!
//! [spec]: http://www.jsonrpc.org/specification
//!
//! ## 1 Overview
//!
//! JSON-RPC is a stateless, light-weight remote procedure call (RPC) protocol. Primarily this
//! specification defines several data structures and the rules around their processing. It is
//! transport agnostic in that the concepts can be used within the same process, over sockets, over
//! http, or in many various message passing environments. It uses JSON (RFC 4627) as data format.
//!
//! It is designed to be simple!
//!
//! # 2 Conventions
//!
//! The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT",
//! "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted
//! as described in RFC 2119.
//!
//! Since JSON-RPC utilizes JSON, it has the same type system (see http://www.json.org or RFC
//! 4627). JSON can represent four primitive types (Strings, Numbers, Booleans, and Null) and two
//! structured types (Objects and Arrays). The term "Primitive" in this specification references
//! any of those four primitive JSON types. The term "Structured" references either of the
//! structured JSON types. Whenever this document refers to any JSON type, the first letter is
//! always capitalized: Object, Array, String, Number, Boolean, Null. True and False are also
//! capitalized.
//!
//! All member names exchanged between the Client and the Server that are considered for matching
//! of any kind should be considered to be case-sensitive. The terms function, method, and
//! procedure can be assumed to be interchangeable.
//!
//! The Client is defined as the origin of Request objects and the handler of Response objects.
//!
//! The Server is defined as the origin of Response objects and the handler of Request objects.
//!
//! One implementation of this specification could easily fill both of those roles, even at the
//! same time, to other different clients or the same client. This specification does not address
//! that layer of complexity.
//!
//! ## 3 Compatibility
//!
//! JSON-RPC 2.0 Request objects and Response objects may not work with existing JSON-RPC 1.0
//! clients or servers. However, it is easy to distinguish between the two versions as 2.0 always
//! has a member named "jsonrpc" with a String value of "2.0" whereas 1.0 does not. Most 2.0
//! implementations should consider trying to handle 1.0 objects, even if not the peer-to-peer and
//! class hinting aspects of 1.0.
//!
//! ## 4 Request Object
//!
//! See [`Request`](struct.Request.html)
//!
//!
//! ## 4.1 Notification
//!
//! See [`IdReq`](enum.IdReq.html)
//!
//! ## 4.2 Parameter Structures
//!
//! See [`Request.params`](struct.Request.html#structfield.params)
//!
//! ## 5 Response object
//!
//! See [`Response`](struct.Response.html)
//!
//! ## 5.1 Error object
//!
//! See [`ErrorObject`](struct.ErrorObject.html)
//!
//! ## 6 Batch
//!
//! > Note: simply use a `Vec<Request>` and `Vec<Response>`
//!
//! To send several Request objects at the same time, the Client MAY send an Array filled with
//! Request objects.
//!
//! The Server should respond with an Array containing the corresponding Response objects, after
//! all of the batch Request objects have been processed. A Response object SHOULD exist for each
//! Request object, except that there SHOULD NOT be any Response objects for notifications. The
//! Server MAY process a batch rpc call as a set of concurrent tasks, processing them in any order
//! and with any width of parallelism.
//!
//! The Response objects being returned from a batch call MAY be returned in any order within the
//! Array. The Client SHOULD match contexts between the set of Request objects and the resulting
//! set of Response objects based on the id member within each Object.
//!
//! If the batch rpc call itself fails to be recognized as an valid JSON or as an Array with at
//! least one value, the response from the Server MUST be a single Response object. If there are no
//! Response objects contained within the Response array as it is to be sent to the client, the
//! server MUST NOT return an empty Array and should return nothing at all.
//!
//! ## 7 Examples
//!
//! Ommitted. See the [specification][spec]
//!
//! ## 8 Extensions
//!
//! See [`Request::is_system_extension`](struct.Request.html#method.is_system_extension)
#![allow(unknown_lints)]
#![allow(redundant_field_names)]
#![warn(missing_docs)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate std_prelude;

pub use serde_json::Value;

mod serialize;

use std_prelude::*;
use serde::ser::Serialize;
use serde::de::DeserializeOwned;

/// The `jsonrpc` version. Will serialize/deserialize to/from `"2.0"`.
pub struct V2_0;

/// An identifier established by the Client that MUST contain a String, Number, or NULL value if
/// included. If it is not included it is assumed to be a notification. The value SHOULD normally
/// not be Null and Numbers SHOULD NOT contain fractional parts
///
/// The Server MUST reply with the same value in the Response object if included. This member is
/// used to correlate the context between the two objects.
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
    /// An String id
    String(String),
    /// An Number id that must be an integer.
    ///
    /// We intentionally do not allow floating point values.
    Int(i64),
    /// A null id
    Null,
}

impl From<String> for Id {
    fn from(s: String) -> Self {
        Id::String(s)
    }
}

impl<'a> From<&'a str> for Id {
    fn from(s: &'a str) -> Self {
        Id::String(s.into())
    }
}

impl From<i64> for Id {
    fn from(v: i64) -> Self {
        Id::Int(v)
    }
}

/// Identical to [`Id`](enum.Id.html) except has the Notification type. Typically you should use
/// `Id` since all functions that would accept IdReq accept `Into<IdReq>`.
///
/// # Notification
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
/// https://github.com/serde-rs/serde/issues/984
///
/// # Examples
/// This just demonstrates what happens if `id` is absent vs null.
///
/// ```rust
/// # extern crate jrpc;
/// extern crate serde_json;
/// use jrpc::{Id, IdReq, Request, Value};
///
/// # fn main() {
/// // id == null
/// let json = r#"
/// {
///     "jsonrpc": "2.0",
///     "method": "CreateFoo",
///     "id": null
/// }
/// "#;
/// let request: Request<Value> = serde_json::from_str(json).unwrap();
/// assert_eq!(request.id, Id::Null.into());
///
/// // id does not exist
/// let json = r#"
/// {
///     "jsonrpc": "2.0",
///     "method": "NotifyFoo"
/// }
/// "#;
/// let request: Request<Value> = serde_json::from_str(json).unwrap();
/// assert_eq!(request.id, IdReq::Notification);
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdReq {
    /// An String id
    String(String),
    /// An Number id that must be an integer.
    ///
    /// We intentionally do not allow floating point values.
    Int(i64),
    /// A null id
    Null,
    /// The notification id, i.e. the id is absent.
    Notification,
}

impl From<Id> for IdReq {
    fn from(id: Id) -> Self {
        match id {
            Id::String(s) => IdReq::String(s),
            Id::Int(i) => IdReq::Int(i),
            Id::Null => IdReq::Null,
        }
    }
}

/// A rpc call is represented by sending a Request object to a Server.
///
/// See the parameters for details.
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
    /// Method names that begin with the word rpc followed by a period character (`'.'`, `U+002E`
    /// or ASCII `0x2e`) are reserved for rpc-internal methods and extensions and MUST NOT be used
    /// for anything else.
    pub method: String,

    /// A Structured value that holds the parameter values to be used during the invocation of the
    /// method.
    ///
    /// ## Spec Requiement
    ///
    /// > Note: the following spec is **not** upheld by this library.
    ///
    /// If present, parameters for the rpc call MUST be provided as a Structured value. Either
    /// by-position through an Array or by-name through an Object.
    ///
    /// - by-position: params MUST be an Array, containing the values in the Server expected
    ///   order.
    /// - by-name: params MUST be an Object, with member names that match the Server expected
    ///   parameter names. The absence of expected names MAY result in an error being
    ///   generated. The names MUST match exactly, including case, to the method's expected
    ///   parameters.
    ///
    #[serde(default)]
    pub params: Option<T>,

    /// The `id`. See [`Id`](enum.Id.html)
    #[serde(default = "notification")]
    pub id: IdReq,
}

impl<T: Serialize + DeserializeOwned> Request<T> {
    /// Return whether the method name is defined as a system extension.
    ///
    /// Method names that begin with `"rpc."` are reserved for system extensions, and MUST NOT be
    /// used for anything else. Each system extension is defined in a related specification. All
    /// system extensions are OPTIONAL.
    pub fn is_system_extension(&self) -> bool {
        self.method.starts_with(".rpc")
    }

    /// Helper to serialize the Request as json.
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Helper to deserialize the Request from json.
    pub fn from_str(s: &str) -> serde_json::Result<T>
    {
        serde_json::from_str(s)
    }
}

impl<T: Serialize + DeserializeOwned> Request<T> {
    /// Create a new Request.
    pub fn new<I, S>(id: I, method: S) -> Self
    where
        I: Into<IdReq>,
        S: Into<String>,
    {
        Self {
            jsonrpc: V2_0,
            method: method.into(),
            params: None,
            id: id.into(),
        }
    }

    /// Create a new Request with the specified params.
    pub fn with_params<I>(id: I, method: String, params: T) -> Self
    where
        I: Into<IdReq>,
    {
        Self {
            jsonrpc: V2_0,
            method: method,
            params: Some(params),
            id: id.into(),
        }
    }
}

/// The Result is either:
/// - a jsonrpc Response (with a result of a specific type)
/// - a Error (with an error of type `serde_json::Value`).
///
/// # Example
///
/// ```rust
/// # extern crate jrpc;
/// extern crate serde_json;
/// use jrpc::{Id, Response};
///
/// # fn main() {
/// let data: Vec<u32> = vec![1, 2, 3];
/// let example = Response::success(Id::from(4), data.clone());
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
#[serde(untagged)]
pub enum Response<T> {
    /// The Response has a `result` object and not an `error` object.
    Ok(Success<T>),
    /// The Response has a `error` object and not an `result` object.
    Err(Error<Value>),
}

impl<T: Serialize + DeserializeOwned> Response<T> {
    /// Retrieve the `id` regardless of whether there was an error or not.
    pub fn id(&self) -> &Id {
        match *self {
            Response::Ok(ref r) => &r.id,
            Response::Err(ref e) => &e.id,
        }
    }

    /// Construct a `Success`
    pub fn success(id: Id, result: T) -> Self {
        Response::Ok(Success::new(id, result))
    }

    /// Construct an `Error`
    pub fn error<C, S>(id: Id, code: C, message: S, data: Option<Value>) -> Self
        where C: Into<ErrorCode>,
              S: Into<String>,
    {
        Response::Err(Error::new(id, code, message, data))
    }

    /// Helper to serialize the Response as json.
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Helper to deserialize the Response from json.
    pub fn from_str(s: &str) -> serde_json::Result<T>
    {
        serde_json::from_str(s)
    }
}

/// The jsonrpc Success response, indicating a successful result.
///
/// See the parameters for more information.
///
/// # Examples
///
/// ```rust
/// # extern crate jrpc;
/// extern crate serde_json;
/// use jrpc::{Id, Success};
///
/// # fn main() {
/// let data: Vec<u32> = vec![1, 2, 3];
/// let example = Success::new(Id::from(4), data.clone());
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
#[serde(deny_unknown_fields)]
pub struct Success<T> {
    /// A String specifying the version of the JSON-RPC protocol. MUST be exactly "2.0".
    pub jsonrpc: V2_0,

    /// The value of this member is determined by the method invoked on the Server.
    pub result: T,

    /// This member is REQUIRED.
    ///
    /// It MUST be the same as the value of the id member in the Request Object.
    ///
    /// If there was an error in detecting the id in the Request object (e.g. Parse error/Invalid
    /// Request), it MUST be Null.
    pub id: Id,
}

impl<T: Serialize + DeserializeOwned> Success<T> {
    /// Construct a `Success`, i.e. a Response with a `result` object.
    pub fn new(id: Id, result: T) -> Self {
        Self {
            jsonrpc: V2_0,
            result: result,
            id: id,
        }
    }

    /// Helper to serialize the Success as json.
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Helper to deserialize the Success from json.
    pub fn from_str(s: &str) -> serde_json::Result<T>
    {
        serde_json::from_str(s)
    }
}

/// The jsonrpc Error response, indicating an error.
///
/// # Examples
///
/// Since the `T` in the `ErrorObject` will _at least_ be based on the `ErrorCode` it is
/// recommended that you deserialize this type as `T=Value` first.
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
#[serde(deny_unknown_fields)]
pub struct Error<T> {
    /// Always "2.0"
    pub jsonrpc: V2_0,
    /// The error object.
    pub error: ErrorObject<T>,
    /// The id of the request.
    pub id: Id,
}

impl<T: Serialize + DeserializeOwned> Error<T> {
    /// Helper to create a new `Error` object.
    pub fn new<C, S>(id: Id, code: C, message: S, data: Option<T>) -> Self
        where C: Into<ErrorCode>,
              S: Into<String>,
    {
        Error {
            jsonrpc: V2_0,
            error: ErrorObject {
                code: code.into(),
                message: message.into(),
                data: data,
            },
            id: id,
        }
    }

    /// Helper to serialize the Error as json.
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Helper to deserialize the Error from json.
    pub fn from_str(s: &str) -> serde_json::Result<T>
    {
        serde_json::from_str(s)
    }
}

/// The jsonrpc Error object, with details of the error.
///
/// When a rpc call encounters an error, the Response Object MUST contain the error member with a
/// value that is a Object. See the attributes for details.
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorObject<T> {
    /// The error code. See [`ErrorCode`](enum.ErrorCode.html)
    pub code: ErrorCode,

    /// A String providing a short description of the error.
    ///
    /// The message SHOULD be limited to a concise single sentence.
    pub message: String,

    /// A Primitive or Structured value that contains additional information about the error.
    ///
    /// This may be omitted.
    ///
    /// The value of this member is defined by the Server (e.g. detailed error
    /// information, nested errors etc.).
    #[serde(default = "default_t")]
    pub data: Option<T>,
}

/// A Number that indicates the error type that occurred.
/// This MUST be an integer.
///
/// The error codes from and including -32768 to -32000 are reserved for pre-defined errors.
/// Any code within this range, but not defined explicitly below is reserved for future use.
/// The error codes are nearly the same as those suggested for XML-RPC at the following url:
/// http://xmlrpc-epi.sourceforge.net/specs/rfc.fault_codes.php
///
/// Use the [`is_valid()`](enum.ErrorCode.html#method.is_valid) method to determine compliance.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
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
    /// This will only return `false` if this is `ServerError` and is outside of the range of -32000
    /// to -32099.
    pub fn is_valid(&self) -> bool {
        match *self {
            ErrorCode::ServerError(value) => (-32099 <= value) && (value <= -32000),
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

fn notification() -> IdReq {
    IdReq::Notification
}

fn default_t<T>() -> Option<T> {
    None
}
