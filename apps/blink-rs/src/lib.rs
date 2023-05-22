#![no_std]

use core::{
    panic::PanicInfo,
    arch::asm,
};

#[repr(u32)]
enum Mmio {
    //UartBitRate = 0xc300_0000 | 0x40,
    //UartDataBits = 0xc300_0000 | 0x44,
    //UartStopBits = 0xc300_0000 | 0x48,
    //UartRxStatus = 0xc300_0000 | 0x80,
    //UartRxData = 0xc300_0000 | 0x84,
    //UartRxBytes  = 0xc300_0000 | 0x88,
    UartTxStatus = 0xc300_0000 | 0x100,
    UartTxData = 0xc300_0000 | 0x104,

    Led = 0xc300_0000 | 0x24,
}

fn peek(addr: Mmio) -> u32 {
    unsafe {
        core::ptr::read_volatile(addr as u32 as *const u32)
    }
}

fn poke(addr: Mmio, data: u32) {
    unsafe {
        core::ptr::write_volatile(addr as u32 as *mut u32, data);
    }
}

fn tx(data: &[u8]) {
    for byte in data {
        while peek(Mmio::UartTxStatus) == 0 {}
        poke(Mmio::UartTxData, *byte as u32);
    }
}

fn sleep(cycles: usize) {
    for _ in 0..cycles {
        unsafe { asm!("nop"); }
    }
}

const TK1_MMIO_TK1_LED_R_BIT: u32    = 2;
const   TK1_MMIO_TK1_LED_G_BIT: u32    = 1;
const   TK1_MMIO_TK1_LED_B_BIT: u32    = 0;
   
#[no_mangle]
extern "C" fn main() -> ! {
    let sleep_time = 100000;
    loop {
        poke(Mmio::Led, 1 << TK1_MMIO_TK1_LED_R_BIT);
        sleep(sleep_time);
        poke(Mmio::Led, 1 << TK1_MMIO_TK1_LED_G_BIT);
        sleep(sleep_time);
        poke(Mmio::Led, 1 << TK1_MMIO_TK1_LED_B_BIT);
        sleep(sleep_time);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
