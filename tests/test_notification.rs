extern crate jrpc;
extern crate serde_json;

use jrpc::*;

#[test]
fn test_notification_id() {
    let value: Vec<u32> = vec![1, 2, 3];
    let request = Request::with_params(
        IdReq::Notification,
        "CreateFoo".to_string(),
        Some(value.clone()),
    );
    let json = r#"
    {
      "jsonrpc":"2.0",
      "method":"CreateFoo",
      "params":[1,2,3]
    }
    "#;
    let json = json.replace("\n", "").replace(" ", "");
    let result = serde_json::to_string(&request).unwrap();
    assert_eq!(json, result);
}

#[test]
fn test_request_id() {
    let value: Vec<u32> = vec![1, 2, 3];
    let request = Request::with_params(
        Id::from(7),
        "CreateFoo".to_string(),
        Some(value.clone()),
    );
    let json = r#"
    {
      "jsonrpc":"2.0",
      "method":"CreateFoo",
      "params":[1,2,3],
      "id": 7
    }
    "#;
    let json = json.replace("\n", "").replace(" ", "");
    let result = serde_json::to_string(&request).unwrap();
    assert_eq!(json, result);
}
