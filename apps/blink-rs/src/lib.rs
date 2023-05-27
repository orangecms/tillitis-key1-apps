#![no_std]
#![no_main]
#![feature(start)]

use core::{arch::asm, panic::PanicInfo};
use k256::SecretKey;

const TK1_MMIO_BASE: u32 = 0xc0000000;
const TK1_MMIO_TK1_BASE: u32 = TK1_MMIO_BASE | 0x3f000000;
const TK1_MMIO_UART_BASE: u32 = TK1_MMIO_BASE | 0x03000000;

core::arch::global_asm!(include_str!("../../../../tkey-libs/libcrt0/crt0.S"));

#[repr(u32)]
enum Mmio {
    UartBitRate = TK1_MMIO_UART_BASE | 0x40,
    UartDataBits = TK1_MMIO_UART_BASE | 0x44,
    UartStopBits = TK1_MMIO_UART_BASE | 0x48,
    UartRxStatus = TK1_MMIO_UART_BASE | 0x80,
    UartRxData = TK1_MMIO_UART_BASE | 0x84,
    UartRxBytes = TK1_MMIO_UART_BASE | 0x88,
    UartTxStatus = TK1_MMIO_UART_BASE | 0x100,
    UartTxData = TK1_MMIO_UART_BASE | 0x104,

    Led = TK1_MMIO_TK1_BASE | 0x24,
}

fn peek(addr: Mmio) -> u32 {
    unsafe { core::ptr::read_volatile(addr as u32 as *const u32) }
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
        unsafe {
            asm!("nop");
        }
    }
}

const TK1_MMIO_TK1_LED_R_BIT: u32 = 2;
const TK1_MMIO_TK1_LED_G_BIT: u32 = 1;
const TK1_MMIO_TK1_LED_B_BIT: u32 = 0;

fn print_nibble(byte: u8) {
    let b = if byte < 10 { byte + 0x30 } else { byte + 0x37 };
    tx(&[b]);
}

// TODO: implement print! macro for rich formatting
fn print_byte(byte: u8) {
    let nibble0 = byte >> 4;
    let nibble1 = byte & 0xf;
    print_nibble(nibble0);
    print_nibble(nibble1);
}

const SLEEP_TIME: u32 = 100000;

#[no_mangle]
#[start]
pub extern "C" fn main() -> ! {
    let junk = [1; 32];
    tx(b"Secret....\n\r");

    match SecretKey::from_slice(&junk) {
        Ok(key) => {
            for k in key.to_bytes() {
                print_byte(k);
            }
        }
        Err(e) => {
            tx(b"Error\n");
        }
    }

    let key = [0xaa, 0xbb, 0x12, 0x34];

    for k in key {
        print_byte(k);
    }

    tx(b"Hello, world!\n\r");
    loop {
        //poke(Mmio::Led, 1 << TK1_MMIO_TK1_LED_R_BIT);
        //sleep(sleep_time);
        //poke(Mmio::Led, 1 << TK1_MMIO_TK1_LED_G_BIT);
        //sleep(sleep_time);
        //poke(Mmio::Led, 1 << TK1_MMIO_TK1_LED_B_BIT);
        //sleep(sleep_time);
    }
}

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
