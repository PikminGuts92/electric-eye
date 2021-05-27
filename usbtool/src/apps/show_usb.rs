use crate::apps::SubApp;
use clap::Clap;
use std::error::Error;

use electric_lib::usb::{HidDevice, HidManager};

#[derive(Clap, Debug)]
pub struct ShowUSBApp {
    #[clap(short = 'w', long = "watch", about = "Continuously watch changes of USB device connectivity")]
    pub watch_changes: bool,
}

impl SubApp for ShowUSBApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>> {
        let man = HidManager::new();
        let devices = man.get_devices();

        println!("Complete!");
        Ok(())
    }
}