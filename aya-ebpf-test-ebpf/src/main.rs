#![no_std]
#![no_main]

use aya_ebpf::{macros::tracepoint, programs::TracePointContext};
use aya_log_ebpf::info;

#[tracepoint]
pub fn aya_ebpf_test(ctx: TracePointContext) -> u32 {
    match try_aya_ebpf_test(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_aya_ebpf_test(ctx: TracePointContext) -> Result<u32, u32> {
    info!(&ctx, "tracepoint syscalls called");
    Ok(0)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[link_section = "license"]
#[no_mangle]
static LICENSE: [u8; 13] = *b"Dual MIT/GPL\0";
