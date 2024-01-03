#![no_main]
#![no_std]
#![feature(panic_info_message)]

mod lang_items;
mod sbi;
#[macro_use]
mod console;
#[macro_use]
mod logging;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();

    extern "C" {
        fn stext();
        fn etext();
    }
    info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);

    extern "C" {
        fn sdata();
        fn edata();
    }
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);

    extern "C" {
        fn srodata();
        fn erodata();
    }
    info!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);

    extern "C" {
        fn sbss();
        fn ebss();
    }
    info!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);

    println!("Hello, world!");
    error!("Hello, world!");
    warn!("Hello, world!");
    info!("Hello, world!");
    debug!("Hello, world!");
    trace!("Hello, world!");
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
