# Contributing to EVO-OS

Thank you for your interest in contributing to EVO-OS! This guide explains how to contribute effectively across the multi-repo architecture.

---

## Before You Start

1. Read the [Architecture Overview](ARCHITECTURE.md) to understand how the repos are structured.
2. Read the [Boot Chain](docs/BOOT_CHAIN.md) to understand how components interact.
3. Pick the right repo for your change (see the layer map below).
4. Check open issues in the relevant submodule repo before starting.

---

## Which Repo Do I Contribute To?

| What you want to change | Target repo |
|------------------------|-------------|
| UEFI/BIOS, FOTA, Secure Boot, TPM | `frimware/` |
| Stage 1/2 boot, long mode setup, ELF loading | `bootloader/` |
| Hardware-specific platform configs | `bsp/` |
| Memory manager, scheduler, syscalls, IPC | `Kernel/` |
| Device drivers (keyboard, VGA, storage, USB) | `drivers/` |
| C standard library, POSIX functions | `libc/` |
| VFS, FAT32, ext4 | `filesystem/` |
| Display server, compositor, window manager | `GUI/` |
| Shell, file manager, settings, browser | `apps/` |
| Root build system, top-level docs | `evo-os/` (this repo) |

---

## Getting Started

### Clone the Full Repo

```bash
git clone --recurse-submodules https://github.com/EVO-OS/evo-os.git
cd evo-os
```

### Update All Submodules

```bash
git submodule update --init --recursive
```

### Build Prerequisites

```bash
# Rust nightly + components
rustup install nightly
rustup override set nightly
rustup component add rust-src llvm-tools-preview
rustup target add i686-unknown-none x86_64-unknown-none

# System tools
sudo apt install nasm qemu-system-x86 qemu-system-arm \
                 gcc binutils gdb build-essential
```

---

## Development Workflow

### 1. Create a Feature Branch

Work on a branch in the **relevant submodule repo**, not the root:

```bash
cd Kernel
git checkout -b feature/my-scheduler-improvement
```

### 2. Build & Test

```bash
# Build entire OS
cd /path/to/evo-os
make all

# Run in QEMU (x86)
make run

# Run with debug output
make debug

# Build only the kernel
cd Kernel && make all

# Clean everything
make clean
```

### 3. Debug with GDB

```bash
# Terminal 1: start QEMU with GDB stub
make debug-gdb

# Terminal 2: connect GDB
gdb build/kernel.elf
(gdb) target remote :1234
(gdb) break kernel_main
(gdb) continue
```

---

## Commit Conventions

We follow **Conventional Commits**:

```
<type>(<scope>): <short description>

[optional body]

[optional footer: Fixes #issue]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `refactor`: Code restructuring
- `perf`: Performance improvement
- `test`: Adding or updating tests
- `chore`: Build, CI, or tooling changes

**Scopes** (use the repo layer name):
- `firmware`, `bootloader`, `bsp`, `kernel`, `drivers`, `libc`, `fs`, `gui`, `apps`

**Examples:**
```
feat(kernel): add round-robin scheduler
fix(drivers): resolve VGA double-buffer race condition
docs(firmware): document FOTA A/B slot protocol
perf(kernel): optimize page table walker for large mappings
```

---

## Layer-Specific Guidelines

### Firmware (`frimware/`)
- All cryptographic operations must use constant-time implementations.
- FOTA update logic must always have a rollback path.
- Secure Boot key enrollment code requires two-party review.
- Test with QEMU UEFI (`-bios /usr/share/OVMF/OVMF_CODE.fd`).

### Bootloader (`bootloader/`)
- Stage 1 must remain exactly 512 bytes. Use `wc -c build/stage1.bin` to verify.
- Stage 2 must correctly transition through 16-bit → 32-bit → 64-bit modes.
- Test every change booting in QEMU before submitting.

### Kernel (`Kernel/`)
- All kernel code must be `no_std`. No `std` imports allowed in kernel crates.
- Use Rust's `unsafe` blocks sparingly and document every `unsafe` block.
- Scheduler changes require running the full QEMU test suite.
- Memory allocator changes require a correctness proof or test coverage.

### Drivers (`drivers/`)
- Drivers must implement the `Driver` trait defined in the kernel.
- Hardware access must go through the BSP HAL, not direct port I/O.
- Every driver must handle initialization failure gracefully (return `Err`, don't panic).

### libc (`libc/`)
- Maintain C ABI compatibility — all public functions must have `#[no_mangle]` and `extern "C"`.
- POSIX functions must match the spec exactly (return values, errno codes).
- Fuzz-test string and memory functions with `cargo-fuzz`.

### Filesystem (`filesystem/`)
- All FS implementations must implement the `VfsNode` trait.
- Never panic on malformed disk data — always return `Err`.
- Test with both clean and intentionally corrupted disk images.

### GUI (`GUI/`)
- All rendering must go through the framebuffer abstraction, not direct memory writes.
- Window events must be processed through the event queue — no busy-polling.
- Font rendering must support both bitmap and vector (FreeType) fonts.

### Apps (`apps/`)
- Apps must only use the public syscall ABI — no internal kernel symbols.
- Each app should have a `--help` flag.
- Apps must handle `SIGTERM` and `SIGKILL` gracefully.

---

## Pull Request Process

1. **Open an issue first** for any non-trivial change to discuss the approach.
2. **PRs should be small** — one logical change per PR.
3. **Write a clear description** explaining *what* changed and *why*.
4. **Include test evidence** — QEMU screenshot, log output, or test run.
5. **Two approvals required** for firmware and kernel changes; one for others.

---

## Code Review Standards

Reviewers will check:
- ✅ Does it compile with `cargo build --release`?
- ✅ Does the OS still boot in QEMU after the change?
- ✅ Is `unsafe` used correctly and minimally?
- ✅ Is new code covered by tests or documented integration test steps?
- ✅ Does it follow the layer's interface contracts?
- ✅ Are error paths handled (no `unwrap()` in kernel/firmware code)?

---

## Getting Help

- Open a Discussion in the relevant submodule repo
- File a bug with the `help wanted` label
- Read the [OSDev Wiki](https://wiki.osdev.org/) for OS fundamentals
- Study [xv6](https://pdos.csail.mit.edu/6.828/2023/xv6.html) for clean kernel patterns

---

*EVO-OS is built for everyone who has ever looked at an OS and thought — "we can do this better."*
