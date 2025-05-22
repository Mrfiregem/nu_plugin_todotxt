use nu_plugin::{MsgPackSerializer, Plugin, PluginCommand};

mod util;

mod commands;
pub use commands::*;

pub struct TodoTxtPlugin;

impl Plugin for TodoTxtPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(commands::TodoTable),
            Box::new(commands::TodoAdd),
            Box::new(commands::TodoList),
        ]
    }
}

fn main() {
    nu_plugin::serve_plugin(&TodoTxtPlugin, MsgPackSerializer);
}
