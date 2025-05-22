use libdonow::parser::Todo;
use nu_plugin::PluginCommand;
use nu_protocol::{LabeledError, PipelineData, SyntaxShape, Type};

use crate::{TodoTxtPlugin, util::get_todo_file_contents};

pub struct TodoAdd;

impl PluginCommand for TodoAdd {
    type Plugin = TodoTxtPlugin;

    fn name(&self) -> &str {
        "todo add"
    }

    fn signature(&self) -> nu_protocol::Signature {
        nu_protocol::Signature::build(self.name())
            .input_output_type(Type::Nothing, Type::Nothing)
            .named(
                "file",
                SyntaxShape::Filepath,
                "path to your todo.txt file (default: ~/.todo.txt)",
                Some('f'),
            )
            .required("description", SyntaxShape::String, "todo item")
            .switch("complete", "add the task marked completed", Some('c'))
            .switch(
                "no-date",
                "don't add creation and/or completion date to item",
                Some('D'),
            )
            .named(
                "priority",
                SyntaxShape::String,
                "priority to give todo item",
                Some('p'),
            )
            .category(nu_protocol::Category::Custom("todo.txt".into()))
    }

    fn description(&self) -> &str {
        "add a new todo item to your todo.txt file"
    }

    fn run(
        &self,
        _plugin: &TodoTxtPlugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, nu_protocol::LabeledError> {
        let desc = call.req::<String>(0)?;
        let complete = call.has_flag("complete")?;
        let no_date = call.has_flag("no-date")?;
        let priority = call
            .get_flag::<String>("priority")?
            .and_then(|s| s.chars().nth(0));

        // Build the todo item string
        let mut todo_str = String::new();
        if complete {
            todo_str.push_str("x ");
        }
        if let Some(prio) = priority {
            let prio = match prio.to_ascii_uppercase() {
                c if c.is_ascii_alphabetic() => c,
                e => return Err(LabeledError::new(format!("unknown priority: {}", e))),
            };
            todo_str.push_str(&format!("({prio}) "));
        }
        if !no_date {
            let today = chrono::Local::now().format("%F");
            if complete {
                todo_str.push_str(&format!("{today} {today} "));
            } else {
                todo_str.push_str(&format!("{today} "));
            }
        }
        todo_str.push_str(&desc);

        // Create todo item from string
        let new_todo_item = Todo::parse(&todo_str)
            .map_err(|e| LabeledError::new(format!("Error creating todo item: {:?}", e)))?;

        let mut todo_table = get_todo_file_contents(call)?;
        todo_table.add(new_todo_item);

        todo_table.save();
        Ok(PipelineData::Empty)
    }
}
