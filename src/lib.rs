//! NexusOS Kernel - Written in Rust
//! A completely new operating system from scratch

#![no_std]
#![no_main]

mod vga;
mod colors;
mod shell;

use core::panic::PanicInfo;
use vga::SCREEN;
use colors::theme::CURRENT_THEME;
use shell::terminal;

/// Kernel entry point - called from bootloader
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize terminal with shell
    let term = terminal();
    term.init();
    
    // Show prompt
    term.prompt();
    
    // Demo: Run some commands
    demo_shell();

    // Halt forever
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}

/// Demo shell functionality
fn demo_shell() {
    let term = terminal();
    
    // Simulate typing "about"
    term.println("about");
    let _ = shell::commands::execute("about", &[]);
    term.prompt();
    
    // Simulate typing "help"
    term.println("help");
    let _ = shell::commands::execute("help", &[]);
    term.prompt();
    
    // Simulate typing "version"
    term.println("version");
    let _ = shell::commands::execute("version", &[]);
    term.prompt();
}

/// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let screen = SCREEN.lock();
    screen.clear_with_color(4); // Red
    screen.draw_string_raw(80, 90, "KERNEL PANIC!", 15, 4);
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}
