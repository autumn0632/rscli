pub struct Files;

impl Files {
    pub fn new() -> Self {
        Files
    }

    pub fn exec(&mut self) -> Result<(), String> {
        println!("start files server: http://0.0.0.0:8080");
        Ok(())
    }
}
