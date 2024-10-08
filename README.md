# rCore Exercises

## Build

``` bash
LOG=TRACE cargo build --release
```

## Run

``` bash
qemu-system-riscv64 -machine virt -nographic -bios target/riscv64gc-unknown-none-elf/release/rustsbi-qemu.bin -device loader,file=target/riscv64gc-unknown-none-elf/release/os
```