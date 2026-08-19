use core::arch::asm;

// Reading ports using asm
#[inline(always)]
pub unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    asm!("in al, dx", out("al") result, in("dx") port, options(nomem, nostack));
    result
}
