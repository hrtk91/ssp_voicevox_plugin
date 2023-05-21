use std::{fs::OpenOptions, io::Write};

use rvvc::{InitializeOptions, VoiceVoxCore};
use winapi::{
    ctypes::c_void,
    um::winbase::{GlobalAlloc, GlobalFree, GMEM_FIXED},
};

mod audio_pool;
mod entrypoint;
mod sstp_parser;
mod sstp_request;
mod sstp_response;

use entrypoint::main;

pub fn write_log(msg: &[u8]) {
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("./out.log")
        .expect("failed to create file");
    file.write_all(msg).expect("failed to write file");
}

#[allow(unused_variables)]
#[no_mangle]
/// #Safety
pub unsafe extern "C" fn load(h: *mut c_void, len: i32) -> bool {
    write_log(b"load...");

    unsafe { GlobalFree(h) };

    if let Err(e) = VoiceVoxCore::initialize(InitializeOptions::default()) {
        write_log(format!("failed to initialize core: {}\r\n", e).as_bytes());
        write_log(b"done\r\n");
        return false;
    };

    VoiceVoxCore::load_model(3).expect("failed load model");

    if let Err(e) = audio_pool::initialize(4) {
        write_log(format!("failed to init audio pool: {}\r\n", e).as_bytes());
    };

    write_log(b"done\r\n");
    true
}

#[no_mangle]
pub extern "C" fn unload() -> bool {
    write_log(b"unload\r\n");

    VoiceVoxCore::finalize();
    audio_pool::finalize();

    false
}

#[no_mangle]
/// #Safety
pub unsafe extern "C" fn request(h: *mut c_void, len: *mut i32) -> *mut c_void {
    let req_str = unsafe {
        let slice = std::slice::from_raw_parts(h as *const u8, *len as usize);
        let req_str = String::from_utf8(slice.to_vec()).unwrap();

        GlobalFree(h);

        req_str
    };

    let ret = main(&req_str);

    unsafe {
        let ptr = GlobalAlloc(GMEM_FIXED, ret.len()) as *mut u8;
        if ptr.is_null() {
            return ptr as *mut c_void;
        }

        *len = ret.len() as i32;
        std::ptr::copy(ret.as_bytes().as_ptr() as *mut u8, ptr, ret.len());

        ptr as *mut c_void
    }
}
