mod apps;
use apps::USBTool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scene = USBTool::new();
    scene.run()
}