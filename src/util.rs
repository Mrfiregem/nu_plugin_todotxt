use std::path::PathBuf;

use libdonow::file::TodoFile;
use nu_protocol::{LabeledError, Value};
use serde_json::Value as JsonValue;

/// Get the path to the user's todo.txt file
pub fn get_todo_file_path(call: &nu_plugin::EvaluatedCall) -> Result<PathBuf, LabeledError> {
    match call.get_flag::<std::path::PathBuf>("file")? {
        Some(path) => Ok(path),
        None => dirs::home_dir()
            .ok_or_else(|| {
                LabeledError::new("Could not find home directory")
                    .with_code("todotxt::fs::missing_home_dir")
            })
            .map(|x| x.join("todo.txt")),
    }
}

/// Read the todo.txt file specified in the call and return it as a JSON object
pub fn get_todo_file_contents(call: &nu_plugin::EvaluatedCall) -> Result<TodoFile, LabeledError> {
    let file_path = get_todo_file_path(call)?;

    // If the file doesn't exist, create it
    if !file_path.exists() {
        std::fs::File::create(&file_path).map_err(|e| {
            LabeledError::new(format!("Could not create file: {}", e))
                .with_code("todotxt::fs::could_not_create")
        })?;
    }

    Ok(TodoFile::new(&file_path.to_string_lossy()))
}

/// Convert a serde_json::Value to a nu_protocol::Value
pub fn value_from_json(item: &JsonValue, span: nu_protocol::Span) -> Value {
    match item {
        JsonValue::Null => Value::nothing(span),
        JsonValue::Bool(bool) => Value::bool(*bool, span),
        JsonValue::Number(number) => {
            if number.is_f64() {
                Value::float(number.as_f64().expect("number should be f64"), span)
            } else {
                Value::int(number.as_i64().expect("number should be integer"), span)
            }
        }
        JsonValue::String(string) => Value::string(string.trim(), span),
        JsonValue::Object(map) => {
            let mut rec = nu_protocol::Record::new();
            for (key, value) in map {
                rec.insert(key, value_from_json(value, span));
            }
            Value::record(rec, span)
        }
        JsonValue::Array(values) => Value::list(
            values.iter().map(|v| value_from_json(v, span)).collect(),
            span,
        ),
    }
}
