#![no_std]

use core::{panic::PanicInfo, ptr};

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() -> ! {
            // type check the given path
            let f: fn() -> ! = $path;

            f()
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    // Initialize RAM
    extern "C" {
        static mut _sbss: u8;
        static _ebss: u8;

        static mut _sdata: u8;
        static _edata: u8;
        static _sidata: u8;
    }

    let bss_ptr = ptr::addr_of_mut!(_sbss);
    let bss_size = &_ebss as *const u8 as usize - bss_ptr as usize;
    core::ptr::write_bytes(bss_ptr, 0, bss_size);

    let data_ptr = ptr::addr_of_mut!(_sdata);
    let data_size = &_edata as *const u8 as usize - data_ptr as usize;
    core::ptr::copy_nonoverlapping(&_sidata as *const u8, data_ptr, data_size);

    extern "Rust" {
        fn main() -> !;
    }
    main()
}

// The reset vector, a pointer into the reset handler
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
