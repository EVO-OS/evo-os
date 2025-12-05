;==============================================================================
; NexusOS Bootloader - Stage 1 (MBR)
; Written in Assembly - loads Stage 2
;==============================================================================

[BITS 16]
[ORG 0x7C00]

STAGE2_LOAD_ADDR equ 0x7E00     ; Load stage 2 right after MBR

start:
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00
    sti

    ; Save boot drive
    mov [boot_drive], dl

    ; Print welcome
    mov si, msg_boot
    call print

    ; Load stage 2 (sectors 2-17)
    mov ah, 0x02        ; Read sectors
    mov al, 16          ; 16 sectors (8KB)
    mov ch, 0           ; Cylinder 0
    mov cl, 2           ; Sector 2
    mov dh, 0           ; Head 0
    mov dl, [boot_drive]
    mov bx, STAGE2_LOAD_ADDR
    int 0x13
    jc disk_error

    ; Jump to stage 2
    jmp STAGE2_LOAD_ADDR

disk_error:
    mov si, msg_err
    call print
    jmp $

print:
    lodsb
    test al, al
    jz .done
    mov ah, 0x0E
    int 0x10
    jmp print
.done:
    ret

boot_drive: db 0
msg_boot: db "NexusOS", 13, 10, 0
msg_err:  db "Disk Error!", 0

times 510-($-$$) db 0
dw 0xAA55
