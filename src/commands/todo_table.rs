use crate::TodoTxtPlugin;
use crate::util::{get_todo_file_contents, value_from_json};
use nu_plugin::{EvaluatedCall, PluginCommand};
use nu_protocol::{IntoPipelineData, LabeledError, PipelineData, Type};
use serde_json::json;
use todo_txt::task::Simple;

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
                nu_protocol::SyntaxShape::Filepath,
                "path to your todo.txt file (default: ~/.todo.txt)",
                Some('f'),
            )
            .category(nu_protocol::Category::Custom("todo.txt".into()))
    }

    fn description(&self) -> &str {
        "Render your todo.txt file as a table"
    }

    fn run(
        &self,
        _plugin: &TodoTxtPlugin,
        _engine: &nu_plugin::EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        open_todo_file_as_table(call)
    }
}

/// Read the todo.txt file specified in the call and return it as a nu table
pub fn open_todo_file_as_table(call: &EvaluatedCall) -> Result<PipelineData, LabeledError> {
    let todo_file = get_todo_file_contents::<Simple>(call)?;
    let json_rep = json!(todo_file.tasks);
    Ok(value_from_json(&json_rep, call.head).into_pipeline_data())
}
