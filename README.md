# Learn Embedded Programming with ESP32

This repository contains a collection of projects explained in the "[impl Rust on ESP32](https://github.com/ImplFerris/esp32-book)" book.  The book provides hands-on tutorials for learning embedded programming with the ESP32, covering various sensors, peripherals, and more.
You can read the book [here](https://esp32.implrust.com/).


## Toolchains for RISC-V and Xtensa Targets

You will also need `espup` to install the necessary toolchains. You can find details [here](https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html).

```sh
cargo install espup
espup install
```

## Using the Project example Without Modifications
When you create a project with the esp-generate, it automatically sets "esp" as the toolchain channel. If you want to "clone" and use example projects instead of creating one from scratch, you need to specify the toolchain name as "book-0.22" (as the project's rust-toolchain.toml configured with toolchain name book-0.22). 

```sh
espup install --name book-0.22 --toolchain-version 1.82.0
```

