use nu_plugin::{MsgPackSerializer, Plugin, PluginCommand, serve_plugin};

mod error;
mod util;

mod commands;

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
            Box::new(commands::TodoRm),
        ]
    }
}

fn main() {
    serve_plugin(&TodoTxtPlugin, MsgPackSerializer);
}
