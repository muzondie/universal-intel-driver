use windows_sys::Win32::System::Registry;

pub fn is_admin() -> bool {
    unsafe {
        let mut token = 0;
        let mut status = 0;
        
        if windows_sys::Win32::Security::OpenProcessToken(
            windows_sys::Win32::System::Threading::GetCurrentProcess(),
            windows_sys::Win32::Security::TOKEN_QUERY,
            &mut token,
        ) == 0 {
            return false;
        }
        
        let mut elevation = std::mem::zeroed();
        let size = std::mem::size_of::<u32>() as u32;
        
        windows_sys::Win32::Security::GetTokenInformation(
            token,
            windows_sys::Win32::Security::TokenElevation,
            &mut elevation as *mut _ as *mut _,
            size,
            &mut size,
        );
        
        elevation != 0
    }
}