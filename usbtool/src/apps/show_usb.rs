use crate::apps::SubApp;
use clap::Clap;
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
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
        let devices = man.get_usb_devices();

        print_devices(&devices);
        println!("Complete!");
        Ok(())
    }
}

fn print_devices(devices: &Vec<HidDevice>) {
    let table = devices
        .iter()
        .map(|d| {
            vec![
                d.path.to_owned().cell(),
                d.product_id.cell(),
                d.vendor_id.cell(),
                d.dev_class.to_owned().cell(),
                d.dev_man.to_owned().cell(),
                d.product_str.to_owned().cell(),
                d.serial_num_str.to_owned().cell(),
                d.dev_inst.unwrap_or_default().cell(),
                d.pdo_name.to_owned().cell(),
            ]
        })
        //.collect::<Vec<_>>()
        .table()
        .title(vec![
            "Path".cell().bold(true),
            "PID".cell().bold(true),
            "VID".cell().bold(true),
            "Class".cell().bold(true),
            "Manufacturer".cell().bold(true),
            "Product".cell().bold(true),
            "Ser. #".cell().bold(true),
            "Dev. Inst.".cell().bold(true),
            "PDO Name".cell().bold(true),
        ])
        .bold(true);

    print_stdout(table).unwrap();
}