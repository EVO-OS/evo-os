//! Built-in Shell Commands

use super::terminal::terminal;
use crate::colors::theme::CURRENT_THEME;

/// Command result
pub enum CommandResult {
    Ok,
    Error(&'static str),
    Exit,
    NotFound,
}

/// Execute a command by name
pub fn execute(cmd: &str, args: &[&str]) -> CommandResult {
    match cmd {
        "help" => cmd_help(),
        "clear" | "cls" => cmd_clear(),
        "echo" => cmd_echo(args),
        "version" | "ver" => cmd_version(),
        "about" => cmd_about(),
        "color" => cmd_color(args),
        "time" => cmd_time(),
        "mem" | "memory" => cmd_memory(),
        "reboot" => cmd_reboot(),
        "halt" | "shutdown" => cmd_halt(),
        "" => CommandResult::Ok,
        _ => CommandResult::NotFound,
    }
}

fn cmd_help() -> CommandResult {
    let term = terminal();
    term.set_color(CURRENT_THEME.terminal_prompt);
    term.println("NexusOS Shell Commands:");
    term.set_color(CURRENT_THEME.terminal_text);
    term.println("  help          - Show this help");
    term.println("  clear, cls    - Clear screen");
    term.println("  echo <text>   - Print text");
    term.println("  version       - Show OS version");
    term.println("  about         - About NexusOS");
    term.println("  color <n>     - Set text color (0-15)");
    term.println("  time          - Show system time");
    term.println("  mem           - Show memory info");
    term.println("  reboot        - Reboot system");
    term.println("  halt          - Shutdown system");
    CommandResult::Ok
}

fn cmd_clear() -> CommandResult {
    terminal().clear();
    CommandResult::Ok
}

fn cmd_echo(args: &[&str]) -> CommandResult {
    let term = terminal();
    for (i, arg) in args.iter().enumerate() {
        if i > 0 { term.print(" "); }
        term.print(arg);
    }
    term.println("");
    CommandResult::Ok
}

fn cmd_version() -> CommandResult {
    let term = terminal();
    term.println("NexusOS v0.1.0 (Rust Edition)");
    term.println("Built with Rust - 100% Memory Safe");
    CommandResult::Ok
}

fn cmd_about() -> CommandResult {
    let term = terminal();
    term.set_color(10); // Light green
    term.println("╭────────────────────────────────────╮");
    term.println("│           N E X U S O S            │");
    term.println("│   A New Operating System from      │");
    term.println("│   Scratch - Written in RUST        │");
    term.println("├────────────────────────────────────┤");
    term.set_color(CURRENT_THEME.terminal_text);
    term.println("│  • Not Linux                       │");
    term.println("│  • Not Windows                     │");
    term.println("│  • Not macOS                       │");
    term.println("│  • Something completely NEW!       │");
    term.set_color(10);
    term.println("╰────────────────────────────────────╯");
    term.set_color(CURRENT_THEME.terminal_text);
    CommandResult::Ok
}

fn cmd_color(args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: color <0-15>");
    }
    
    // Parse color number
    let color = match args[0] {
        "0" => 0, "1" => 1, "2" => 2, "3" => 3,
        "4" => 4, "5" => 5, "6" => 6, "7" => 7,
        "8" => 8, "9" => 9, "10" => 10, "11" => 11,
        "12" => 12, "13" => 13, "14" => 14, "15" => 15,
        _ => return CommandResult::Error("Invalid color (0-15)"),
    };
    
    terminal().set_color(color);
    terminal().println("Color changed!");
    CommandResult::Ok
}

fn cmd_time() -> CommandResult {
    // TODO: Read from RTC
    terminal().println("System time: 12:00:00 (RTC not implemented)");
    CommandResult::Ok
}

fn cmd_memory() -> CommandResult {
    let term = terminal();
    term.println("Memory Information:");
    term.println("  Total:     32 MB");
    term.println("  Kernel:    ~64 KB");
    term.println("  Available: ~31 MB");
    CommandResult::Ok
}

fn cmd_reboot() -> CommandResult {
    terminal().println("Rebooting...");
    // Triple fault to reboot
    unsafe {
        core::arch::asm!(
            "lidt [0]",
            "int 0"
        );
    }
    CommandResult::Ok
}

fn cmd_halt() -> CommandResult {
    terminal().println("System halting...");
    CommandResult::Exit
}
