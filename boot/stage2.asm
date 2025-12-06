;==============================================================================
; EvoOS Bootloader - Stage 2
; Sets up VGA graphics, protected mode, and jumps to Rust kernel
;==============================================================================

[BITS 16]
[ORG 0x7E00]

KERNEL_ADDR equ 0x100000    ; 1MB - kernel load address

stage2_start:
    ; Set VGA Mode 13h (320x200, 256 colors) - MUST do before protected mode!
    mov ax, 0x0013
    int 0x10

    ; Fill screen with blue as immediate visual feedback
    mov ax, 0xA000
    mov es, ax
    xor di, di
    mov cx, 32000
    mov ax, 0x0101      ; Blue
    rep stosw

    ; Enable A20
    in al, 0x92
    or al, 2
    out 0x92, al

    ; Load kernel to 0x10000 (will copy to 1MB later)
    mov ax, 0x1000
    mov es, ax
    xor bx, bx
    mov ah, 0x02
    mov al, 127         ; Load 127 sectors (~64KB)
    mov ch, 0
    mov cl, 18          ; Start from sector 18
    mov dh, 0
    mov dl, 0x00        ; Floppy
    int 0x13

    ; Load GDT and enter protected mode
    cli
    lgdt [gdt_desc]
    mov eax, cr0
    or eax, 1
    mov cr0, eax
    jmp 0x08:protected_mode

[BITS 32]
protected_mode:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    mov esp, 0x90000

    ; Copy kernel from 0x10000 to 0x100000
    mov esi, 0x10000
    mov edi, KERNEL_ADDR
    mov ecx, 16384      ; 64KB / 4 = 16384 dwords
    rep movsd

    ; Jump to Rust kernel!
    call KERNEL_ADDR

    ; Should never return
    cli
    hlt
    jmp $

;==============================================================================
; GDT
;==============================================================================
gdt_start:
    dq 0                            ; Null descriptor
gdt_code:
    dw 0xFFFF, 0x0000               ; Code segment
    db 0x00, 0x9A, 0xCF, 0x00
gdt_data:
    dw 0xFFFF, 0x0000               ; Data segment
    db 0x00, 0x92, 0xCF, 0x00
gdt_end:

gdt_desc:
    dw gdt_end - gdt_start - 1
    dd gdt_start + 0x7E00           ; Absolute address

times 8192-($-$$) db 0
