
#[cfg(windows)] use winapi;

#[derive(Debug)]
pub struct HidManager {

}

#[derive(Debug)]
pub struct HidDevice {
    pub path: String,
    pub product_id: i32,
    pub vendor_id: i32,
    pub product_str: Option<String>,
    pub serial_num_str: Option<String>,
    pub dev_inst: Option<i32>,
    pub pdo_name: Option<String>,
}

impl HidManager {
    pub fn new() -> Self {
        HidManager {

        }
    }

    #[cfg(windows)]
    pub fn get_devices(&self) -> Vec<HidDevice> {
        Vec::new()
    }

    #[cfg(not(windows))]
    pub fn get_devices(&self) -> Vec<HidDevice> {
        // TODO: Return unsupported platform error
        Vec::new()
    }
}