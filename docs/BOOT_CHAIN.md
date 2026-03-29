# EVO-OS Boot Chain — Detailed Reference

This document explains every step in the EVO-OS boot sequence, from the moment power is applied to the moment a user sees the desktop. Each step maps to a specific git submodule repo.

---

## Overview Diagram

```
                         POWER ON
                            │
                            ▼
          ┌─────────────────────────────────┐
          │  CPU Reset Vector               │
          │  • Jumps to 0xFFFF0 (BIOS) or  │
          │    firmware ROM address (UEFI)  │
          └────────────────┬────────────────┘
                           │
                           ▼
          ┌─────────────────────────────────┐   [frimware repo]
          │  UEFI / BIOS Firmware           │
          │  1. POST (Power-On Self-Test)   │
          │  2. RAM training & init         │
          │  3. CPU microcode load          │
          │  4. PCIe link setup             │
          │  5. Clock/voltage config        │
          │  6. ACPI table construction     │
          │  7. Secure Boot key enroll      │
          │  8. Scan for bootable device    │
          └────────────────┬────────────────┘
                           │
                           ▼
          ┌─────────────────────────────────┐   [frimware repo]
          │  FOTA Engine                    │
          │  (Firmware Over-The-Air)        │
          │  • Check for pending updates    │
          │  • Verify signature on pkg      │
          │  • Apply to inactive A/B slot   │
          │  • Mark slot active             │
          │  • Rollback if verify fails     │
          │  NOTE: On desktop x86, this is  │
          │  a no-op unless update pending  │
          └────────────────┬────────────────┘
                           │
                           ▼
          ┌─────────────────────────────────┐   [bootloader repo]
          │  Stage 1 Bootloader (512 bytes) │
          │  • MBR / UEFI stub entry        │
          │  • Loaded at 0x7C00 by BIOS     │
          │  • Reads Stage 2 from disk      │
          │  • Jumps to Stage 2 at 0x1000   │
          └────────────────┬────────────────┘
                           │
                           ▼
          ┌─────────────────────────────────┐   [bootloader repo]
          │  Stage 2 Bootloader             │
          │  CPU Mode Transitions:          │
          │  16-bit Real Mode               │
          │    → Enable A20 line            │
          │    → Switch to 32-bit Protected │
          │    → Set up 64-bit page tables  │
          │    → Switch to 64-bit Long Mode │
          │  Then:                          │
          │  • Parse kernel ELF from disk   │
          │  • Set up boot info struct      │
          │    (memory map, framebuffer)    │
          │  • Jump to kernel entry point   │
          └────────────────┬────────────────┘
                           │
                           ▼
          ┌─────────────────────────────────┐   [Kernel repo]
          │  EVO-OS Kernel Entry            │
          │  Rust no_std kernel_main()      │
          │  1. Physical memory manager     │
          │  2. Set up IDT (interrupts)     │
          │  3. Initialize PIC/APIC         │
          │  4. Virtual memory + paging     │
          │  5. Heap allocator (kmalloc)    │
          │  6. Device driver init          │
          │  7. Scheduler + process table   │
          │  8. VFS mount root filesystem   │
          │  9. Launch init process (PID 1) │
          └────────────────┬────────────────┘
                           │
                           ▼
          ┌─────────────────────────────────┐   [drivers repo] [bsp repo]
          │  Driver Initialization Phase    │
          │  • Keyboard driver              │
          │  • VGA / framebuffer driver     │
          │  • ATA / NVMe storage driver    │
          │  • Network (Ethernet) driver    │
          │  • USB controller driver        │
          │  BSP provides HW abstraction    │
          └────────────────┬────────────────┘
                           │
                           ▼
          ┌─────────────────────────────────┐   [libc repo] [filesystem repo]
          │  Userspace Runtime Setup        │
          │  • Mount root FS (FAT32/ext4)   │
          │  • libc loaded into userspace   │
          │  • /sbin/init (PID 1) starts    │
          │  • Mounts all other FS          │
          │  • Starts system services       │
          └────────────────┬────────────────┘
                           │
                           ▼
          ┌─────────────────────────────────┐   [GUI repo] [apps repo]
          │  Desktop / GUI                  │
          │  • Framebuffer compositor init  │
          │  • Window manager starts        │
          │  • Login screen or shell        │
          │  • User applications load       │
          └─────────────────────────────────┘
```

---

## CPU Mode Transitions (x86-64)

The bootloader must take the CPU through three privilege/address modes before the 64-bit kernel can run:

```
POWER ON
│  CPU starts in 16-bit Real Mode
│  • Max 1 MB address space
│  • No memory protection
│  • BIOS interrupts available
▼
Enable A20 Line
│  • Unlocks access above 1 MB
│  • Via keyboard controller or Fast A20
▼
Set up GDT (Global Descriptor Table)
│  • Defines code/data segments
│  • Required for protected mode
▼
Switch to 32-bit Protected Mode
│  • Set CR0.PE = 1
│  • 4 GB address space
│  • Memory protection enabled
│  • BIOS interrupts no longer available
▼
Set up 4-level Page Tables (PML4)
│  • Identity-map kernel load address
│  • Map kernel to higher half virtual address
│  • Set CR3 = PML4 base address
▼
Enable PAE + Long Mode in EFER MSR
│  • Set EFER.LME = 1
│  • Set CR4.PAE = 1
▼
Switch to 64-bit Long Mode
│  • Set CR0.PG = 1 (enable paging) → activates long mode
│  • Far jump to 64-bit code segment
▼
64-bit mode active — jump to kernel_main()
```

---

## Memory Map at Handoff

When the bootloader calls `kernel_main()`, it passes a boot info struct containing:

| Field | Description |
|-------|-------------|
| `memory_map` | Array of physical memory regions (free, reserved, ACPI, etc.) |
| `framebuffer` | Address, width, height, pitch, and pixel format of the display |
| `kernel_start` | Physical start address of the kernel binary |
| `kernel_end` | Physical end address of the kernel binary |
| `rsdp_addr` | ACPI Root System Description Pointer address |
| `cmdline` | Kernel command-line string |

---

## FOTA Update Flow (Embedded / Android Targets)

```
Boot start
    │
    ▼
Read NVRAM flags: "update_pending" ?
    │
    ├── NO ──► Normal boot (continue to Stage 1)
    │
    └── YES ─► FOTA Update Path
                    │
                    ▼
              Download/verify update package
              (cryptographic signature check)
                    │
                    ├── FAIL ──► Clear flag, boot from active slot
                    │
                    └── PASS ─► Write to inactive slot (A or B)
                                    │
                                    ▼
                              Set inactive slot as "next boot"
                                    │
                                    ▼
                              Reboot
                                    │
                                    ▼
                              Boot from new slot
                                    │
                              Run post-update verification
                                    │
                              ├── FAIL ──► Roll back to old slot
                              └── PASS ──► Mark slot as stable
```

---

## Secure Boot Chain of Trust

```
OEM Key (in flash)
    │ verifies signature of
    ▼
UEFI Firmware (frimware repo)
    │ verifies signature of
    ▼
Bootloader binary (bootloader repo)
    │ verifies signature/hash of
    ▼
Kernel image (Kernel repo)
    │
    ▼ (runtime — optional)
Driver signatures checked at load time
```

Each link in this chain is cryptographically verified before execution. If any step fails, the system either refuses to boot (strict mode) or logs a TPM PCR violation and boots in a degraded mode.

---

*See also:*
- [ARCHITECTURE.md](../ARCHITECTURE.md) — Full system architecture
- [bootloader/docs/STAGES.md](../../bootloader/docs/STAGES.md) — Bootloader stage detail
- [frimware/ARCHITECTURE.md](../../frimware/ARCHITECTURE.md) — Firmware internals
