# YARS: Yet Another RISC-V Simulator

`yars` is a RISC-V simulator that currently supports RV32IM ISA.

## Building

To be able to build this project you will need Rust toolchain installed
([rustup](https://rustup.rs/) installation recommended).

```sh
$ cargo build --release
```

It will generate a binary at `target/release` which can be used standalone.

## Running

```sh
$ cargo run --release -- [FLAGS] [OPTIONS] <program>
```
OR
```sh
$ target/release/yars [FLAGS] [OPTIONS] <program>
```

This simulator only supports statically linked ELF binaries built for the
target triple `riscv32-unknown-elf`.

## Usage

| Flag                | Description                                           |
|---------------------|-------------------------------------------------------|
|`-h, --help`         | Prints help information                               |
|`-i, --interactive`  | Runs the program interactively                        |
|`-l, --log`          | Logs instruction execution                            |
|`-V, --version`      | Prints version information                            |
|`-m, --memory <size>`| Allocate `<size>` MiB for target memory [default: 32] |
|`--pc <address>`     | Override program entry point                          |

## License

This project is licensed under the [MIT License](LICENSE).
