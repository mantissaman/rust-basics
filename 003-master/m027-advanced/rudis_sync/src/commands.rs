use crate::RUDIS_DB;
use resp::Value;

pub fn process_client_request(decoded_msg: Value) -> Vec<u8> {
    let reply = if let Value::Array(v) = decoded_msg {
        match &v[0] {
            Value::Bulk(ref s) if s=="GET" || s=="get" => handle_get(v),
            Value::Bulk(ref s) if s=="SET" || s=="set" => handle_set(v),
            Value::Bulk(ref s) if s=="COMMAND" || s=="command" => {
                Ok(Value::String("Ready".to_string()))
            },
            other => unimplemented!("{:?} is not supported", other)
        }
    } else{
        Err(Value::Error("Invalid Command".to_string()))
    };

    match reply {
        Ok(r) | Err(r) => r.encode()
    }
}

pub fn handle_get(v: Vec<Value>) -> Result<Value, Value> {
    let v = v.iter().skip(1).collect::<Vec<_>>();
    if v.is_empty() {
        return Err(Value::Error("Expected 1 argument for GET command".to_string()))
    }
    let db_ref = RUDIS_DB.lock().unwrap();
    let reply = if let Value::Bulk(ref s) = &v[0] {
        db_ref.get(s).map(|e| Value::Bulk(e.to_string())).unwrap_or(Value::Null)
    } else {
        Value::Null
    };
    Ok(reply)
}

pub fn handle_set(v: Vec<Value>) -> Result<Value, Value> {
    let v = v.iter().skip(1).collect::<Vec<_>>();
    if v.is_empty() || v.len() < 2 {
        return Err(Value::Error("Expected 2 arguments for SET command".to_string()))
    }
    match (&v[0], &v[1]) {
        (Value::Bulk(k), Value::Bulk(v)) => {
            let _ = RUDIS_DB
                .lock()
                .unwrap()
                .insert(k.to_string(), v.to_string());
        }
        _ => unimplemented!("SET not implemented for {:?}", v),
    }

    Ok(Value::String("OK".to_string()))
}