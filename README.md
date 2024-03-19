Developing a simple operating system in Rust

create bootimage with bootimage tool:
- install bootimage tool: cargo install bootimage
- run bootimage tool (execute in project dir): cargo bootimage

use the created bootimage with qemu: qemu-system-x86_64 -drive format=raw,file=target/x86_64-rustOStt/debug/bootimage-rustOS.bin
