use std::ffi::{CStr, CString};

use crate::error::RtMidiError;
use crate::ffi;
use crate::RtMidiPort;

pub fn open_port<T: AsRef<str>>(
    ptr: *mut ffi::RtMidiWrapper,
    port_number: RtMidiPort,
    port_name: T,
) -> Result<(), RtMidiError> {
    let port_name = CString::new(port_name.as_ref())?;
    unsafe {
        ffi::rtmidi_open_port(ptr, port_number, port_name.as_ptr());
        (*ptr).into()
    }
}

pub fn open_virtual_port<T: AsRef<str>>(
    ptr: *mut ffi::RtMidiWrapper,
    port_name: T,
) -> Result<(), RtMidiError> {
    let port_name = CString::new(port_name.as_ref())?;
    unsafe {
        ffi::rtmidi_open_virtual_port(ptr, port_name.as_ptr());
        (*ptr).into()
    }
}

pub fn close_port(ptr: *mut ffi::RtMidiWrapper) -> Result<(), RtMidiError> {
    unsafe {
        ffi::rtmidi_close_port(ptr);
        (*ptr).into()
    }
}

pub fn port_count(ptr: *mut ffi::RtMidiWrapper) -> Result<RtMidiPort, RtMidiError> {
    let port_count = unsafe { ffi::rtmidi_get_port_count(ptr) };
    match unsafe { Result::<(), RtMidiError>::from(*ptr) } {
        Ok(_) => Ok(port_count),
        Err(e) => Err(e),
    }
}

pub fn port_name<'a>(
    ptr: *mut ffi::RtMidiWrapper,
    port_number: RtMidiPort,
) -> Result<String, RtMidiError> {
    let mut buffer_len: i32 = 0;
    let size_res = unsafe {
        ffi::rtmidi_get_port_name(ptr, port_number, std::ptr::null_mut(), &mut buffer_len)
    };

    if size_res != 0 || buffer_len <= 0 {
        return Err(RtMidiError::Error(
            "Error getting port name length".to_string(),
        ));
    }

    let mut buffer = vec![0u8; buffer_len as usize];

    let res = unsafe {
        ffi::rtmidi_get_port_name(
            ptr,
            port_number,
            buffer.as_mut_ptr() as *mut i8,
            &mut buffer_len,
        )
    };

    if res != buffer_len - 1 {
        return Err(RtMidiError::Error(format!(
            "Error getting port name: {}",
            res
        )));
    }

    let c_str = unsafe { CStr::from_bytes_with_nul_unchecked(&buffer) };
    match c_str.to_str() {
        Ok(s) => Ok(s.to_owned()),
        Err(e) => Err(RtMidiError::Utf8(e)),
    }
}
