use std::fmt::Display;
use std::io::Write;
use std::{path::PathBuf, str::FromStr};

use nu_plugin::EvaluatedCall;
use nu_protocol::Value;
use serde_json::Value as JsonValue;
use todo_txt::{Task, task::List};

use crate::error::TodoPluginError;

/// Get the path to the user's todo.txt file
pub fn get_todo_file_path(call: &EvaluatedCall) -> Result<PathBuf, TodoPluginError> {
    match call.get_flag::<std::path::PathBuf>("file")? {
        Some(path) => Ok(path),
        None => dirs::home_dir()
            .ok_or(TodoPluginError::MissingHomeDirectory)
            .map(|x| x.join("todo.txt")),
    }
}

/// Read the todo.txt file specified in the call and return it as a JSON object
pub fn get_todo_file_contents<T>(call: &EvaluatedCall) -> Result<List<T>, TodoPluginError>
where
    T: Task,
{
    let file_path = get_todo_file_path(call)?;

    // If the file doesn't exist, create it
    if !file_path.exists() {
        std::fs::File::create(&file_path)?;
    }

    let contents = std::fs::read_to_string(file_path)?;

    Ok(todo_txt::task::List::from_str(&contents).expect("reached infallable error"))
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

/// Given a list of tasks, write to todo.txt file
pub fn write_tasks_to_disk<T>(
    call: &EvaluatedCall,
    task_list: List<T>,
) -> Result<(), TodoPluginError>
where
    T: Task + Display,
{
    let file_path = get_todo_file_path(call)?;
    let mut file = std::fs::OpenOptions::new().write(true).open(file_path)?;

    for task in task_list.tasks {
        writeln!(file, "{}", task).map_err(TodoPluginError::from)?;
    }

    Ok(())
}
