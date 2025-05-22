use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, IntoPipelineData, LabeledError, PipelineData, SyntaxShape, Type};

use crate::TodoTxtPlugin;

use super::{get_todo_file_as_json, value_from_json};

pub struct TodoTable;

impl PluginCommand for TodoTable {
    type Plugin = TodoTxtPlugin;

    fn name(&self) -> &str {
        "todo table"
    }

    fn signature(&self) -> nu_protocol::Signature {
        nu_protocol::Signature::build(self.name())
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
        open_todo_file_as_table(call)
    }
}

/// Read the todo.txt file specified in the call and return it as a nu table
pub fn open_todo_file_as_table(call: &EvaluatedCall) -> Result<PipelineData, LabeledError> {
    let todo_file = get_todo_file_as_json(call)?;
    Ok(value_from_json(&todo_file, call.head)).map(|x| x.into_pipeline_data())
}
