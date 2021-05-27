use std::mem::{size_of, zeroed};
use std::ptr::{null, null_mut};
#[cfg(windows)]
use winapi::{
    self,
    shared::minwindef::{BOOL, FALSE},
    shared::guiddef::GUID,
    shared::windef::HWND,
    shared::minwindef::DWORD,
    um::winnt::PCWSTR,
    um::setupapi,
};

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
        unsafe {
            let flags = setupapi::DIGCF_ALLCLASSES | setupapi::DIGCF_PRESENT;
            let dev_info_set: setupapi::HDEVINFO = setupapi::SetupDiGetClassDevsW(null(), null(), null_mut(), flags);

            let mut dev_info_data: setupapi::SP_DEVINFO_DATA = zeroed();
            dev_info_data.cbSize = size_of::<setupapi::SP_DEVINFO_DATA>() as u32;

            let mut dev_interface_data: setupapi::SP_DEVICE_INTERFACE_DATA = zeroed();
            dev_interface_data.cbSize = size_of::<setupapi::SP_DEVICE_INTERFACE_DATA>() as u32;

            let mut dev_idx: u32 = 0;
            loop {
                let setup_info_res = setupapi::SetupDiEnumDeviceInfo(dev_info_set, dev_idx, &mut dev_info_data);
                if setup_info_res == FALSE {
                    break;
                }

                //setupapi::SetupDiGetDevicePropertyW(dev_info_set, &dev_info_data, )
                //setupapi::SetupDiEnumDeviceInfo(dev_info_set, dev_idx, &mut dev_info_data);

                let mut member_idx: u32 = 0;
                loop {
                    let setup_inter_res = setupapi::SetupDiEnumDeviceInterfaces(dev_info_set, &mut dev_info_data, null(), member_idx, &mut dev_interface_data);
                    if setup_inter_res == FALSE {
                        break;
                    }

                    member_idx += 1;
                }

                dev_idx += 1;
            }
        }

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