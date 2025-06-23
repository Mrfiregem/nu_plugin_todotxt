use miette::Diagnostic;
use nu_protocol::{LabeledError, ShellError};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum TodoPluginError {
    #[error("error modifying file: {0}")]
    #[diagnostic(code(todotxt::error::std::io))]
    Io(#[from] std::io::Error),
    #[error("error from nushell code: {0}")]
    #[diagnostic(code(todotxt::error::nushell))]
    Nushell(#[from] LabeledError),
    #[error("missing home directory")]
    #[diagnostic(code(todotxt::error::missing_home_directory))]
    MissingHomeDirectory,
    #[error("no task ids given")]
    #[diagnostic(code(todotxt::error::mising_index))]
    NoIndex,
    #[error("given id out of range")]
    #[diagnostic(code(todotxt::error::index_out_of_range))]
    IndexOutOfRange,
    #[error("unknown priority: {0}")]
    #[diagnostic(code(todotxt::error::unknown_priority))]
    UnknownPriority(char),
    #[error("unknown todo.txt plugin error")]
    #[diagnostic(code(todotxt::error::unknown))]
    Unknown,
}

impl From<TodoPluginError> for LabeledError {
    fn from(value: TodoPluginError) -> Self {
        Self::from_diagnostic(&value)
    }
}

impl From<ShellError> for TodoPluginError {
    fn from(value: ShellError) -> Self {
        TodoPluginError::Nushell(value.into())
    }
}
