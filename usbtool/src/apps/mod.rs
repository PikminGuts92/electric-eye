use clap::Clap;
use std::error::Error;

mod show_usb;
pub use self::show_usb::*;

// From Cargo.toml
const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) trait SubApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>>;
}

#[derive(Clap, Debug)]
#[clap(name = PKG_NAME, version = VERSION, about = "Use this tool to manage USB devices")]
struct Options {
    #[clap(subcommand)]
    commands: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    #[clap(name = "show", about = "Display connected USB devices")]
    ShowUSB(ShowUSBApp)
}

#[derive(Debug)]
pub struct USBTool {
    options: Options,
}

impl USBTool {
    pub fn new() -> USBTool {
        USBTool {
            options: Options::parse()
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        match &mut self.options.commands {
            SubCommand::ShowUSB(app) => app.process()
        }
    }
}