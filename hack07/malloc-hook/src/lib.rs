use std::ffi::c_void;
use std::os::raw::c_char;
use std::ptr;
use std::sync::Once;
use libc::RTLD_NEXT;

#[link(name = "dl")]
extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlerror() -> *const c_char;
}

static INIT: Once = Once::new();
static mut REAL_MALLOC: Option<unsafe extern "C" fn(size: usize) -> *mut c_void> = None;

unsafe fn initialize() {
    REAL_MALLOC = Some(std::mem::transmute(dlsym(RTLD_NEXT, b"malloc\0".as_ptr() as *const c_char)));
    if REAL_MALLOC.is_none() {
        eprintln!("dlsym error: {:?}", std::ffi::CStr::from_ptr(dlerror()));
        std::process::exit(1);
    }
}

#[no_mangle]
pub extern "C" fn malloc(size: usize) -> *mut c_void {
    unsafe {
        INIT.call_once(|| {
            initialize();
        });

        eprintln!("malloc: size={}", size);

        if let Some(real_malloc) = REAL_MALLOC {
            real_malloc(size)
        } else {
            ptr::null_mut()
        }
    }
}
