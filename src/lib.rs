use std::os::raw::{ c_void, c_char };
use std::ffi::CString;
use std::mem;

use chemcore::daylight;

#[no_mangle]
pub extern "C" fn alloc(length: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(length);
    let ptr = buf.as_mut_ptr();

    mem::forget(buf);

    ptr
}

#[no_mangle]
pub extern "C" fn read_smiles(
    p_smiles: *mut c_char
) -> i8 {
    let smiles = unsafe {
        CString::from_raw(p_smiles)
    };

    match smiles.into_string() {
        Ok(smiles) => {
            match daylight::read(&smiles) {
                Ok(_) => 0,
                Err(_) => -1
            }
        },
        Err(_) => -2
    }
}