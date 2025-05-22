use std::io::Write;
use std::{fs::OpenOptions, str::FromStr};

use nu_plugin::PluginCommand;
use nu_protocol::{LabeledError, PipelineData, SyntaxShape, Type};
use todo_txt::task::Simple;

use crate::{
    TodoTxtPlugin,
    util::{get_todo_file_contents, get_todo_file_path},
};

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
        let new_todo_item = todo_txt::task::Simple::from_str(&todo_str)
            .map_err(|_| LabeledError::new("Unable to init new task object"))?;

        let mut todo_table = get_todo_file_contents::<Simple>(call)?;
        todo_table.push(new_todo_item);

        let file_path = get_todo_file_path(call)?;
        let mut file = OpenOptions::new()
            .write(true)
            .open(file_path)
            .map_err(|e| LabeledError::new(format!("Error opening todo file for writing: {e}")))?;

        for task in todo_table.tasks {
            match writeln!(file, "{}", task) {
                Ok(_) => {}
                Err(e) => {
                    return Err(LabeledError::new(format!(
                        "Error writing task to file: {e}"
                    )));
                }
            };
        }

        Ok(PipelineData::Empty)
    }
}
