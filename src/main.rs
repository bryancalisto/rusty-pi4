#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

mod boot {
    use core::arch::global_asm;

    global_asm!(".section .text._start");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        // Turn pin21 into an output
        core::ptr::write_volatile(0xfe20_0008 as *mut u32, 1 << 3);

        loop {
            // Turn on
            core::ptr::write_volatile(0xfe20_001c as *mut u32, 1 << 21);

            for _ in 1..50000 {
                asm!("nop");
            }

            // Turn off
            core::ptr::write_volatile(0xfe20_0028 as *mut u32, 1 << 21);

            for _ in 1..50000 {
                asm!("nop");
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
