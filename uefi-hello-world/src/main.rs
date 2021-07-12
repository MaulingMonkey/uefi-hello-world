#![allow(non_camel_case_types)]
//#![allow(dead_code, unused_variables)]
#![no_main]
#![no_std]

use core::ffi::c_void;
use wchar::wchz;



// https://uefi.org/sites/default/files/resources/UEFI%20Spec%202_6.pdf page 23
// 2.3.1 Data Types

pub type BOOLEAN = u8;
pub type INTN  = isize;
pub type UINTN = usize;
pub type CHAR8 = u8;
pub type CHAR16 = u16;
pub type EFI_GUID = u128;
pub type EFI_STATUS = UINTN;
pub type EFI_HANDLE = usize; // pointer
pub type EFI_EVENT = usize; // pointer
pub type EFI_LBA = u64;
pub type EFI_MAC_ADDRESS = [u8; 32];
pub type EFI_IPv4_ADDRESS = [u8; 4];
pub type EFI_IPv6_ADDRESS = [u8; 16];
pub type EFI_IP_ADDRESS = [u8; 16];

#[allow(dead_code)] const EFI_SUCCESS : EFI_STATUS = 0;




// https://uefi.org/sites/default/files/resources/UEFI%20Spec%202_6.pdf page 95
// 4.2 EFI Table Header
#[repr(C)] pub struct EFI_TABLE_HEADER {
    pub signature:      u64,
    pub revision:       u32,
    pub header_size:    u32,
    pub crc32:          u32,
    pub reserved:       u32,
}

// https://uefi.org/sites/default/files/resources/UEFI%20Spec%202_6.pdf page 96
// 4.3 EFI System Table
#[repr(C)] pub struct EFI_SYSTEM_TABLE {
    pub hdr:                        EFI_TABLE_HEADER,
    pub firmware_vendor:            *const u16,
    pub firmware_revision:          u32,
    pub console_in_handle:          EFI_HANDLE,
    pub conin:                      *const EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    pub console_out_handle:         EFI_HANDLE,
    pub conout:                     *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    pub standard_error_handle:      EFI_HANDLE,
    pub stderr:                     *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    pub runtime_services:           *const EFI_RUNTIME_SERVICES,
    pub boot_services:              *const EFI_BOOT_SERVICES,
    pub number_of_table_entries:    usize, // uintn
    pub configuration_table:        *const EFI_CONFIGURATION_TABLE,
}

// https://uefi.org/sites/default/files/resources/UEFI%20Spec%202_6.pdf page 104
// 4.6 EFI Configuration Table & Properties Table
#[repr(C)] pub struct EFI_CONFIGURATION_TABLE {
    pub vendor_guid:    EFI_GUID,
    pub vendor_table:   *const c_void,
}

// Misc.
#[repr(C)] pub struct EFI_RUNTIME_SERVICES  { _non_exhaustive: () }
#[repr(C)] pub struct EFI_BOOT_SERVICES     { _non_exhaustive: () }



// https://uefi.org/sites/default/files/resources/UEFI%20Spec%202_6.pdf page 467
// 11.3 Simple Text Input Protocol

#[allow(dead_code)] const EFI_SIMPLE_TEXT_INPUT_PROTOCOL_GUID : EFI_GUID = guid(0x387477c1,0x69c7,0x11d2, [0x8e,0x39,0x00,0xa0,0xc9,0x69,0x72,0x3b]);

#[repr(C)] pub struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL {
    pub reset:                  EFI_INPUT_RESET,
    pub read_key_stroke:        EFI_INPUT_READ_KEY,
    pub wait_for_key:           EFI_EVENT,
}

type EFI_INPUT_RESET                = extern "win64" fn(this: *const EFI_SIMPLE_TEXT_INPUT_PROTOCOL, extended_verification: BOOLEAN) -> EFI_STATUS;
type EFI_INPUT_READ_KEY             = extern "win64" fn(this: *const EFI_SIMPLE_TEXT_INPUT_PROTOCOL, key: &mut EFI_INPUT_KEY) -> EFI_STATUS;

#[repr(C)] pub struct EFI_INPUT_KEY {
    pub scan_code:      u16,
    pub unicode_char:   CHAR16,
}



// https://uefi.org/sites/default/files/resources/UEFI%20Spec%202_6.pdf page 470
// 11.4 Simple Text Output Protocol

#[allow(dead_code)] const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID : EFI_GUID = guid(0x387477c2,0x69c7,0x11d2,[0x8e,0x39,0x00,0xa0,0xc9,0x69,0x72,0x3b]);

#[repr(C)] pub struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    pub reset:                  EFI_TEXT_RESET,
    pub output_string:          EFI_TEXT_STRING,
    pub test_string:            EFI_TEXT_TEST_STRING,
    pub query_mode:             EFI_TEXT_QUERY_MODE,
    pub set_mode:               EFI_TEXT_SET_MODE,
    pub set_attribute:          EFI_TEXT_SET_ATTRIBUTE,
    pub clear_screen:           EFI_TEXT_CLEAR_SCREEN,
    pub set_cursor_position:    EFI_TEXT_SET_CURSOR_POSITION,
    pub enable_cursor:          EFI_TEXT_ENABLE_CURSOR,
    pub mode:                   *mut SIMPLE_TEXT_OUTPUT_MODE,
}

#[repr(C)] pub struct SIMPLE_TEXT_OUTPUT_MODE {
    pub max_mode:               i32,
    pub mode:                   i32,
    pub attribute:              i32,
    pub cursor_column:          i32,
    pub cursor_row:             i32,
    pub cursor_visible:         BOOLEAN,
}

type EFI_TEXT_RESET                 = extern "win64" fn(this: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, extended_verification: BOOLEAN) -> EFI_STATUS;
type EFI_TEXT_STRING                = extern "win64" fn(this: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, string: *const CHAR16) -> EFI_STATUS;
type EFI_TEXT_TEST_STRING           = extern "win64" fn(this: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, string: *const CHAR16) -> EFI_STATUS;
type EFI_TEXT_QUERY_MODE            = extern "win64" fn(this: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, mode_number: UINTN, columns: &mut UINTN, rows: &mut UINTN) -> EFI_STATUS;
type EFI_TEXT_SET_MODE              = extern "win64" fn(this: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, mode_number: UINTN) -> EFI_STATUS;
type EFI_TEXT_SET_ATTRIBUTE         = extern "win64" fn(this: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, attribute: UINTN) -> EFI_STATUS;
type EFI_TEXT_CLEAR_SCREEN          = extern "win64" fn(this: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL) -> EFI_STATUS;
type EFI_TEXT_SET_CURSOR_POSITION   = extern "win64" fn(this: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, column: UINTN, row: UINTN) -> EFI_STATUS;
type EFI_TEXT_ENABLE_CURSOR         = extern "win64" fn(this: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, visible: BOOLEAN) -> EFI_STATUS;



#[no_mangle] pub extern "win64" fn efi_main(_image_handle: EFI_HANDLE, system_table: &EFI_SYSTEM_TABLE) -> EFI_STATUS {
    let _status = unsafe { ((*system_table.conout).clear_screen)(system_table.conout) };
    let _status = unsafe { ((*system_table.conout).output_string)(system_table.conout, wchz!("Hello, UEFI!\r\n").as_ptr()) };
    let _status = unsafe { ((*system_table.conout).enable_cursor)(system_table.conout, 1) };
    loop {}
    // EFI_SUCCESS
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[allow(dead_code)] const fn guid(a: u32, b: u16, c: u16, d: [u8; 8]) -> u128 {
    ((a as u128) << 96) |
    ((b as u128) << 68) |
    ((c as u128) << 64) |
    (u64::from_be_bytes(d) as u128) << 0
}
