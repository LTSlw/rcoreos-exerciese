# rCore Exercises

## Build

### in dir `user`

``` bash
cargo build --release
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/00_hello_world -O binary target/riscv64gc-unknown-none-elf/release/00_hello_world.bin
...
```

### in dir `os`

``` bash
LOG=TRACE cargo build --release
```

## Run

rustsbi-qemu should be downloaded manually.

``` bash
qemu-system-riscv64 -machine virt -nographic -bios target/riscv64gc-unknown-none-elf/release/rustsbi-qemu.bin -device loader,file=target/riscv64gc-unknown-none-elf/release/os
```