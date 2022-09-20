#![no_std]
#![no_main]

#[cfg(any(target_arch = "aarch64"))]
#[link_section=".text._boot"]
#[no_mangle]
fn _boot() -> ! {
     loop {}   
}

#[inline(never)]
#[panic_handler]
fn _panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}