# UEFI Hello World

Experimenting with Rust + UEFI



# Tested Versions

```
C:\>code --version
1.58.0
2d23c42a936db1c7b3b06f918cde29561cc47cd6
x64

C:\>rustc +nightly --version
rustc 1.55.0-nightly (8b87e8539 2021-07-08)

C:\>"C:\Program Files\qemu\qemu-system-x86_64" --version
QEMU emulator version 6.0.50 (v6.0.0-14198-gedb3abe0d1-dirty)
Copyright (c) 2003-2021 Fabrice Bellard and the QEMU Project developers
```

See also pinned versions in:
*   `rust-toolchain.toml`
*   `Cargo.lock`



# Downloads

* VS Code:  https://code.visualstudio.com/
* Rustup:   https://rustup.rs/
* QEMU:     https://www.qemu.org/download/#windows  (or specifically: https://qemu.weilnetz.de/w64/2021/qemu-w64-setup-20210706.exe )



# Overview

## xtask/...

[cargo xtask](https://github.com/matklad/cargo-xtask) style build logic.
Well, it would be, if I bothered to parse any command line arguments!
Since I only support `xtask b`, I just assume that's all you used.

Main build logic.  Creates a UEFI binary with:
```
cargo +nightly build -Z build-std=core --target=x86_64-pc-uefi-msvc.json --bin uefi-hello-world
```
Then wraps that in a FAT32 filesystem, and then wraps *that* in a disk image that uses GPT partitions to indicate it's a EFI System Partition.
`diskpart` or similar would be more straightforward, but host-OS-specific, so I've manually done things the hard way.



## uefi-hello-world/...

Using [the `uefi` crate](https://lib.rs/crates/uefi) would be a lot more straightforward.
Instead, I've created hello world almost entirely from scratch.
I do use [the `wchar` crate](https://lib.rs/crates/wchar) because I'm too lazy to type `&['H' as wchar_t, 'e' as wchar_t, ...]`.



## .cargo/config.toml

[cargo xtask](https://github.com/matklad/cargo-xtask) companion file



## .vscode/...

Setup build (`Ctrl`+`Shift`+`B`) & QEMU launch (`F5`) logic.
Actual sane debug environment not setup (PRs welcome!)



## target/...

* `x86_64-pc-uefi-msvc/debug/uefi-hello-world.efi` - the generated UEFI binary
* `debug/uefi-hello-world.img` - the generated disk image



<h2 name="license">License</h2>

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.



<h2 name="contribution">Contribution</h2>

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
