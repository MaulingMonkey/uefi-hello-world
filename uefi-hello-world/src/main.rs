#![feature(abi_efiapi)]
#![feature(llvm_asm)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![no_main]
#![no_std]

use core::fmt::Write;
use core::marker::PhantomData;

use uefi::prelude::*;
use uefi::CStr16;
use uefi::proto::console::gop::BltOp;
use uefi::proto::console::gop::BltRegion;
use uefi::proto::console::gop::GraphicsOutput;
use wchar::wchz;



#[entry] fn efi_main(_image_handle: Handle, system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&system_table).unwrap().unwrap();

    system_table.stdout().clear().unwrap().unwrap();
    system_table.stdout().output_string(CStr16::from_u16_with_nul(wchz!("Hello, UEFI!!\r\n")).ok().unwrap()).unwrap().unwrap();
    system_table.stdout().enable_cursor(true).unwrap().unwrap();

    let mut pci = unsafe { Pci::new() };

    let _ = writeln!(system_table.stdout());
    let _ = writeln!(system_table.stdout(), "PCI");
    let _ = writeln!(system_table.stdout(), "Bus   Slot  Vendor  Device  Description");
    for bus in 0 ..= 255u8 {
        for slot in 0 ..= 31u8 {
            if let Some((vendor, device)) = pci.get_vendor_device(bus, slot) {
                // https://www.pcilookup.com/
                // https://github.com/qemu/qemu/blob/a97fca4ceb9d9b10aa8b582e817a5ee6c42ffbaf/include/hw/pci/pci.h#L32
                let desc = match (vendor, device) {
                    (0x1014, 0x027F)    => "IBM 440GX",
                    (0x1014, 0xFFFF)    => "IBM OpenPic2",
                    (0x1014, _other)    => "IBM ???",

                    (0x1054, 0x350E)    => "Hitachi SH7751R",
                    (0x1054, _other)    => "Hitachi ???",

                    (0x106B, _other)    => "Apple ???",
                    (0x10EC, _other)    => "Realtek ???",
                    (0x10EE, _other)    => "Xilinx ???",
                    (0x11AB, _other)    => "Marvell ???",

                    (0x1234, 0x1111)    => "QEMU/Bochs VGA",
                    (0x1234, 0x1112)    => "QEMU/Bochs IPMI",
                    (0x1234, _other)    => "QEMU/Bochs ???",

                    (0x15AD, 0x0405)    => "VMWare SVGA2",
                    (0x15AD, 0x0710)    => "VMWare SVGA",
                    (0x15AD, 0x0720)    => "VMWare Net",
                    (0x15AD, 0x0730)    => "VMWare SCSI",
                    (0x15AD, 0x07B0)    => "VMWare VMXNET3",
                    (0x15AD, 0x07C0)    => "VMWare PVSCSI",
                    (0x15AD, 0x1729)    => "VMWare IDE",
                    (0x15AD, _other)    => "VMWare ???",

                    (0x1AF4, 0x1000)    => "Red Hat / Qumranet VirtIO Net",
                    (0x1AF4, 0x1001)    => "Red Hat / Qumranet VirtIO Block",
                    (0x1AF4, 0x1002)    => "Red Hat / Qumranet VirtIO Balloon",
                    (0x1AF4, 0x1003)    => "Red Hat / Qumranet VirtIO Console",
                    (0x1AF4, 0x1004)    => "Red Hat / Qumranet VirtIO SCSI",
                    (0x1AF4, 0x1005)    => "Red Hat / Qumranet VirtIO RNG",
                    (0x1AF4, 0x1009)    => "Red Hat / Qumranet VirtIO 9P",
                    (0x1AF4, 0x1012)    => "Red Hat / Qumranet VirtIO VSock",
                    (0x1AF4, 0x1013)    => "Red Hat / Qumranet VirtIO PMem",
                    (0x1AF4, 0x1014)    => "Red Hat / Qumranet VirtIO IOMMU",
                    (0x1AF4, 0x1015)    => "Red Hat / Qumranet VirtIO Mem",
                    (0x1AF4, _other)    => "Red Hat / Qumranet ???",

                    (0x1B36, 0x0001)    => "Red Hat Bridge",
                    (0x1B36, 0x0002)    => "Red Hat Serial",
                    (0x1B36, 0x0003)    => "Red Hat Serial 2",
                    (0x1B36, 0x0004)    => "Red Hat Serial 4",
                    (0x1B36, 0x0005)    => "Red Hat Test",
                    (0x1B36, 0x0006)    => "Red Hat Rocker",
                    (0x1B36, 0x0007)    => "Red Hat SDHCI",
                    (0x1B36, 0x0008)    => "Red Hat PCIE Host",
                    (0x1B36, 0x0009)    => "Red Hat PXB",
                    (0x1B36, 0x000A)    => "Red Hat Bridge Seat",
                    (0x1B36, 0x000B)    => "Red Hat PXB PCIE",
                    (0x1B36, 0x000C)    => "Red Hat PCIE RP",
                    (0x1B36, 0x000D)    => "Red Hat XHCI",
                    (0x1B36, 0x000E)    => "Red Hat PCIE Bridge",
                    (0x1B36, 0x000F)    => "Red Hat MDPY",
                    (0x1B36, 0x0010)    => "Red Hat NVME",
                    (0x1B36, 0x0011)    => "Red Hat PVPANIC",
                    (0x1B36, 0x0100)    => "Red Hat QXL",
                    (0x1B36, _other)    => "Red Hat ???",

                    (0x8086, 0x100E)    => "Intel 82540EM Gigabit Ethernet Controller",
                    (0x8086, 0x1209)    => "Intel 82551IT",
                    (0x8086, 0x1229)    => "Intel 82557",
                    (0x8086, 0x1237)    => "Intel 440FX - 82441FX PMC [Natoma]",
                    (0x8086, 0x2922)    => "Intel 82801IR",
                    (0x8086, 0x7000)    => "Intel 82371SB PIIX3 ISA [Natoma/Triton II]",
                    (0x8086, _other)    => "Intel ???",

                    _                   => "???",
                };
                let _ = writeln!(system_table.stdout(), "0x{:02x}  0x{:02x}  0x{:04x}  0x{:04x}  {}", bus, slot, vendor, device, desc);
            }
        }
    }


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




// https://wiki.osdev.org/PCI#Configuration_Space_Access_Mechanism_.231

struct Pci {
    config_address: Port<u32>,
    config_data:    Port<u32>,
}

impl Pci {
    /// # Safety
    ///
    /// * Assumes exclusive control over the PCI I/O configuration space (ports 0xCF8, 0xCFC)
    const unsafe fn new() -> Self {
        Self {
            config_address: Port::new(0xCF8),
            config_data:    Port::new(0xCFC),
        }
    }

    fn get_vendor_device(&mut self, bus: u8, slot: u8) -> Option<(u16, u16)> {
        let device_vendor = self.read_config_dword(bus, slot, 0, 0);
        let device = (device_vendor >> 16) as u16;
        let vendor = (device_vendor >>  0) as u16;
        if vendor == 0xFFFF {
            None
        } else {
            Some((vendor, device))
        }
    }

    fn read_config_dword(&mut self, bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
        let enabled = true;
        assert!(func <= 0b00000111);
        assert!(slot <= 0b00011111);
        assert!(offset & 0b11 == 0);

        let address =
            ((enabled as u32) << 31) |
            ((bus as u32)     << 16) |
            ((slot as u32)    << 11) |
            ((func as u32)    <<  8) |
            ((offset as u32)  <<  0) |
            0;

        self.config_address.write(address);
        self.config_data.read()
    }
}



// Vaguely based on http://www.randomhacks.net/2015/11/09/bare-metal-rust-cpu-port-io/

struct Port<T> {
    port:   u16,
    _type:  PhantomData<*mut T>,
}

impl<T> Port<T> {
    /// # Safety
    ///
    /// * Assumes the I/O port is valid, safe to read/write
    pub const unsafe fn new(port: u16) -> Self {
        Self { port, _type: PhantomData }
    }
}

impl Port<u8> {
    // XXX: Should these fns be unsafe?  Probably.
    pub fn write(&mut self, value: u8) { unsafe { llvm_asm!("outb %al, %dx" :: "{dx}"(self.port), "{al}"(value) :: "volatile") }; }
    pub fn read(&mut self) -> u8 { let result : u8; unsafe { llvm_asm!("inb %dx, %al" : "={al}"(result) : "{dx}"(self.port) :: "volatile") }; result }
}

impl Port<u16> {
    pub fn write(&mut self, value: u16) { unsafe { llvm_asm!("outw %ax, %dx" :: "{dx}"(self.port), "{ax}"(value) :: "volatile") }; }
    pub fn read(&mut self) -> u16 { let result : u16; unsafe { llvm_asm!("inw %dx, %ax" : "={ax}"(result) : "{dx}"(self.port) :: "volatile") }; result }
}

impl Port<u32> {
    pub fn write(&mut self, value: u32) { unsafe { llvm_asm!("outl %eax, %dx" :: "{dx}"(self.port), "{eax}"(value) :: "volatile") }; }
    pub fn read(&mut self) -> u32 { let result : u32; unsafe { llvm_asm!("inl %dx, %eax" : "={eax}"(result) : "{dx}"(self.port) :: "volatile") }; result }
}
