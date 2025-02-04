use sysinfo::{System, SystemExt};
use windows_sys::Win32::System::Registry;

pub struct HardwareScanner;

impl HardwareScanner {
    pub fn detect_intel_hardware() -> Vec<HardwareComponent> {
        let mut components = Vec::new();
        let sys = System::new_all();
        
        for device in sys.devices() {
            if let Some(hwid) = device.hardware_id() {
                if hwid.contains("INTEL") || hwid.contains("8086") {
                    components.push(HardwareComponent {
                        hwid: hwid.to_string(),
                        name: device.name().to_string(),
                        current_driver: Self::get_registry_driver(hwid),
                    });
                }
            }
        }
        
        components
    }

    fn get_registry_driver(hwid: &str) -> Option<String> {
        unsafe {
            let mut key = 0;
            let subkey = widestring::U16CString::from_str("SYSTEM\\CurrentControlSet\\Enum\\").unwrap();
            
            if Registry::RegOpenKeyExW(
                Registry::HKEY_LOCAL_MACHINE,
                subkey.as_ptr(),
                0,
                Registry::KEY_READ,
                &mut key,
            ) == 0 {
                
            }
            
            None
        }
    }
}

pub struct HardwareComponent {
    pub hwid: String,
    pub name: String,
    pub current_driver: Option<String>,
}