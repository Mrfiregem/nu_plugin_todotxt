use libdonow::file::TodoFile;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{
    Category, IntoPipelineData, LabeledError, PipelineData, Record, Signature, Span, SyntaxShape,
    Type, Value,
};
use serde_json::Value as JsonValue;

use crate::TodoTxtPlugin;

use super::get_todo_file_path;

pub struct TodoTable;

impl PluginCommand for TodoTable {
    type Plugin = TodoTxtPlugin;

    fn name(&self) -> &str {
        "todo table"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(self.name())
            .input_output_type(Type::Nothing, Type::table())
            .named(
                "file",
                SyntaxShape::Filepath,
                "path to your todo.txt file (default: ~/.todo.txt)",
                Some('f'),
            )
            .category(Category::Custom("todo.txt".into()))
    }

    fn description(&self) -> &str {
        "Render your todo.txt file as a table"
    }

    fn run(
        &self,
        _plugin: &TodoTxtPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let todo_file_path = get_todo_file_path(call)?;
        let todo_file: JsonValue = TodoFile::new(&todo_file_path.to_string_lossy()).as_json();

        Ok(value_from_json(&todo_file, call.head).into_pipeline_data())
    }
}

fn value_from_json(item: &JsonValue, span: Span) -> Value {
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
        JsonValue::String(string) => Value::string(string, span),
        JsonValue::Object(map) => {
            let mut rec = Record::new();
            for (key, value) in map.iter() {
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
