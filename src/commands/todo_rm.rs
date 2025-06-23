use nu_plugin::PluginCommand;
use nu_protocol::{PipelineData, SyntaxShape, Type};
use todo_txt::task::Simple;

use crate::{
    TodoTxtPlugin,
    error::TodoPluginError,
    util::{get_todo_file_contents, write_tasks_to_disk},
};

pub struct TodoRm;

impl PluginCommand for TodoRm {
    type Plugin = TodoTxtPlugin;

    fn name(&self) -> &str {
        "todo rm"
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
            .switch("dry-run", "ask before removing each task", Some('n'))
            .rest("id", SyntaxShape::Int, "id(s) to remove")
            .category(nu_protocol::Category::Custom("todotxt".into()))
    }

    fn description(&self) -> &str {
        "remove tasks by the given id(s)"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, nu_protocol::LabeledError> {
        let ids = call.rest::<usize>(0)?;
        let mut task_list = get_todo_file_contents::<Simple>(call)?;
        let dry_run = call.has_flag("dry-run")?;

        validate_ids(&ids, task_list.len())?;

        // Loop over the task list backwards, removing tasks with matching indexes
        let mut removals = Vec::new();
        for (idx, task) in task_list.tasks.iter().enumerate() {
            if ids.contains(&idx) {
                match dry_run {
                    true => println!("Would remove #{idx}: {task}"),
                    false => removals.push(idx),
                }
            }
        }

        // If not in dry-run mode, write changes to the file
        if !dry_run {
            task_list = task_list
                .iter()
                .enumerate()
                .filter(|(index, _)| !removals.contains(index))
                .map(|(_, task)| task)
                .collect();
            write_tasks_to_disk(call, task_list)?;
        }

        Ok(PipelineData::Empty)
    }
}

/// Error if no ids provided, or an id greater than the number of tasks exists
fn validate_ids(ids: &[usize], list_size: usize) -> Result<(), TodoPluginError> {
    if ids.is_empty() {
        return Err(TodoPluginError::NoIndex);
    }
    if !ids.iter().all(|i| *i < list_size) {
        return Err(TodoPluginError::IndexOutOfRange);
    }
    Ok(())
}
