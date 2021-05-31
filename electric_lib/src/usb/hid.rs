use std::convert::TryInto;
use std::io::Read;
use std::mem::{size_of, zeroed};
use std::ptr::{null, null_mut};
#[cfg(windows)]
use winapi::{
    self,
    shared::devpkey::{
        DEVPKEY_Device_FriendlyName,
        DEVPKEY_Device_LocationInfo,
        DEVPKEY_Device_PDOName,
    },
    shared::devpropdef::{DEVPROPKEY, DEVPROPTYPE},
    shared::minwindef::{BOOL, BYTE, DWORD, FALSE},
    shared::guiddef::GUID,
    shared::usbiodef::GUID_DEVINTERFACE_USB_DEVICE,
    shared::windef::HWND,
    um::{
        cfgmgr32::{
            CM_Get_Device_IDW,
            MAX_DEVICE_ID_LEN,
        },
        setupapi,
        winnt::WCHAR,
    }
};

#[derive(Debug)]
pub struct HidManager {
    #[cfg(windows)]
    dev_info_set: setupapi::HDEVINFO,
}

#[derive(Debug, Default)]
pub struct HidDevice {
    pub path: String,
    pub product_id: u32,
    pub vendor_id: u32,
    pub product_str: String,
    pub serial_num_str: String,
    pub dev_inst: Option<u32>,
    pub pdo_name: String,
}

impl HidManager {
    pub fn new() -> Self {
        unsafe {
            // Init device info setup
            let flags = setupapi::DIGCF_PRESENT | setupapi::DIGCF_DEVICEINTERFACE;
            let dev_info_set: setupapi::HDEVINFO = setupapi::SetupDiGetClassDevsW(&GUID_DEVINTERFACE_USB_DEVICE, null(), null_mut(), flags);

            HidManager {
                dev_info_set,
            }
        }
    }

    #[cfg(windows)]
    pub fn get_usb_devices(&self) -> Vec<HidDevice> {
        let mut buffer: [BYTE; 4096] = [0; 4096];
        let mut instance_id_buffer: [WCHAR; MAX_DEVICE_ID_LEN] = [0; MAX_DEVICE_ID_LEN];
        let mut devices = Vec::new();

        unsafe {
            let mut dev_info_data: setupapi::SP_DEVINFO_DATA = zeroed();
            dev_info_data.cbSize = size_of::<setupapi::SP_DEVINFO_DATA>() as u32;

            //let mut dev_interface_data: setupapi::SP_DEVICE_INTERFACE_DATA = zeroed();
            //dev_interface_data.cbSize = size_of::<setupapi::SP_DEVICE_INTERFACE_DATA>() as u32;

            let mut dev_idx: u32 = 0;
            loop {
                let setup_info_res = setupapi::SetupDiEnumDeviceInfo(self.dev_info_set, dev_idx, &mut dev_info_data);
                if setup_info_res == FALSE {
                    break;
                }

                // Get device properties
                let friendly_name = self.get_device_property_string(&DEVPKEY_Device_FriendlyName, &mut dev_info_data, &mut buffer);
                let loc_info = self.get_device_property_string(&DEVPKEY_Device_LocationInfo, &mut dev_info_data, &mut buffer);
                let pdo_name = self.get_device_property_string(&DEVPKEY_Device_PDOName, &mut dev_info_data, &mut buffer);

                CM_Get_Device_IDW(dev_info_data.DevInst, &mut instance_id_buffer as *mut WCHAR, instance_id_buffer.len() as u32, 0);
                let dev_id = String::from_utf16(&instance_id_buffer).unwrap_or_default();
                let (pid, vid, serial) = get_usb_details(&dev_id);

                /*let mut member_idx: u32 = 0;
                loop {
                    let setup_inter_res = setupapi::SetupDiEnumDeviceInterfaces(self.dev_info_set, &mut dev_info_data, null(), member_idx, &mut dev_interface_data);
                    if setup_inter_res == FALSE {
                        break;
                    }

                    member_idx += 1;
                }*/

                dev_idx += 1;

                devices.push(HidDevice {
                    //path: String,
                    product_id: match pid {
                        Some(pid) => u32::from_str_radix(pid, 16)
                            .unwrap_or_default(),
                        None => 0,
                    },
                    vendor_id: match vid {
                        Some(vid) => u32::from_str_radix(vid, 16)
                            .unwrap_or_default(),
                        None => 0,
                    },
                    product_str: friendly_name,
                    serial_num_str: match serial {
                        Some(s) => s.to_owned(),
                        None => String::new(),
                    },
                    //dev_inst: Option<i32>,
                    pdo_name,
                    ..Default::default()
                });
            }
        }

        devices
    }

    #[cfg(windows)]
    fn get_device_property_string(&self, prop_key: &DEVPROPKEY, dev_info_data: &mut setupapi::SP_DEVINFO_DATA, buffer: &mut [BYTE; 4096]) -> String {
        let mut prop_type: DEVPROPTYPE = 0;
        let mut req_size: DWORD = 0;

        unsafe {
            let setup_prop_res = setupapi::SetupDiGetDevicePropertyW(self.dev_info_set, dev_info_data, prop_key, &mut prop_type, buffer as *mut BYTE, buffer.len() as u32, &mut req_size, 0);
            // TODO: Check error from setup_prop_res

            // Get device friendly name
            if req_size == 0 {
                return String::new();
            } else {
                // Convert u8 slice to u16 vector
                let str_vec = &buffer[..(req_size as usize)]
                    .chunks(2)
                    .map(|c| u16::from_le_bytes(c.try_into().unwrap())) // le_bytes is system specific!
                    .collect::<Vec<u16>>();

                return String::from_utf16(str_vec).unwrap_or_default();
            }
        }
    }

    #[cfg(not(windows))]
    pub fn get_usb_devices(&self) -> Vec<HidDevice> {
        // TODO: Return unsupported platform error
        Vec::new()
    }
}

fn get_usb_details(dev_id: &str) -> (Option<&str>, Option<&str>, Option<&str>) { // pid, vid, serial #
    let mut pid = None;
    let mut vid = None;
    let mut serial = None;

    let split = dev_id.split("\\").collect::<Vec<_>>();

    // Get pid/vid
    if let Some(ids) = split.get(1) {
        for id in ids.split("&") {
            match &id[..4] {
                "PID_" => pid = Some(&id[4..]),
                "VID_" => vid = Some(&id[4..]),
                _ => continue,
            }
        }
    }

    // Get serial #
    if let Some(s) = split.get(2) {
        serial = Some(*s);
    }

    (pid, vid, serial)
}