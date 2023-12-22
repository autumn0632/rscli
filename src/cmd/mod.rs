use std::process::exit;

mod init;
pub use init::Init;

pub enum CliCmd {
    Init,
    Exit,
    Unknown(String),
}
impl CliCmd {
    pub fn exec_cmd(&self) -> Result<(), String> {
        match self {
            CliCmd::Init => Init::new().exec(),
            CliCmd::Exit => {
                exit(0);
                #[allow(unreachable_code)]
                Ok(())
            }
            CliCmd::Unknown(cmd) => Err(format!("Unknown command: {}", cmd)),
        }
    }
}
