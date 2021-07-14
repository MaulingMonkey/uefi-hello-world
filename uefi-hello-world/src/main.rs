#![allow(non_camel_case_types)]
#![no_main]
#![no_std]

mod intrinsics;

use uefi::prelude::*;
use uefi::CStr16;
use wchar::wchz;



#[no_mangle] pub extern "win64" fn efi_main(_image_handle: Handle, system_table: SystemTable<Boot>) -> Status {
    let _ = system_table.stdout().clear().unwrap();
    let _ = system_table.stdout().output_string(CStr16::from_u16_with_nul(wchz!("Hello, UEFI!!")).ok().unwrap()).unwrap();
    let _ = system_table.stdout().enable_cursor(true).unwrap();
    loop {}
    // Status::SUCCESS
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
