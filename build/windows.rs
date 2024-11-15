use std::error::Error;

pub struct Windows {

}

impl Windows {
    pub fn new() -> Self {
        Self {
            
        }
    }

    pub fn install(&self) -> Result<(), Box<dyn Error>> {
        winres::WindowsResource::new()
            .set_icon("resources/money-svgrepo-com.ico")
            .compile()?;
        Ok(())
    }
}