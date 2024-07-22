use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::HANDLE,
        Security::{
            AdjustTokenPrivileges, LookupPrivilegeValueW, SE_PRIVILEGE_ENABLED, SE_SHUTDOWN_NAME,
            TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES, TOKEN_QUERY,
        },
        System::{
            Shutdown::{ExitWindowsEx, EWX_FORCEIFHUNG, EWX_POWEROFF, SHTDN_REASON_FLAG_PLANNED},
            Threading::{GetCurrentProcess, OpenProcessToken},
        },
    },
};

use super::Error;

pub fn shutdown() -> Result<(), Error> {
    unsafe {
        enable_privilege()?;
        ExitWindowsEx(EWX_POWEROFF | EWX_FORCEIFHUNG, SHTDN_REASON_FLAG_PLANNED)?;
        Ok(())
    }
}

/// Check the privilege is available and enable it
unsafe fn enable_privilege() -> Result<(), Error> {
    let mut tkp = TOKEN_PRIVILEGES::default();
    let mut h_token: HANDLE = HANDLE::default();
    let process_handle = GetCurrentProcess();
    // Get a token for current process
    OpenProcessToken(
        process_handle,
        TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
        &mut h_token as *mut _,
    )?;
    // Get the shutdown privilege id
    LookupPrivilegeValueW(
        PCWSTR::null(),
        SE_SHUTDOWN_NAME,
        &mut tkp.Privileges[0].Luid as *mut _,
    )?;
    // fixup tkp
    tkp.PrivilegeCount = 1;
    tkp.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;

    // Enable shutdown privilege
    AdjustTokenPrivileges(h_token, false, Some(&tkp), 0, None, None)?;
    Ok(())
}

impl From<windows::core::Error> for Error {
    fn from(e: windows::core::Error) -> Self {
        // mask off top of hresult
        match e.code().0 & 0xffff {
            // ERROR_PRIVILEGE_NOT_HELD
            0x0522 => Error::NoPermission,
            _ => Error::Unknown(e.message()),
        }
    }
}
