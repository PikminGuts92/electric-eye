use std::convert::TryInto;
use std::io::Read;
use std::mem::{size_of, zeroed};
use std::ptr::{null, null_mut};
#[cfg(windows)]
use winapi::{
    self,
    shared::devpkey::DEVPKEY_Device_FriendlyName,
    shared::devpropdef::DEVPROPTYPE,
    shared::minwindef::{BOOL, BYTE, DWORD, FALSE},
    shared::guiddef::GUID,
    shared::usbiodef::GUID_DEVINTERFACE_USB_DEVICE,
    shared::windef::HWND,
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
            let flags = setupapi::DIGCF_PRESENT | setupapi::DIGCF_DEVICEINTERFACE;
            let dev_info_set: setupapi::HDEVINFO = setupapi::SetupDiGetClassDevsW(&GUID_DEVINTERFACE_USB_DEVICE, null(), null_mut(), flags);

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

                // Get device properties
                let mut prop_type: DEVPROPTYPE = zeroed();
                let mut buffer: [BYTE; 4096] = zeroed();
                let mut req_size: DWORD = 0;
                let setup_prop_res = setupapi::SetupDiGetDevicePropertyW(dev_info_set, &mut dev_info_data, &DEVPKEY_Device_FriendlyName, &mut prop_type, &mut buffer as *mut BYTE, buffer.len() as u32, &mut req_size, 0);

                //setupapi::SetupDiGetDevicePropertyW(dev_info_set, &dev_info_data, )
                //setupapi::SetupDiEnumDeviceInfo(dev_info_set, dev_idx, &mut dev_info_data);

                let name;

                if req_size == 0 {
                    name = String::new();
                } else {
                    let str_vec = &buffer[..(req_size as usize)]
                        .chunks(2)
                        .map(|c| u16::from_le_bytes(c.try_into().unwrap())) // le_bytes is system specific!
                        .collect::<Vec<u16>>();

                    name = String::from_utf16(str_vec).unwrap_or_default();
                }

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