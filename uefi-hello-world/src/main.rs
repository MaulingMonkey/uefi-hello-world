#![allow(non_camel_case_types)]
#![no_main]
#![no_std]

extern crate rlibc; // force linking for mem{cmp,copy,move,set}

use core::cell::Cell;
use core::cell::RefCell;
use core::fmt::Write;

use uefi::prelude::*;
use uefi::CStr16;
use uefi::proto::console::gop::BltOp;
use uefi::proto::console::gop::BltRegion;
use uefi::proto::console::gop::GraphicsOutput;
use wchar::wchz;



struct Global {
    system_table:   RefCell<Option<SystemTable<Boot>>>,
    panicing:       Cell<bool>,
}

unsafe impl Send for Global {} // XXX
unsafe impl Sync for Global {} // XXX

static GLOBAL : Global = Global {
    system_table:   RefCell::new(None),
    panicing:       Cell::new(false),
};

#[no_mangle] pub extern "win64" fn efi_main(_image_handle: Handle, system_table: SystemTable<Boot>) -> Status {
    *GLOBAL.system_table.borrow_mut() = Some(unsafe { system_table.unsafe_clone() });
    system_table.stdout().clear().unwrap().unwrap();
    system_table.stdout().output_string(CStr16::from_u16_with_nul(wchz!("Hello, UEFI!!\r\n")).ok().unwrap()).unwrap().unwrap();
    system_table.stdout().enable_cursor(true).unwrap().unwrap();

    let (w, h) = include!("../logo.png.dims");
    let h = h-1; // workaround https://github.com/rust-osdev/uefi-rs/pull/257
    let bin : &[u8]  = &include_bytes!("../logo.png.bin")[..];
    let bin : &[u32] = bytemuck::try_cast_slice(bin).unwrap();
    let bin = unsafe { core::mem::transmute(bin) };
    
    let go : &mut GraphicsOutput = unsafe { &mut *system_table.boot_services().locate_protocol().unwrap().unwrap().get() };
    let (screen_w, screen_h) = go.current_mode_info().resolution();
    go.blt(BltOp::BufferToVideo { buffer: bin, dest: ((screen_w - w)/2, (screen_h - h)/2), dims: (w, h), src: BltRegion::Full }).unwrap().unwrap();

    loop {}
    // Status::SUCCESS
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    let recursive = GLOBAL.panicing.get();
    GLOBAL.panicing.set(true);
    let system_table = GLOBAL.system_table.borrow();
    if let Some(system_table) = system_table.as_ref() {
        if recursive {
            let _ = write!(system_table.stdout(), "Panic occured:\r\n<recursive panic trying to display panic info>\r\n");
        } else {
            write!(system_table.stdout(), "Panic occured:\r\n{}\r\n", _info).unwrap();
        }
        let _ = system_table.stdout().enable_cursor(false);
    }
    loop {}
}
