#![cfg(windows)]

use gdiplus::enums::Status;

#[test]
fn test_status_enum() {
    assert_eq!(Status::Ok as u32, winapi::um::gdiplustypes::Ok);
    assert_eq!(Status::Win32Error as u32, winapi::um::gdiplustypes::Win32Error);
    assert_eq!(Status::PropertyNotFound as u32, winapi::um::gdiplustypes::PropertyNotFound);
}
