#==============================================================================
# NexusOS Makefile - Rust Edition
# Build system for the Rust-based operating system
#==============================================================================

# Tools
ASM = nasm
QEMU = qemu-system-i386

# Rust target
TARGET = i686-nexus
RUST_TARGET_PATH = $(shell pwd)

# Directories
BUILD = build
BOOT = boot
SRC = src

# Output files
STAGE1 = $(BUILD)/stage1.bin
STAGE2 = $(BUILD)/stage2.bin
KERNEL = $(BUILD)/kernel.bin
OS_IMAGE = $(BUILD)/nexus.img

.PHONY: all clean run debug

all: $(BUILD) rust_kernel $(OS_IMAGE)
	@echo ""
	@echo "╔═══════════════════════════════════════════════════════╗"
	@echo "║  NexusOS (Rust Edition) built successfully!           ║"
	@echo "║  Run 'make run' to boot in QEMU                       ║"
	@echo "╚═══════════════════════════════════════════════════════╝"
	@echo ""

$(BUILD):
	mkdir -p $(BUILD)

# Build bootloaders
$(STAGE1): $(BOOT)/stage1.asm
	$(ASM) -f bin $< -o $@

$(STAGE2): $(BOOT)/stage2.asm
	$(ASM) -f bin $< -o $@

# Build Rust kernel
rust_kernel: $(BUILD)
	@echo "Building Rust kernel..."
	RUST_TARGET_PATH=$(RUST_TARGET_PATH) cargo build --release --target $(TARGET).json -Zbuild-std=core -Zbuild-std-features=compiler-builtins-mem
	cp target/$(TARGET)/release/libnexus_os.a $(BUILD)/kernel.a
	ld -m elf_i386 -T linker.ld -o $(BUILD)/kernel.elf $(BUILD)/kernel.a
	objcopy -O binary $(BUILD)/kernel.elf $(KERNEL)

# Create disk image
$(OS_IMAGE): $(STAGE1) $(STAGE2) rust_kernel
	@echo "Creating bootable disk image..."
	dd if=/dev/zero of=$@ bs=512 count=2880 2>/dev/null
	dd if=$(STAGE1) of=$@ bs=512 count=1 conv=notrunc 2>/dev/null
	dd if=$(STAGE2) of=$@ bs=512 seek=1 conv=notrunc 2>/dev/null
	dd if=$(KERNEL) of=$@ bs=512 seek=17 conv=notrunc 2>/dev/null
	@echo "Created: $@"

run: $(OS_IMAGE)
	@echo "Booting NexusOS in QEMU..."
	$(QEMU) -drive format=raw,file=$(OS_IMAGE),if=floppy -boot a -m 32M

debug: $(OS_IMAGE)
	$(QEMU) -drive format=raw,file=$(OS_IMAGE),if=floppy -boot a -m 32M -d int -no-reboot

clean:
	rm -rf $(BUILD) target
	cargo clean 2>/dev/null || true
