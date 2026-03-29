# EVO-OS

> **A completely new operating system built from scratch in Rust** — not a Linux fork, not a Windows clone, not a macOS derivative. EVO-OS is designed to run on both modern desktops (x86-64) and mobile/embedded targets (ARM), with a memory-safe kernel, a modular repo structure, and a clean userspace stack.

[![Rust](https://img.shields.io/badge/Rust-nightly-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue)](LICENSE)
[![Architecture](https://img.shields.io/badge/Arch-x86--64%20%7C%20ARM-green)](docs/ROADMAP.md)
[![Status](https://img.shields.io/badge/Status-Active%20Development-yellow)](docs/ROADMAP.md)

---

## Why EVO-OS?

| Aspect | Linux | Windows | macOS | **EVO-OS** |
|--------|-------|---------|-------|-------------|
| Primary Language | C | C / C++ | C / Obj-C | **Rust** |
| Memory Safety | ❌ | ❌ | ❌ | ✅ |
| Built from scratch | No (1991 rewrite) | No | No | **Yes** |
| Kernel Type | Monolithic | Hybrid | Hybrid (XNU) | **Monolithic → Hybrid** |
| FOTA support | Partial | No | No | **Yes (A/B slots)** |
| Open by default | ✅ | ❌ | ❌ | **✅** |

EVO-OS was built because every major OS carries decades of legacy debt — security vulnerabilities from C/C++, architectural decisions made in the 1970s–90s, and closed or semi-closed development models. We start fresh, with Rust's compile-time memory safety guarantees, a clean modular architecture, and a publicly documented design from the very first boot.

---

## Full Boot Chain

```
┌────────────────────────────────────────────────────────────────┐
│  Power On                                                      │
│  CPU jumps to reset vector (hardwired ROM address)            │
└────────────────────┬───────────────────────────────────────────┘
                     ▼
┌────────────────────────────────────────────────────────────────┐
│  UEFI / BIOS  [frimware repo]                                  │
│  POST → RAM training → PCIe init → clock/voltage → Secure Boot │
│  Loads and verifies the bootloader via Secure Boot key chain  │
└────────────────────┬───────────────────────────────────────────┘
                     ▼
┌────────────────────────────────────────────────────────────────┐
│  FOTA Engine  [frimware repo]                                  │
│  Checks for pending firmware/OS image updates (A/B slots)     │
│  Applies signed update packages, rolls back on failure        │
│  (Triggered on Android/embedded targets before main OS boot)  │
└────────────────────┬───────────────────────────────────────────┘
                     ▼
┌────────────────────────────────────────────────────────────────┐
│  Bootloader — Stage 1 + Stage 2  [bootloader repo]            │
│  Stage 1: 512-byte MBR, loads Stage 2 from disk               │
│  Stage 2: sets up 64-bit long mode, loads kernel ELF          │
│  Passes memory map + boot params to kernel entry point        │
└────────────────────┬───────────────────────────────────────────┘
                     ▼
┌────────────────────────────────────────────────────────────────┐
│  EVO-OS Kernel  [Kernel repo]  — Rust no_std                  │
│  Memory manager (physical + paging + heap)                    │
│  Scheduler (Round Robin → Priority → CFS)                     │
│  Interrupt/exception handlers (IDT, PIC/APIC)                 │
│  System call interface (userspace ↔ kernel bridge)            │
│  IPC: pipes, message queues, shared memory                    │
└────────────────────┬───────────────────────────────────────────┘
                     ▼
┌────────────────────────────────────────────────────────────────┐
│  Drivers + BSP  [drivers repo] [bsp repo]                      │
│  Keyboard, VGA/framebuffer, storage (ATA/NVMe), network, USB  │
│  Board Support Packages for x86-64 and ARM targets            │
└────────────────────┬───────────────────────────────────────────┘
                     ▼
┌────────────────────────────────────────────────────────────────┐
│  libc + Filesystem  [libc repo] [filesystem repo]              │
│  relibc-style Rust libc — malloc, printf, POSIX I/O           │
│  VFS layer: FAT32 (initial), ext4 (planned)                   │
└────────────────────┬───────────────────────────────────────────┘
                     ▼
┌────────────────────────────────────────────────────────────────┐
│  Userspace: init + shell + GUI  [GUI repo] [apps repo]         │
│  Init system → mounts filesystems → starts services           │
│  Framebuffer compositor → window manager → display server     │
│  Bundled apps: shell, file manager, settings, browser         │
└────────────────────────────────────────────────────────────────┘
```

---

## Repository Map

This monorepo uses **git submodules** — each layer has its own repository with independent versioning and ownership:

| Repo | Layer | Description |
|------|-------|-------------|
| [`frimware`](frimware/) | Firmware | UEFI/BIOS core, FOTA engine, hardware init, flash layout, TPM/Secure Boot |
| [`bootloader`](bootloader/) | Boot | Stage 1 (MBR), Stage 2 (long mode setup), kernel ELF loader |
| [`bsp`](bsp/) | BSP | Board Support Packages — hardware-specific configs for x86-64 and ARM |
| [`Kernel`](Kernel/) | Kernel | Core OS kernel — memory, scheduler, syscalls, IPC, interrupts |
| [`drivers`](drivers/) | Drivers | Device drivers — keyboard, VGA, storage, network, USB |
| [`libc`](libc/) | Userspace | Rust-native C standard library (relibc-style), POSIX ABI compatible |
| [`filesystem`](filesystem/) | Storage | VFS interface + FAT32/ext4 filesystem implementations |
| [`GUI`](GUI/) | Graphics | Framebuffer compositor, window manager, display server, font rendering |
| [`apps`](apps/) | Applications | Bundled first-party apps — shell, file manager, settings, browser |

---

## Architecture Overview

```
┌──────────────────────────────────────────────────────────┐
│ Layer 5 — Applications          [apps]                   │
│  Shell · File Manager · Settings · Browser               │
├──────────────────────────────────────────────────────────┤
│ Layer 4 — GUI & Display         [GUI]                    │
│  Framebuffer · Compositor · Window Manager · Fonts       │
├──────────────────────────────────────────────────────────┤
│ Layer 3 — Userspace Runtime     [libc] [filesystem]      │
│  relibc · VFS · FAT32 · ext4                             │
├──────────────────────────────────────────────────────────┤
│ Layer 2 — Kernel                [Kernel] [drivers] [bsp] │
│  Memory · Scheduler · Syscalls · IPC · Drivers · BSP    │
├──────────────────────────────────────────────────────────┤
│ Layer 1 — Boot                  [bootloader]             │
│  Stage 1 (MBR) · Stage 2 (Long Mode) · ELF Loader       │
├──────────────────────────────────────────────────────────┤
│ Layer 0 — Firmware              [frimware]               │
│  UEFI/BIOS · FOTA · TPM · Secure Boot · Flash Layout    │
└──────────────────────────────────────────────────────────┘
```

---

## Quick Start

### Prerequisites

```bash
# Rust nightly + bare-metal target
rustup install nightly
rustup component add rust-src --toolchain nightly
rustup target add i686-unknown-none

# Tools
sudo apt install nasm qemu-system-x86 gcc binutils
```

### Build & Run in QEMU

```bash
# Clone with all submodules
git clone --recurse-submodules https://github.com/EVO-OS/evo-os.git
cd evo-os

# Build the full OS image
make all

# Boot in QEMU
make run

# Debug with GDB
make debug
```

### Build a Single Submodule

```bash
# Just the kernel
cd Kernel && make all

# Just the bootloader
cd bootloader && make all
```

---

## Project Structure

```
evo-os/                        ← Root monorepo (this repo)
├── frimware/                  ← [submodule] Firmware layer
├── bootloader/                ← [submodule] Boot stages
├── bsp/                       ← [submodule] Board support packages
├── Kernel/                    ← [submodule] OS kernel (Rust + C)
├── drivers/                   ← [submodule] Device drivers
├── libc/                      ← [submodule] C standard library (Rust)
├── filesystem/                ← [submodule] VFS + filesystem impls
├── GUI/                       ← [submodule] Display server + compositor
├── apps/                      ← [submodule] Bundled applications
├── boot/                      ← Local bootloader ASM (stage1, stage2)
├── src/                       ← Local Rust kernel source
│   ├── lib.rs                 ← Kernel entry point
│   ├── vga.rs                 ← VGA graphics driver
│   ├── colors/                ← Color system (Radix-inspired)
│   └── shell/                 ← Built-in shell
├── docs/
│   ├── BOOT_CHAIN.md          ← Detailed boot chain documentation
│   └── ROADMAP.md             ← Development roadmap
├── ARCHITECTURE.md            ← Full system architecture
├── CONTRIBUTING.md            ← Contribution guide
├── Cargo.toml                 ← Rust workspace
├── Makefile                   ← Top-level build system
├── linker.ld                  ← Kernel linker script
└── i686-evo.json              ← Custom Rust bare-metal target
```

---

## Built-in Shell Commands

Once booted, EVO-OS provides a minimal interactive shell:

| Command | Description |
|---------|-------------|
| `help` | Show available commands |
| `clear` | Clear the screen |
| `echo <text>` | Print text to terminal |
| `version` | Show OS version |
| `about` | About EVO-OS |
| `color <0-15>` | Change text color |
| `mem` | Show memory map |
| `reboot` | Reboot the system |
| `halt` | Shutdown the system |

---

## Roadmap

| Timeline | Milestone |
|----------|-----------|
| Week 1–2 | Bootloader prints "Hello" in QEMU ✅ |
| Month 1 | Kernel boots, handles interrupts, basic memory management ✅ |
| Month 3 | Processes, scheduler, system calls working 🔄 |
| Month 6 | File system, shell, basic userspace programs 🔄 |
| Year 1 | Networking, GUI compositor, driver ecosystem 📋 |
| Year 2+ | POSIX compliance, full app ecosystem, multi-platform 📋 |

See [`docs/ROADMAP.md`](docs/ROADMAP.md) for the detailed roadmap.

---

## Documentation

- [Architecture Overview](ARCHITECTURE.md)
- [Boot Chain](docs/BOOT_CHAIN.md)
- [Development Roadmap](docs/ROADMAP.md)
- [Contributing Guide](CONTRIBUTING.md)

### Per-Repo Documentation

Each submodule has its own architecture and design documentation:
- [Firmware Docs](frimware/ARCHITECTURE.md)
- [Bootloader Stages](bootloader/docs/STAGES.md)
- [Kernel Architecture](Kernel/ARCHITECTURE.md)
- [Driver Model](drivers/docs/DRIVER_MODEL.md)
- [libc ABI](libc/docs/ABI.md)
- [VFS Design](filesystem/docs/VFS.md)
- [GUI Compositor](GUI/docs/COMPOSITOR.md)
- [App Model](apps/docs/APP_MODEL.md)

---

## License

MIT License — see [LICENSE](LICENSE)

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) first.

---

**EVO-OS** — *Something completely new.*
