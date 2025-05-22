use nu_plugin::PluginCommand;
use nu_protocol::{PipelineData, Type};

use crate::{TodoTxtPlugin, util::get_todo_file_contents};

pub struct TodoList;

impl PluginCommand for TodoList {
    type Plugin = TodoTxtPlugin;

    fn name(&self) -> &str {
        "todo list"
    }

    fn signature(&self) -> nu_protocol::Signature {
        nu_protocol::Signature::build(self.name())
            .input_output_type(Type::Nothing, Type::Nothing)
            .named(
                "file",
                nu_protocol::SyntaxShape::Filepath,
                "path to your todo.txt file (default: ~/.todo.txt)",
                Some('f'),
            )
            .switch("all", "include completed tasks", Some('a'))
            .category(nu_protocol::Category::Custom("todo.txt".into()))
    }

    fn description(&self) -> &str {
        "Pretty-print your todo.txt file"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, nu_protocol::LabeledError> {
        let show_all = call.has_flag("all")?;
        let todo_file = get_todo_file_contents(call)?;
        for todo in todo_file
            .into_iter()
            .filter(|t| if show_all { true } else { !t.completed })
        {
            let content = format!("{}", todo);
            println!("{}", content.trim());
        }
        Ok(PipelineData::Empty)
    }
}
