# HighlightOS

<!-- logo instead of name -->

x86_64 OS (kernel) made completely from scratch in Assembly & Rust

[![README in English](https://img.shields.io/badge/EN-%F0%9F%87%AC%F0%9F%87%A7-blue?color=%23ffb454&labelColor=%230a0c0c)](https://github.com/adamperkowski/highlightos/blob/main/README.md)
[![README in Polish](https://img.shields.io/badge/PL-%F0%9F%87%B5%F0%9F%87%B1-blue?color=%23ffb454&labelColor=%230a0c0c)](https://github.com/adamperkowski/highlightos/blob/main/README-pl.md)
[![README in Italian](https://img.shields.io/badge/IT-%F0%9F%87%AE%F0%9F%87%B9-blue?color=%23ffb454&labelColor=%230a0c0c)](https://github.com/adamperkowski/highlightos/blob/main/README-it.md)
[![Documentation](https://img.shields.io/badge/Documentation-%F0%9F%8C%84-blue?color=%23ffb454&labelColor=%230a0c0c)](https://os.adamperkowski.dev)

[![GitHub Release](https://img.shields.io/github/v/release/adamperkowski/highlightos?label=Latest%20Released%20Version&color=%23ffb454&labelColor=%230a0c0c)](https://github.com/adamperkowski/highlightos/releases)
[![GitHub License](https://img.shields.io/github/license/adamperkowski/highlightos?label=License&color=%23ffb454&labelColor=%230a0c0c)](https://github.com/adamperkowski/highlightos/blob/main/LICENSE) ![GitHub repo size](https://img.shields.io/github/repo-size/adamperkowski/highlightos?label=Repo%20Size&color=%23ffb454&labelColor=%230a0c0c)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/adamperkowski/highlightos/asm.yml?branch=main&label=ASM%20Build&color=%23ffb454&labelColor=%230a0c0c)](https://github.com/adamperkowski/highlightos/actions) [![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/adamperkowski/highlightos/rust.yml?branch=main&label=HLKernel%20Build&color=%23ffb454&labelColor=%230a0c0c)](https://github.com/adamperkowski/highlightos/actions)

## Table of contents
- [Build from source on Linux](#build-from-source-on-linux)
- [Run in QEMU on Linux](#run-in-qemu-on-linux)
- [Run on real hardware](#run-on-real-hardware)
- [Appendix](#appendix)
  - [Development status](#development-status)
  - [Additional info](#additional-info)
  - [Credits](#credits)

<!-- showcase -->
<!-- features -->
<!-- List of commands and features will be available soon. -->

<!-- installation & docs -->
## Build from source on Linux
### Main Kernel
**Requirements:**
 - [Git](https://git-scm.com) version control system
 - [Rust toolchain](https://www.rust-lang.org/tools/install)

**Steps:**
 1. Install the nightly toolchain:
```bash
rustup toolchain install nightly
```
 2. Install required components:
```bash
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu && rustup component add llvm-tools-preview --toolchain nightly-x86_64-unknown-linux-gnu && cargo install bootimage
```
 3. Create a local clone of the repository:
```bash
git clone git@github.com:adamperkowski/highlightos.git && cd highlightos
```
 4. `cd` into the `kernel/` directory:
```bash
cd kernel
```
 5. Build the bootable binary:
```bash
cargo bootimage --release
```
> This command will create the `target/target/release` directory in which you'll find the `bootimage-hlkernel.bin` binary file.

### HighlightOS ASM
**Requirements:**
 - [Git](https://git-scm.com) version control system
 - [NASM](https://nasm.us) Assembly compiler

**Steps:**
 1. Create a local clone of the repository:
```bash
git clone git@github.com:adamperkowski/highlightos.git && cd highlightos
```
 2. `cd` into the `asm/` directory:
```bash
cd asm
```
 3. Compile the bootable binary:
```bash
nasm -f bin boot.asm -o boot.bin
```
> This command will generate & place the `boot.bin` file inside of the current directory.

## Run in QEMU on Linux

**Requirements:**
 - [QEMU](https://www.qemu.org/download/#linux) (full package)
 - A bootable binary of HighlightOS. You can download one from [releases](https://github.com/adamperkowski/highlightos/releases) or you can [build it yourself](#build-from-source-on-linux).

**Steps:**
 1. `cd` into directory that contains the binary.
 2. Run the following command:
```bash
qemu-system-x86_64 -drive format=raw,file=<your_binary_filename>.bin
```
> [!IMPORTANT]
> Replace `<your_binary_filename>` with the actual name of the binary you have downloaded/built.

## Run on real hardware
You can also flash the binary image onto a USB stick and boot it on a real machine. 

You can flash it by running the following command:
```bash
dd if=<your_binary_filename>.bin of=/dev/sdX && sync
```

> [!IMPORTANT]
> Make sure to replace `<your_binary_filename>.bin` with your downloaded/compiled binary name and make sure to replace `/dev/sdX` with your USB's actual partition number. **Any data on it will be lost!**

> [!NOTE]
>You can choose the device to boot off of from your BIOS boot menu (accessible by pressing <kbd>F8</kbd> or <kbd>F12</kbd>).
>
>**<ins>Double-check that your motherboard is capable of booting legacy media(s)</ins>, as HighlightOS is not UEFI-compatible yet.**

<!-- contributing -->
## Appendix

[![Documentation](https://img.shields.io/badge/Documentation-%F0%9F%8C%84-blue?color=%23ffb454&labelColor=%230a0c0c)](https://os.adamperkowski.dev)

### Development status
https://github.com/user-attachments/assets/663e8477-4275-411b-a39a-c12e54096ad8

### Additional info
**Did you know we have a IRC channel? It's `#highlightos` on [libera.chat](https://libera.chat).**

List of built-in commands and features is available [here](https://github.com/adamperkowski/highlightos/wiki/Commands#built-in-commands).<br>
To gain further information on HighlightOS, we invite you to visit the [wiki](https://github.com/adamperkowski/highlightos/wiki/).

**_Using precompiled binaries from the code section of the repo is not recommended._**

### Contributors
**HUGE thanks to everyone contributing:**

<a href="https://github.com/adamperkowski/highlightos/graphs/contributors">
  <img src="https://raw.githubusercontent.com/adamperkowski/highlightos/gh-pages/CONTRIBUTORS.svg"/>
</a>

### Credits
*Some parts of the code are inspired by [blog_os](https://github.com/phil-opp/blog_os). Great project!*

*Copyleft 2024 Adam Perkowski*
