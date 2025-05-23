use nu_protocol::{LabeledError, ShellError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodoPluginError {
    #[error("error modifying file: {0}")]
    Io(#[from] std::io::Error),
    #[error("error from nushell code: {0}")]
    Nushell(#[from] nu_protocol::LabeledError),
    #[error("missing home directory")]
    MissingHomeDirectory,
    #[error("no task ids given")]
    NoIndex,
    #[error("given id out of range")]
    IndexOutOfRange,
    #[error("unknown todotxt plugin error")]
    Unknown,
}

impl From<TodoPluginError> for LabeledError {
    fn from(value: TodoPluginError) -> Self {
        match value {
            TodoPluginError::Nushell(labeled_error) => labeled_error,
            TodoPluginError::Unknown => LabeledError::new("error encountered while running")
                .with_code("todotxt::error::unknown")
                .with_help("consider reporting this error on github"),
            TodoPluginError::Io(error) => {
                LabeledError::new(format!("encountered io error: {error}"))
                    .with_code("todotxt::error::std::io")
            }
            TodoPluginError::MissingHomeDirectory => {
                LabeledError::new("could not determine home directory location")
                    .with_code("todotxt::error::missing_home_directory")
            }
            TodoPluginError::NoIndex => LabeledError::new("no task ids specified")
                .with_code("todotxt::error::mising_index")
                .with_help("read command help for expected signiture"),
            TodoPluginError::IndexOutOfRange => LabeledError::new("given index is out of range")
                .with_code("todotxt::error::index_out_of_range"),
        }
    }
}

impl From<ShellError> for TodoPluginError {
    fn from(value: ShellError) -> Self {
        TodoPluginError::Nushell(value.into())
    }
}
