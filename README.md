# EvoOS

> A completely new operating system written in **Rust** from scratch — not Linux, not Windows, not macOS.

![Rust](https://img.shields.io/badge/Rust-100%25-orange)
![License](https://img.shields.io/badge/License-MIT-blue)
![Architecture](https://img.shields.io/badge/Arch-x86-green)

## Features

- 🦀 **100% Rust Kernel** — Memory safe, no undefined behavior
- 🎨 **Radix-inspired Color System** — Semantic color tokens and themes
- 💻 **Bash-like Shell** — Built-in commands: `help`, `clear`, `echo`, `reboot`
- 🖥️ **VGA Graphics** — Mode 13h (320×200, 256 colors)
- 📦 **Tiny Size** — ~99KB kernel binary

## Project Structure

```
os_from_scratch/
├── Cargo.toml          # Rust project config
├── Makefile            # Build system
├── i686-evo.json       # Custom bare-metal target
├── linker.ld           # Kernel linker script
├── boot/
│   ├── stage1.asm      # MBR bootloader (512 bytes)
│   └── stage2.asm      # Protected mode + VGA setup
└── src/
    ├── lib.rs          # Kernel entry point
    ├── vga.rs          # VGA graphics driver
    ├── colors/         # Color system (Radix-inspired)
    │   ├── palette.rs  # ColorScale definitions
    │   ├── semantic.rs # Semantic tokens
    │   └── theme.rs    # Light/Dark themes
    └── shell/          # Command shell
        ├── terminal.rs # Terminal display
        ├── commands.rs # Built-in commands
        └── parser.rs   # Command parser
```

## Quick Start

### Prerequisites

- Rust nightly toolchain
- NASM assembler
- QEMU emulator
- GNU ld linker

### Build & Run

```bash
# Clone the repository
git clone https://github.com/yourusername/os_from_scratch.git
cd os_from_scratch

# Build the OS
make all

# Run in QEMU
make run
```

### Commands

| Command | Description |
|---------|-------------|
| `help` | Show available commands |
| `clear` | Clear the screen |
| `echo <text>` | Print text |
| `version` | Show OS version |
| `about` | About EvoOS |
| `color <0-15>` | Change text color |
| `mem` | Show memory info |
| `reboot` | Reboot system |
| `halt` | Shutdown system |

## Architecture

```
┌─────────────────────────────────────────┐
│              EvoOS                      │
├─────────────────────────────────────────┤
│  Shell (commands, terminal, parser)     │
├─────────────────────────────────────────┤
│  Colors (palette, semantic, theme)      │
├─────────────────────────────────────────┤
│  VGA Driver (Mode 13h, 320×200)         │
├─────────────────────────────────────────┤
│  Rust Kernel (no_std, no_main)          │
├─────────────────────────────────────────┤
│  Bootloader (Stage 1 + Stage 2)         │
└─────────────────────────────────────────┘
```

## Why EvoOS?

| Aspect | Linux | Windows | macOS | **EvoOS** |
|--------|-------|---------|-------|-------------|
| Language | C | C/C++ | C/Obj-C | **Rust** |
| Kernel Size | ~30MB | ~100MB | ~50MB | **~99KB** |
| Memory Safe | No | No | No | **Yes** |
| From Scratch | No* | No | No | **Yes** |

## License

MIT License - see [LICENSE](LICENSE)

## Contributing

Contributions are welcome! Feel free to open issues or submit PRs.

---

**EvoOS** — *Something completely new.*
