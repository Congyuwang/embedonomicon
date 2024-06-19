#![no_main]
#![no_std]

use cortex_m_semihosting::{
    debug,
    hio::{self, HostStream},
};

use log::{error, warn, Log};
use rt::entry;

struct Logger {
    hstdout: HostStream,
}

impl Log for Logger {
    type Error = ();

    fn log(&mut self, address: u8) -> Result<(), ()> {
        self.hstdout.write_all(&[address])
    }
}

entry!(main);

fn main() -> ! {
    let hstdout = hio::hstdout().unwrap();
    let mut logger = Logger { hstdout };

    let _ = warn!(logger, "Hello, world!");

    let _ = error!(logger, "Goodbye");

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
