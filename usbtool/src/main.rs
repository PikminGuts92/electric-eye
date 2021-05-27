mod apps;
use apps::USBTool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut usb_tool = USBTool::new();
    usb_tool.run()
}