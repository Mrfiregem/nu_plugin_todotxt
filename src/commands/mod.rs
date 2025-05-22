use std::path::PathBuf;

mod todo_table;

use nu_plugin::EvaluatedCall;
use nu_protocol::LabeledError;
pub use todo_table::TodoTable;

fn get_todo_file_path(call: &EvaluatedCall) -> Result<PathBuf, LabeledError> {
    match call.get_flag::<PathBuf>("file")? {
        Some(path) => Ok(path),
        None => dirs::home_dir()
            .ok_or_else(|| LabeledError::new("Could not find home directory"))
            .map(|x| x.join("todo.txt")),
    }
    .clone()
}
