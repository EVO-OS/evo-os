---
description: Run EVO-OS in QEMU for testing and debugging
---

# Test EVO-OS in QEMU

This workflow boots EVO-OS in QEMU and validates basic functionality.

## Prerequisites

- EVO-OS must have been built first (`/build` workflow or `make all`)
- QEMU must be installed: `sudo apt install qemu-system-x86`

## Steps

1. Verify the disk image exists:
```bash
ls -lh build/evo.img
```

// turbo
2. Run EVO-OS in QEMU (interactive — boots and runs):
```bash
qemu-system-i386 \
  -drive format=raw,file=build/evo.img,if=floppy \
  -boot a \
  -m 32M \
  -display gtk
```

3. To run QEMU headlessly (for CI / automated testing):
```bash
qemu-system-i386 \
  -drive format=raw,file=build/evo.img,if=floppy \
  -boot a \
  -m 32M \
  -display none \
  -serial stdio \
  -no-reboot \
  -d int \
  2>&1 | tee build/qemu_test.log
```

4. Check log for expected boot messages:
```bash
grep -E "(EvoOS|Hello|Booting|kernel)" build/qemu_test.log || echo "Boot output not found"
```

## Debug with GDB

// turbo
5. Start QEMU with GDB server (does not start until GDB connects):
```bash
qemu-system-i386 \
  -drive format=raw,file=build/evo.img,if=floppy \
  -boot a \
  -m 32M \
  -s -S \
  -display gtk &
```

6. Connect GDB (in a separate terminal):
```bash
gdb build/kernel.elf
```

Then inside GDB:
```
(gdb) target remote :1234
(gdb) break kernel_main
(gdb) continue
(gdb) info registers
(gdb) x/10i $pc
```

## Expected Boot Sequence

A successful boot should show in the QEMU window:
1. Blank screen briefly (BIOS POST)
2. "Booting EvoOS..." from Stage 1
3. Rust kernel initializes (VGA output)
4. Color palette demonstration
5. Shell prompt appears: `evo> `

## Common Issues

- **QEMU shows "Boot failed"** → disk image not found or Stage 1 is bad. Rebuild with `make all`.
- **Screen stays black** → Stage 2 may have crashed. Run with `-d int -no-reboot` to see interrupts.
- **"Triple fault" in GDB** → likely a paging setup issue in Stage 2. Check page table initialization.
