/// Shows an error to the user. Release builds have no console, so a message box
/// is the only way to tell a non-technical user that something went wrong.
#[cfg(windows)]
pub fn error(message: &str) {
    use windows_sys::Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MB_OK, MessageBoxW};

    let text: Vec<u16> = message.encode_utf16().chain(Some(0)).collect();
    let caption: Vec<u16> = "Akoyo".encode_utf16().chain(Some(0)).collect();
    // SAFETY: both strings are valid, NUL-terminated UTF-16 that outlive the call.
    unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            text.as_ptr(),
            caption.as_ptr(),
            MB_OK | MB_ICONERROR,
        );
    }
}

#[cfg(not(windows))]
pub fn error(message: &str) {
    eprintln!("{message}");
}
