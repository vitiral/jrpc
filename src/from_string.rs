use std_prelude::*;
use std::fmt;
use std::result;
use serde::ser::Serialize;
use serde::de::{Deserialize, DeserializeOwned};
use serde::de;

use super::*;

/// Deserialize a jsonrpc Response into a rust Result.
///
/// Autohandles helpful error messages.
pub fn from_str<T: Serialize + DeserializeOwned>(
    s: &str,
) -> result::Result<Result<T>, DeResultError> {
    let result: result::Result<Response<T>, _> = serde_json::from_str(s);
    let result_error = match result {
        Ok(r) => return Ok(Ok(r)),
        Err(e) => e.to_string(),
    };
    let error: result::Result<Error<Value>, _> = serde_json::from_str(s);
    if let Ok(e) = error {
        return Ok(Err(e));
    }

    let value: Value = match serde_json::from_str(s) {
        Ok(v) => v,
        Err(e) => {
            return Err(DeResultError {
                hint: format!("Invalid JSON: {}", e),
            })
        }
    };

    let mut object = match value {
        Value::Object(o) => o,
        _ => {
            return Err(DeResultError {
                hint: format!("Not an object: {:?}", s),
            })
        }
    };

    let v2_0 = "2.0".to_string();
    match object.remove("jsonrpc") {
        Some(Value::String(v2_0)) => {}
        None => {
            return Err(DeResultError {
                hint: "jsonrpc attribute does not exist".into(),
            })
        }
        v @ _ => {
            return Err(DeResultError {
                hint: format!("jsonrpc attribute is the incorrect value: {:?}", v,),
            })
        }
    }

    let id = match object.remove("id") {
        Some(id) => id,
        None => {
            return Err(DeResultError {
                hint: "id does not exist".into(),
            });
        }
    };

    match id {
        Value::Null | Value::String(_) => {}
        Value::Number(n) => {
            if !n.is_i64() {
                return Err(DeResultError::new(format!(
                    "id is a non-i64 number: {:?}",
                    n
                )));
            }
        }
        i @ _ => {
            return Err(DeResultError::new(format!(
                "id is not a valid type: {:?}",
                i
            )));
        }
    }

    let result = object.remove("result");
    let error = object.remove("error");

    if result.is_some() && error.is_some() {
        return Err(DeResultError {
            hint: "both `result` and `error` fields are present".into(),
        });
    }

    if !object.is_empty() {
        let mut keys: Vec<_> = object.keys().collect();
        keys.sort();
        return Err(DeResultError {
            hint: format!("Extra keys are present: {:?}", keys),
        });
    }

    // TODO: look into the error object more.

    Err(DeResultError {
        hint: format!(
            "Could not deserialize into either Response or Error\
            Possible cause:\n{}", &result_error),
    })
}

