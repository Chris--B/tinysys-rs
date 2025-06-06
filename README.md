# `tinysys` SDK from to Rust
`tinysys` is a hobbyist compute platform: https://github.com/ecilasun/tinysys.
> Tinysys started out as a hobby project. It now has two RISC-V cores, and several other facilities listed below, and can happily run most software with minimal tweaks.
>
> Of course, before you ask, it does run DOOM, and with sound and keyboard input! (Lately, it has been running Quake as well.)

This repo exposes raw and idiomatic C bindings of the [tinysys SDK](https://github.com/ecilasun/tinysys/blob/main/software/SDK/README.md) to Rust. For the `sys` crate, we use `bindgen` to generate and **check in** the bindings. This means using the crate has light dependencies, but updating it is manual. See the section [Updating the SDK+Bindings](#updating-the-sdkbindings) below for more details.

### Building the crates
The crate builds with a single `cargo build`. 

We set the default target-triple in `.cargo/config.toml` so it works for `tinysys` out of the box.

## TODOs

Here's a list of TODOs I'm working on in no particular order.
- [ ] Idiomatic Rust traits etc
    - We should provide impls for utiltiy traits like those from `bytemuck` on the raw C types.
    - Due to the Orphan Rule, this crate **must** provide them. Unlike idiomatic wrappers, clients cannot provide these.
- [ ] Idiomatic Rust wrappers
    - It would be nice to have bindings that are more ergonomic to use

## Updating the SDK+Bindings
To update everything, you need to install a RISCV toolchain, update the SDK, and build the C/++ code. The SDK can be updated, built, or bindings regenerated independently of eachother.

### Installing riscv-tools
Install the Rust target with rustup:
```sh
# TODO: Not sure if this is the right target for tinysys, but it's what I'm using atm.
rustup target add riscv32imac-unknown-none-elf
```

#### Windows
TODO: I'm not sure how to install riscv-tools on these platforms yet, so in the meantime suggest following Linux under WSL.

### Linux
TODO: Untested yet, https://github.com/riscv-software-src/homebrew-riscv?tab=readme-ov-file#installation

#### macOS
Install `riscv-tools` using homebrew, as detailed [here](https://github.com/riscv-software-src/homebrew-riscv?tab=readme-ov-file#installation).
Also install LLVM. We currently use homebrew LLVM for compiling the SDK due to an issue where `bindgen`(clang) and `riscv-tools` disagree on whether `uint32_t` is an `unsigned int` or an `unsigned long`. This breaks our bindings, but a Clang build with a riscv backend resolves it. Apple Clang does not ship with a riscv backend.

```sh
brew tap riscv-software-src/riscv
brew install riscv-tools llvm
```

### Downloading the C SDK
With that installed, make sure it's findable in the path and run the update script:
```sh
./tinysys-sys/c_sdk/update_sdk.sh 
```
This downloads the latest SDK files from the `tinysys` repo and copies it in into `tinysys-sys/c_sdk/SDK`. The old `SDK` is copied to `SDK.old` as a backup.

### Rebuilding
To rebuild `libtinysys_sdk.a`, run the following:
```sh
make -C tinysys-sys/c_sdk build
```

### Running `bindgen`
Then run bindgen to update the bindings.
```sh
make -C tinysys-sys/c_sdk sdk.rs
cargo fmt
```
This generates `tinysys-sys/src/include/sdk.h` and `tinysys-sys/src/sdk.rs` from the previously-downloaded SDK and runs `bindgen`.
Currently it pulls riscv headers using `riscv64-unknown-elf-gcc -print-sysroot`. If it is named differently on your platform, you'll have to edit `./tinysys-rs/tinysys-sys/c_sdk/Makefile` locally.

If you're only changing the bindgen options, you do not need to rerun `./tinysys-sys/c_sdk/update_sdk.sh`.
