use std::process::exit;

mod init;
pub use init::Init;

pub enum CliCmd {
    Init,
    Exit,
    Unknown(String),
}

impl CliCmd {
    pub fn exec_cmd(&self) {
        match self {
            CliCmd::Init => Init::new().running(),
            CliCmd::Exit => exit(0),
            CliCmd::Unknown(cmd) => println!("Unknown command: {}", cmd),
        }
    }
}
