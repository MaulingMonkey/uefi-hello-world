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

    // QEMU PCI Device Map:
    //
    //  Bus   Slot  Func  Vendor  Device  Description
    //  CC    Subcl PrgIF Rev             Category
    //
    //  0x00  0x00  0x00  0x8086  0x1237  Intel 440FX - 82441FX PMC [Natoma]
    //  0x06  0x00  0x00  0x02            Host Bridge
    //
    //  0x00  0x01  0x00  0x8086  0x7000  Intel 82371SB PIIX3 ISA [Natoma/Triton II]
    //  0x06  0x01  0x00  0x00            ISA Bridge
    //
    //  0x00  0x01  0x01  0x8086  0x7010  Intel ???
    //  0x01  0x01  0x80  0x00            IDE Controller (ISA Compatibility mode-only controller, supports bus mastering)
    //
    //  0x00  0x01  0x03  0x8086  0x7113  Intel ???
    //  0x06  0x80  0x00  0x03            Bridge (Other)
    //
    //  0x00  0x02  0x00  0x1234  0x1111  QEMU/Bochs VGA
    //  0x03  0x00  0x00  0x02            VGA Controller
    //
    //  0x00  0x03  0x00  0x8086  0x100e  Intel 82540EM Gigabit Ethernet Controller
    //  0x02  0x00  0x00  0x03            Ethernet Controller
    //
    // NOTES:
    //
    // The Intel 440FX has a PDF here: <https://wiki.qemu.org/images/b/bb/29054901.pdf>
    // This describes the PCI I/O configuration space quite thoroughly s(See 3.1 I/O Mapped Registers on page 17+)
    //
    // Intel 82371SB PIIX3 ISA PDF: <http://pdf.datasheetcatalog.com/datasheet/Intel/mXvqwzr.pdf>
    // The QEMU version lacks function 2 / USB support?

    let _ = writeln!(system_table.stdout());
    let _ = writeln!(system_table.stdout(), "PCI");
    let _ = writeln!(system_table.stdout(), "Bus   Slot  Func  Vendor  Device  Description");
    let _ = writeln!(system_table.stdout(), "CC    Subcl PrgIF Rev             Category");
    for bus in 0 ..= 255u8 {
        for slot in 0 ..= 31u8 {
            'funcs: for func in 0 ..= 7u8 {
                if let Some((vendor, device)) = pci.get_vendor_device(bus, slot, func) {
                    let (cc, subclass, pif, rev) = pci.get_cc_subclass_pif_rev(bus, slot, func);
                    let (bist, htype, latency, cacheline) = pci.get_bist_htype_latency_cacheline(bus, slot, func);
                    let multifunction = htype & 0x80 == 0x80;

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

                    // https://wiki.osdev.org/PCI#Class_Codes
                    let classdesc = match (cc, subclass, pif) {
                        (0x00, 0x00, _pif)  => "Non-VGA-Compatible Unclassified Device",
                        (0x00, 0x01, _pif)  => "VGA-Compatible Unclassified Device",
                        (0x00, _sub, _pif)  => "Unclassified Device",

                        (0x01, 0x00, _pif)  => "SCSI Bus Controller",
                        (0x01, 0x01, 0x00)  => "IDE Controller (ISA Compatibility mode only)",
                        (0x01, 0x01, 0x05)  => "IDE Controller (PCI native mode only)",
                        (0x01, 0x01, 0x0A)  => "IDE Controller (ISA Compatibility mode controller, supports both channels switched to PCI native mode)",
                        (0x01, 0x01, 0x0F)  => "IDE Controller (PCI native mode controller, supports both channels switched to ISA compatibility mode)",
                        (0x01, 0x01, 0x80)  => "IDE Controller (ISA Compatibility mode-only controller, supports bus mastering)",
                        (0x01, 0x01, 0x85)  => "IDE Controller (PCI native mode-only controller, supports bus mastering)",
                        (0x01, 0x01, 0x8A)  => "IDE Controller (ISA Compatibility mode controller, supports both channels switched to PCI native mode, supports bus mastering)",
                        (0x01, 0x01, 0x8F)  => "IDE Controller (PCI native mode controller, supports both channels switched to ISA compatibility mode, supports bus mastering)",
                        (0x01, 0x01, _pif)  => "IDE Controller (Unknown Prog IF)",
                        (0x01, 0x02, _pif)  => "Floppy Disk Controller",
                        (0x01, 0x03, _pif)  => "IPI Bus Controller",
                        (0x01, 0x04, _pif)  => "RAID Controller",
                        (0x01, 0x05, 0x20)  => "ATA Controller (Single DMA)",
                        (0x01, 0x05, 0x30)  => "ATA Controller (Chained DMA)",
                        (0x01, 0x05, _pif)  => "ATA Controller (Unknown Prog IF)",
                        (0x01, 0x06, 0x00)  => "SATA Controller (Vendor Specific Interface)",
                        (0x01, 0x06, 0x01)  => "SATA Controller (AHCI 1.0)",
                        (0x01, 0x06, 0x02)  => "SATA Controller (Serial Storage Bus)",
                        (0x01, 0x07, 0x00)  => "SAS Controller (SAS)",
                        (0x01, 0x07, 0x01)  => "SAS Controller (Serial Storage Bus)",
                        (0x01, 0x07, _pif)  => "SAS Controller (Unknown Prog IF)",
                        (0x01, 0x08, 0x01)  => "NVMHCI Controller",
                        (0x01, 0x08, 0x02)  => "NVM Express Controller",
                        (0x01, 0x08, _pif)  => "Non-Volatile Memory Controller",
                        (0x01, 0x80, _pif)  => "Mass Storage Controller (Other)",
                        (0x01, _sub, _pif)  => "Mass Storage Controller (Unknown Subclass)",

                        (0x02, 0x00, _pif)  => "Ethernet Controller",
                        (0x02, 0x01, _pif)  => "Token Ring Controller",
                        (0x02, 0x02, _pif)  => "FDDI Controller",
                        (0x02, 0x03, _pif)  => "ATM Controller",
                        (0x02, 0x04, _pif)  => "ISDN Controller",
                        (0x02, 0x05, _pif)  => "WorldFip Controller",
                        (0x02, 0x06, _pif)  => "PICMG 2.14 Multi Computing Controller",
                        (0x02, 0x07, _pif)  => "Infiniband Controller",
                        (0x02, 0x08, _pif)  => "Fabric Controller",
                        (0x02, 0x80, _pif)  => "Network Controller (Other)",
                        (0x02, _sub, _pif)  => "Network Controller (Unknown Subclass)",

                        (0x03, 0x00, 0x00)  => "VGA Controller",
                        (0x03, 0x00, 0x01)  => "8514/VGA-Compatible Controller",
                        (0x03, 0x00, _pif)  => "VGA Compatible Controller",
                        (0x03, 0x01, _pif)  => "XGA Controller",
                        (0x03, 0x02, _pif)  => "3D Controller (Not VGA-Compatible)",
                        (0x03, 0x80, _pif)  => "Display Controller (Other)",
                        (0x03, _sub, _pif)  => "Display Controller (Unknown Subclass)",

                        (0x04, 0x00, _pif)  => "Multimedia Video Controller",
                        (0x04, 0x01, _pif)  => "Multimedia Audio Controller",
                        (0x04, 0x02, _pif)  => "Computer Telephony Device",
                        (0x04, 0x03, _pif)  => "Audio Device",
                        (0x04, 0x80, _pif)  => "Multimedia Controller (Other)",
                        (0x04, _sub, _pif)  => "Multimedia Controller (Unknown Subclass)",

                        (0x05, 0x00, _pif)  => "RAM Controller",
                        (0x05, 0x01, _pif)  => "Flash Controller",
                        (0x05, 0x80, _pif)  => "Memory Controller (Other)",
                        (0x05, _sub, _pif)  => "Memory Controller (Unknown Subclass)",

                        (0x06, 0x00, _pif)  => "Host Bridge",
                        (0x06, 0x01, _pif)  => "ISA Bridge",
                        (0x06, 0x02, _pif)  => "EISA Bridge",
                        (0x06, 0x03, _pif)  => "MCA Bridge",
                        (0x06, 0x04, 0x00)  => "PCI-to-PCI Bridge (Normal Decode)",
                        (0x06, 0x04, 0x01)  => "PCI-to-PCI Bridge (Subtractive Decode)",
                        (0x06, 0x04, _pif)  => "PCI-to-PCI Bridge (Unknown Prog IF)",
                        (0x06, 0x05, _pif)  => "PCMCIA Bridge",
                        (0x06, 0x06, _pif)  => "NuBus Bridge",
                        (0x06, 0x07, _pif)  => "CardBus Bridge",
                        (0x06, 0x08, 0x00)  => "RACEway Bridge (Transparent Mode)",
                        (0x06, 0x08, 0x01)  => "RACEway Bridge (Endpoint Mode)",
                        (0x06, 0x08, _pif)  => "RACEway Bridge (Unknown Prog IF)",
                        (0x06, 0x09, 0x40)  => "PCI-to-PCI Bridge (Semi-Transparent, Primary bus towards host CPU)",
                        (0x06, 0x09, 0x80)  => "PCI-to-PCI Bridge (Semi-Transparent, Secondary bus towards host CPU)",
                        (0x06, 0x09, _pif)  => "PCI-to-PCI Bridge (Unknown Prog IF)",
                        (0x06, 0x0A, _pif)  => "InfiniBand-to-PCI Host Bridge",
                        (0x06, 0x80, _pif)  => "Bridge (Other)",
                        (0x06, _sub, _pif)  => "Bridge (Unknown Subclass)",

                        (0x07, _sub, _pif)  => "Simple Communication Controller",

                        (0x08, _sub, _pif)  => "Base System Peripheral",

                        (0x09, _sub, _pif)  => "Input Device Controller",

                        (0x0A, _sub, _pif)  => "Docking Station",

                        (0x0B, _sub, _pif)  => "Processor",

                        (0x0C, _sub, _pif)  => "Serial Bus Controller",

                        (0x0D, _sub, _pif)  => "Wireless Controller",

                        (0x0E, _sub, _pif)  => "Intelligent Controller",

                        (0x0F, _sub, _pif)  => "Satellite Communication Controller",

                        (0x10, _sub, _pif)  => "Encryption Controller",

                        (0x11, _sub, _pif)  => "Signal Processing Controller",

                        (0x12, _sub, _pif)  => "Processing Accelerator",

                        (0x13, _sub, _pif)  => "Non-Essential Instrumentation",

                        (0x40, _sub, _pif)  => "Co-Processor",

                        (0xFF, _sub, _pif)  => "Unassigned Class (Vendor Specific)",

                        (_cls, _sub, _pif)  => "Reserved Class",

                        _                   => "???",
                    };

                    let _ = writeln!(system_table.stdout());
                    let _ = writeln!(system_table.stdout(), "0x{:02x}  0x{:02x}  0x{:02x}  0x{:04x}  0x{:04x}  {}", bus, slot, func, vendor, device, desc);
                    let _ = writeln!(system_table.stdout(), "0x{:02x}  0x{:02x}  0x{:02x}  0x{:02x}            {}", cc, subclass, pif, rev, classdesc);

                    if func == 0 && !multifunction { break 'funcs }
                } else if func == 0 { break 'funcs }
            }
        }
    }


    let (w, h) = include!("../logo.png.dims");
    let h = h-1; // workaround https://github.com/rust-osdev/uefi-rs/pull/257
    let bin : &[u8]  = &include_bytes!("../logo.png.bin")[..];
    let bin : &[u32] = bytemuck::try_cast_slice(bin).unwrap();
    let bin = unsafe { core::mem::transmute(bin) };

    if false {
        let go : &mut GraphicsOutput = unsafe { &mut *system_table.boot_services().locate_protocol().unwrap().unwrap().get() };
        let (screen_w, screen_h) = go.current_mode_info().resolution();
        go.blt(BltOp::BufferToVideo { buffer: bin, dest: ((screen_w - w)/2, (screen_h - h)/2), dims: (w, h), src: BltRegion::Full }).unwrap().unwrap();
    }

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

    fn get_vendor_device(&mut self, bus: u8, slot: u8, func: u8) -> Option<(u16, u16)> {
        let device_vendor = self.read_config_dword(bus, slot, func, 0);
        let device = (device_vendor >> 16) as u16;
        let vendor = (device_vendor >>  0) as u16;
        if vendor == 0xFFFF {
            None
        } else {
            Some((vendor, device))
        }
    }

    // get_status_command

    fn get_cc_subclass_pif_rev(&mut self, bus: u8, slot: u8, func: u8) -> (u8, u8, u8, u8) {
        let dw = self.read_config_dword(bus, slot, func, 8);
        let cc          = (dw >> 24) as u8;
        let subclass    = (dw >> 16) as u8;
        let pif         = (dw >>  8) as u8;
        let rev         = (dw >>  0) as u8;
        (cc, subclass, pif, rev)
    }

    fn get_bist_htype_latency_cacheline(&mut self, bus: u8, slot: u8, func: u8) -> (u8, u8, u8, u8) {
        let dw = self.read_config_dword(bus, slot, func, 0xC);
        let bist        = (dw >> 24) as u8;
        let htype       = (dw >> 16) as u8;
        let latency     = (dw >>  8) as u8;
        let cacheline   = (dw >>  0) as u8;
        (bist, htype, latency, cacheline)
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
