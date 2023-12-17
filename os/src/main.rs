// disable standard library
#![no_std] 
//disable normal entry point
#![no_main] 

// entry point for 

/// outputing function name
#[no_mangle] 
pub extern "C" fn _start() -> ! {
    loop {}
}



//Panic Handler
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
