use fatfs::{FatType, FileSystem, FormatVolumeOptions, FsOptions};
use gpt::disk::LogicalBlockSize;
use gpt::partition_types;

use std::convert::TryInto;
use std::fs::File;
use std::io::Cursor;
use std::process::Command;



fn main() {
    run("cargo build -Z build-std=core,compiler_builtins,alloc --target=x86_64-unknown-uefi --bin uefi-hello-world");
    create_imgs();
}

fn create_imgs() {
    let lbs = LogicalBlockSize::Lb512;
    let efi_system_partition_src = create_efi_system_partition();
    let disk_size = efi_system_partition_src.len() + 2*34 * 512;

    let mut disk = vec![0u8; disk_size];
    let mut dd = Box::new(Cursor::new(&mut disk[..]));
    gpt::mbr::ProtectiveMBR::with_lb_size((disk_size/512).try_into().unwrap_or(0xFFFFFFFF)).overwrite_lba0(&mut dd).expect("failed to create protective MBR");
    let mut gpt = gpt::GptConfig::default().initialized(false).writable(true).logical_block_size(lbs).create_from_device(dd, None).expect("failed to create GptDisk");
    gpt.update_partitions(Default::default()).expect("failed to create blank partition table");

    let efi_system_partition_label = "EFI System"; // I vaguely recall reading that some buggy UEFI implementations require exactly this label to boot?  Can't find a primary source for this though, so don't quote me on that.
    let efi_system_partition_id = gpt.add_partition(efi_system_partition_label, (efi_system_partition_src.len()).try_into().unwrap(), partition_types::EFI, 0).expect("failed to create EFI system partition");
    let efi_system_partition = gpt.partitions().get(&efi_system_partition_id).expect("unable to query EFI system partition");
    let efi_system_partition_start : usize = efi_system_partition.bytes_start(lbs).unwrap().try_into().unwrap();
    //let efi_system_partition_len   : usize = efi_system_partition.bytes_len(lbs).unwrap().try_into().unwrap(); // buggy: https://github.com/Quyzi/gpt/issues/64
    gpt.write().unwrap();

    let efi_system_partition_dst = &mut disk[efi_system_partition_start ..];
    efi_system_partition_dst[.. efi_system_partition_src.len()].copy_from_slice(&efi_system_partition_src);

    std::fs::write("target/debug/uefi-hello-world.img", &disk[..]).expect("Unable to write image");
}

fn create_efi_system_partition() -> Vec<u8> {
    let mut p = vec![0u8; 32 * 1024 * 1024];

    fatfs::format_volume(Cursor::new(&mut p[..]), FormatVolumeOptions::new().fat_type(FatType::Fat32).volume_label(*b"RUSTY-UEFI!")).expect("Unable to format image");

    {
        let fs = FileSystem::new(Cursor::new(&mut p[..]), FsOptions::new()).expect("Unable to create filesystem");
        fs.root_dir().create_dir("EFI").unwrap();
        fs.root_dir().create_dir("EFI/BOOT").unwrap();

        let mut src_efi = File::open("target/x86_64-unknown-uefi/debug/uefi-hello-world.efi").expect("Unable to read uefi-hello-world.efi");
        let mut dst_efi = fs.root_dir().create_file("EFI/BOOT/BOOTX64.EFI").expect("Unable to create file");
        std::io::copy(&mut src_efi, &mut dst_efi).unwrap();
    }

    p
}

fn run(cmd: &str) {
    let mut args = cmd.split(' ');
    let mut cmd = Command::new(args.next().expect("run: empty command"));
    cmd.args(args);
    let status = cmd.status().unwrap();
    assert!(status.code() == Some(0), "{:?} failed with exit code: {:?}", cmd, status.code());
}
