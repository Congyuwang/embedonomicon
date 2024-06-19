#![no_std]
#![no_main]

rt::entry!(main);

fn main() -> ! {
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn HardFault(_ef: *const u32) -> ! {
    loop {}
}
