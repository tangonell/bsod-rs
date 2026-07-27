#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

const ERROR_CODE: u32 = 0xDEADDEAD;

#[link(name = "ntdll")]
unsafe extern "system" {
    fn RtlAdjustPrivilege(
        privilege: u32,
        enable: bool,
        current_thread: bool,
        enabled: *mut bool
    ) -> i32;

    fn NtRaiseHardError(
        error_status: u32,
        number_of_parameters: u32,
        unicode_string_parameter_mask: u32,
        parameters: *mut usize,
        valid_response_option: u32,
        response: *mut u32
    ) -> i32;
}

#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn bsod() {
    RtlAdjustPrivilege(19, true, false, &mut false);
    NtRaiseHardError(ERROR_CODE, 0, 0, core::ptr::null_mut(), 6, &mut 0);
}

#[unsafe(no_mangle)]
fn main() {
    unsafe {
        bsod();
    }
}