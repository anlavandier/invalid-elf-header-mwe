#![no_std]

#![allow(warnings)]



#[global_allocator]
static ALLOCATOR: talc::Talck<talc::locking::AssumeUnlockable, talc::ClaimOnOom> = {
    // Static 4KiB Arena
    static mut MEMORY: [u8; 4096] = [0; 4096];
    let span = talc::Span::from_array(core::ptr::addr_of!(MEMORY).cast_mut());
    talc::Talc::new(unsafe { talc::ClaimOnOom::new(span) }).lock()
};

extern crate alloc;
use alloc::string::{String, ToString};

mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        "Hello, World!".to_string()
    }
    fn add_two(x: u32, y: u32) -> u32 {
        x + y
    }
}

bindings::export!(Component with_types_in bindings);



use core::panic::PanicInfo;
#[panic_handler]
pub fn panic_handler(_: &PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}