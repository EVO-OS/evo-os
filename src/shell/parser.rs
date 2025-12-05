//! Command Line Parser

/// Maximum command length
const MAX_CMD_LEN: usize = 128;
/// Maximum number of arguments
const MAX_ARGS: usize = 16;

/// Parse a command line into command and arguments
pub fn parse_line(line: &str) -> (&str, [&str; MAX_ARGS], usize) {
    let mut args: [&str; MAX_ARGS] = [""; MAX_ARGS];
    let mut arg_count = 0;
    let mut cmd = "";
    
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return ("", args, 0);
    }
    
    // Split by spaces
    let mut in_word = false;
    let mut word_start = 0;
    
    for (i, c) in trimmed.char_indices() {
        if c == ' ' {
            if in_word {
                if arg_count == 0 {
                    cmd = &trimmed[word_start..i];
                } else if arg_count <= MAX_ARGS {
                    args[arg_count - 1] = &trimmed[word_start..i];
                }
                arg_count += 1;
                in_word = false;
            }
        } else {
            if !in_word {
                word_start = i;
                in_word = true;
            }
        }
    }
    
    // Handle last word
    if in_word {
        if arg_count == 0 {
            cmd = &trimmed[word_start..];
        } else if arg_count <= MAX_ARGS {
            args[arg_count - 1] = &trimmed[word_start..];
        }
        arg_count += 1;
    }
    
    // arg_count includes the command, so subtract 1 for actual args
    let actual_args = if arg_count > 1 { arg_count - 1 } else { 0 };
    
    (cmd, args, actual_args)
}

/// Input buffer for command line
pub struct InputBuffer {
    buffer: [u8; MAX_CMD_LEN],
    len: usize,
}

impl InputBuffer {
    pub const fn new() -> Self {
        Self {
            buffer: [0; MAX_CMD_LEN],
            len: 0,
        }
    }
    
    pub fn clear(&mut self) {
        self.len = 0;
    }
    
    pub fn push(&mut self, c: u8) -> bool {
        if self.len < MAX_CMD_LEN - 1 {
            self.buffer[self.len] = c;
            self.len += 1;
            true
        } else {
            false
        }
    }
    
    pub fn pop(&mut self) -> bool {
        if self.len > 0 {
            self.len -= 1;
            true
        } else {
            false
        }
    }
    
    pub fn as_str(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(&self.buffer[..self.len])
        }
    }
}
