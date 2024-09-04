use std::ffi::c_void;
use std::os::raw::c_char;
use std::os::raw::c_int;
use libc::RTLD_NEXT;

#[link(name = "dl")]
extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlerror() -> *const c_char;
}

static mut REAL_MAIN: Option<unsafe extern "C" fn(argc: c_int, argv: *const *const c_char, *const *const c_char) -> c_int> = None;

unsafe extern "C" fn wrapped_main(argc: c_int, argv: *const *const c_char, envp: *const *const c_char) -> c_int
{
    println!("Hello from wrapped_main()!");
    return REAL_MAIN.unwrap()(argc, argv, envp);
}

#[no_mangle]
pub extern "C" fn __libc_start_main(main: extern fn(c_int, *const *const c_char, *const *const c_char) -> c_int , 
    _argc: c_int, 
    _udp_av: *const *const c_char, 
    _init: extern fn(), 
    _fini: extern fn(), 
    _rtld_fini: extern fn(), 
    _stack_end: *const c_void) -> c_int
{
    unsafe {
        let real_libc_start_main: Option<unsafe extern "C" fn(
            main: unsafe extern "C" fn(c_int, *const *const c_char, *const *const c_char) -> c_int, 
            argc: c_int, 
            udp_av: *const *const c_char, 
            init: extern "C" fn(), 
            fini: extern "C" fn(), 
            rtld_fini: extern "C" fn(), 
            stack_end: *const c_void) -> c_int> = Some(std::mem::transmute( dlsym(RTLD_NEXT, b"__libc_start_main\0".as_ptr() as *const c_char))); 

        if real_libc_start_main.is_none() {
            eprintln!("dlsym error: {:?}", std::ffi::CStr::from_ptr(dlerror()));
            std::process::exit(1);
        }

        REAL_MAIN = Some(main);

        return real_libc_start_main.unwrap()(wrapped_main, _argc, _udp_av, _init, _fini, _rtld_fini, _stack_end);
    }
}
