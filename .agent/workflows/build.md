---
description: Build the full EVO-OS image and all submodules
---

# Build EVO-OS

This workflow builds the complete EVO-OS bootable disk image from all submodule components.

## Prerequisites

Ensure you have all required tools installed:

```bash
rustup install nightly
rustup component add rust-src --toolchain nightly
sudo apt install nasm qemu-system-x86 gcc binutils build-essential
```

## Steps

// turbo
1. Update all git submodules to their latest tracked commits:
```bash
git submodule update --init --recursive
```

// turbo
2. Clean all previous build artifacts:
```bash
make clean
```

// turbo
3. Build the Rust kernel (no_std, custom i686-evo target):
```bash
RUST_TARGET_PATH=$(pwd) cargo build --release --target i686-evo.json -Zbuild-std=core -Zbuild-std-features=compiler-builtins-mem
```

// turbo
4. Assemble the Stage 1 bootloader (must be exactly 512 bytes):
```bash
nasm -f bin boot/stage1.asm -o build/stage1.bin
wc -c build/stage1.bin  # Verify: must be 512
```

// turbo
5. Assemble the Stage 2 bootloader:
```bash
nasm -f bin boot/stage2.asm -o build/stage2.bin
```

6. Build the complete OS disk image:
```bash
make all
```

## Expected Output

On success, you should see:
```
╔═══════════════════════════════════════════════════════╗
║  EvoOS (Rust Edition) built successfully!             ║
║  Run 'make run' to boot in QEMU                       ║
╚═══════════════════════════════════════════════════════╝
```

The bootable disk image will be at: `build/evo.img`

## Troubleshooting

- **"error: no such target: i686-evo"** → Ensure `RUST_TARGET_PATH=$(pwd)` is set when calling cargo
- **"nasm: command not found"** → Install with `sudo apt install nasm`
- **"Stage 1 is not 512 bytes"** → The bootloader ASM has a size error; check `boot/stage1.asm` padding
