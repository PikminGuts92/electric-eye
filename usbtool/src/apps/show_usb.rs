use crate::apps::SubApp;
use clap::Clap;
use std::error::Error;

#[derive(Clap, Debug)]
pub struct ShowUSBApp {
    #[clap(short = 'w', long = "watch", about = "Continuously watch changes of USB device connectivity")]
    pub watch_changes: bool
}

impl SubApp for ShowUSBApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>> {

        Ok(())
    }
}