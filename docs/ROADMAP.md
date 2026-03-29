# EVO-OS Development Roadmap

This roadmap tracks the development of EVO-OS from initial bootloader to a full desktop OS experience. It is organized by milestone, with current status indicators.

---

## Status Legend

| Icon | Meaning |
|------|---------|
| ✅ | Complete |
| 🔄 | In Progress |
| 📋 | Planned |
| 🔬 | Research Phase |

---

## Milestone 0 — Foundation (Weeks 1–2)

**Goal:** Get "Hello World" printing in QEMU.

| Task | Status | Repo |
|------|--------|------|
| Set up Rust nightly toolchain + bare-metal target | ✅ | root |
| Write Stage 1 MBR bootloader (512 bytes, NASM) | ✅ | bootloader |
| Write Stage 2 bootloader (protected mode) | ✅ | bootloader |
| Rust kernel prints "Hello World" via VGA | ✅ | Kernel / src |
| QEMU boots successfully | ✅ | root |

---

## Milestone 1 — Kernel Core (Month 1)

**Goal:** Kernel boots, handles interrupts, manages memory.

| Task | Status | Repo |
|------|--------|------|
| IDT setup (Interrupt Descriptor Table) | ✅ | Kernel |
| PIC / APIC initialization | ✅ | Kernel |
| Physical memory manager (bitmap allocator) | ✅ | Kernel |
| 64-bit long mode transition in bootloader | ✅ | bootloader |
| VGA framebuffer driver (Mode 13h, 320×200) | ✅ | src/vga.rs |
| Color system (Radix-inspired palette) | ✅ | src/colors |
| Basic exception handlers (page fault, GPF) | 🔄 | Kernel |
| Heap allocator (`kmalloc` / `kfree` in Rust) | 🔄 | Kernel |

---

## Milestone 2 — Processes & Scheduling (Month 3)

**Goal:** Multiple processes running, system calls working.

| Task | Status | Repo |
|------|--------|------|
| Process Control Block (PCB) data structure | 🔄 | Kernel |
| Round-robin scheduler | 🔄 | Kernel |
| Context switching (save/restore registers) | 🔄 | Kernel |
| Virtual memory + page table management | 🔄 | Kernel |
| System call interface (syscall instruction) | 📋 | Kernel |
| Initial syscall table (read, write, exit, fork) | 📋 | Kernel |
| User/kernel mode separation | 📋 | Kernel |
| IPC: pipes | 📋 | Kernel |
| IPC: message queues | 📋 | Kernel |
| Timer-based preemption (PIT / HPET) | 📋 | Kernel |

---

## Milestone 3 — Storage & File System (Month 6)

**Goal:** Root filesystem mounts, shell can read/write files.

| Task | Status | Repo |
|------|--------|------|
| ATA PIO disk driver | 📋 | drivers |
| FAT32 filesystem read support | 📋 | filesystem |
| VFS (Virtual File System) layer | 📋 | filesystem |
| FAT32 filesystem write support | 📋 | filesystem |
| Mount root FS at boot | 📋 | Kernel |
| `/dev /proc /sys` virtual filesystems | 📋 | filesystem |
| Basic shell (fork + exec from FS) | 📋 | apps |
| Core utilities: `ls`, `cat`, `echo`, `mkdir` | 📋 | apps |
| Init system (PID 1, mounts and services) | 📋 | apps |

---

## Milestone 4 — Drivers & Hardware (Month 9)

**Goal:** Full hardware support for target platforms.

| Task | Status | Repo |
|------|--------|------|
| PS/2 Keyboard driver | ✅ | src |
| VGA Mode 13h driver | ✅ | src |
| PS/2 Mouse driver | 📋 | drivers |
| VESA/UEFI framebuffer (1920×1080) | 📋 | drivers |
| ATA/SATA DMA driver | 📋 | drivers |
| NVMe driver | 📋 | drivers |
| USB XHCI controller | 📋 | drivers |
| USB HID (keyboard + mouse) | 📋 | drivers |
| RTL8139 / Ethernet driver | 📋 | drivers |
| ACPI power management | 📋 | drivers |
| PCIe device enumeration | 📋 | drivers |

---

## Milestone 5 — Userspace & libc (Month 12)

**Goal:** C programs can run, POSIX basics work.

| Task | Status | Repo |
|------|--------|------|
| libc: `malloc` / `free` / `realloc` | 📋 | libc |
| libc: `printf` / `fprintf` / `sprintf` | 📋 | libc |
| libc: POSIX file I/O (`open`, `read`, `write`, `close`) | 📋 | libc |
| libc: string functions (`strlen`, `strcpy`, `memcpy`) | 📋 | libc |
| libc: `fork` / `exec` / `waitpid` | 📋 | libc |
| libc: `pthread` basics | 📋 | libc |
| Dynamic linker / ELF loader in userspace | 📋 | libc |
| Port `bash` or write a minimal shell | 📋 | apps |
| Port `coreutils` or equivalent | 📋 | apps |

---

## Milestone 6 — Networking (Year 1+)

**Goal:** TCP/IP stack, basic network apps.

| Task | Status | Repo |
|------|--------|------|
| Network card driver (e1000 / virtio-net) | 📋 | drivers |
| Ethernet frame handling | 📋 | Kernel |
| ARP protocol | 📋 | Kernel |
| IPv4 stack | 📋 | Kernel |
| TCP stack | 📋 | Kernel |
| UDP stack | 📋 | Kernel |
| DHCP client | 📋 | apps |
| DNS resolver | 📋 | apps |
| `curl`-like HTTP client | 📋 | apps |
| `ssh` client | 📋 | apps |

---

## Milestone 7 — GUI (Year 1–2)

**Goal:** Graphical desktop with running applications.

| Task | Status | Repo |
|------|--------|------|
| Framebuffer compositor | 📋 | GUI |
| Window manager (create, move, resize, z-order) | 📋 | GUI |
| Input event routing (keyboard, mouse) | 📋 | GUI |
| FreeType font rendering | 📋 | GUI |
| Widget toolkit (buttons, text, menus) | 📋 | GUI |
| IPC between apps and GUI compositor | 📋 | GUI |
| Terminal emulator app | 📋 | apps |
| File manager app | 📋 | apps |
| Settings panel app | 📋 | apps |
| Image viewer app | 📋 | apps |

---

## Milestone 8 — Security & Platform (Year 2+)

**Goal:** Secure boot, update infrastructure, multi-platform.

| Task | Status | Repo |
|------|--------|------|
| Secure Boot (UEFI key enrollment) | 🔬 | frimware |
| TPM measured boot (PCR recording) | 🔬 | frimware |
| FOTA engine (A/B slot updates) | 🔬 | frimware |
| ARM (AArch64) port | 🔬 | bsp |
| RISC-V port | 🔬 | bsp |
| POSIX compliance test suite | 📋 | Kernel + libc |
| Formal verification of kernel memory safety | 🔬 | Kernel |
| ext4 filesystem support | 📋 | filesystem |

---

## Long-Term Vision (Year 3+)

- Full POSIX compliance (run unmodified Linux/macOS CLI apps)
- Android app compatibility layer (via ART runtime)
- Own package manager and software repository
- Self-hosting (EVO-OS compiles its own toolchain on itself)
- Multi-user support with proper permission model
- Hardware security key (FIDO2) support
- Container / sandboxing support

---

*This roadmap is updated with each release. See the [CHANGELOG](../CHANGELOG.md) for what's been completed.*

---

## Full Printed Reference

A complete LaTeX/PDF reference guide covering all repository layers (Firmware → Bootloader → BSP → Kernel → Drivers → libc → Userspace → GUI) is available at [`docs/evo-os-guide.tex`](evo-os-guide.tex).

Compile with:

```bash
pdflatex docs/evo-os-guide.tex
```
