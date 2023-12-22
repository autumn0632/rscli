use inquire::{
    ui::{RenderConfig, Styled},
    Text,
};

use crate::cmd;

enum CliInput {
    Cmd(cmd::CliCmd),

    Empty,
}

impl CliInput {
    pub fn new(input: &str) -> Self {
        match input {
            "init" => CliInput::Cmd(cmd::CliCmd::Init),
            "exit" => CliInput::Cmd(cmd::CliCmd::Exit),
            _ => CliInput::Empty,
        }
    }

    pub fn handle(&self) {
        match self {
            CliInput::Cmd(cmd) => cmd.exec_cmd(),
            CliInput::Empty => {}
        }
    }
}

pub fn pares_stdin() {
    loop {
        let input = Text::new("").prompt();
        match input {
            Ok(cmd) => CliInput::new(&cmd).handle(),
            Err(err) => println!("{}", err),
        }
    }
}

pub fn set_custom_render_config() {
    let mut render_config = RenderConfig::default();
    render_config.prompt_prefix = Styled::new(ddi_prompt_prefix());
    render_config.answered_prompt_prefix = Styled::new(ddi_prompt_prefix());
    inquire::set_global_render_config(render_config);
}

fn ddi_prompt_prefix() -> &'static str {
    "DDI#>>>"
}
