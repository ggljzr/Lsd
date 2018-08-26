extern crate std;

pub const COMMAND_SIZE: usize = 3; //size of command in bytes

const EMPTY_CHAR: u8 = 95; // char used as empty spaces (_)
const BLOCK_CHAR: u8 = 35;

//commands
const CMD_INIT: u8 = 0;
const CMD_WRITE: u8 = 1;
const CMD_SETC: u8 = 2;
const CMD_CLEAR: u8 = 3;
const CMD_HOME: u8 = 4;
const CMD_CURSOR: u8 = 5;
const CMD_NOCURSOR: u8 = 6;
const CMD_INVALID: u8 = 255;

#[derive(Debug)]
pub struct Display {
    cols: usize,
    rows: usize,
    cursor_c: usize,
    cursor_r: usize,
    blink: bool,
    cursor: bool,
    char_buffer: Vec<Vec<u8>>,
}

impl Display {
    pub fn new(cols: usize, rows: usize) -> Display {
        Display {
            cols,
            rows,
            cursor_c: 0,
            cursor_r: 0,
            blink: false,
            cursor: false,
            char_buffer: vec![vec![EMPTY_CHAR; cols]; rows],
        }
    }

    fn _write_byte(&mut self, val: u8) {
        self.char_buffer[self.cursor_r][self.cursor_c] = val;

        self.cursor_c += 1;
        if self.cursor_c == self.cols {
            self.cursor_c = 0;
            self.cursor_r += 1;

            if self.cursor_r == self.rows {
                self.cursor_r = 0;
            }
        }
    }

    fn _set_cursor(&mut self, col: usize, row: usize) -> Result<(), std::io::Error> {
        if col < self.cols && row < self.rows {
            self.cursor_c = col;
            self.cursor_r = row;
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Invalid cursor position",
            ))
        }
    }

    fn _home(&mut self) {
        self.cursor_c = 0;
        self.cursor_r = 0;
    }

    fn _clear(&mut self) {
        self.char_buffer = vec![vec![EMPTY_CHAR; self.cols]; self.rows];
        self._home();
    }

    fn _scroll_right(&mut self) {
        let mut c = EMPTY_CHAR;

        if let Some(v) = self.char_buffer[self.rows - 1].pop() {
            c = v;
        }

        for row in &mut self.char_buffer {
            row.insert(0, c);
            if let Some(v) = row.pop() {
                c = v;
            }
        }

        self.char_buffer[self.rows - 1].push(c);
    }

    pub fn print_disp(&self) {
        println!("{}", self.to_string());
    }

    pub fn to_string(&self) -> String {
        let mut res = String::new();

        for v in &self.char_buffer {
            for c in v {
                res.push(*c as char);
            }
            res.push(' ');
        }
        res
    }

    pub fn get_draw_data(&self) -> Vec<Vec<u8>> {
        let mut data = self.char_buffer.clone();
        if self.cursor == true {
            data[self.cursor_r][self.cursor_c] = BLOCK_CHAR;
        }
        
        data
    }

    pub fn exec_command(&mut self, cmd: &[u8]) -> Result<(), std::io::Error> {
        let mut cmd_num = CMD_INVALID;

        if let Some(v) = cmd.get(0) {
            cmd_num = *v;
        }

        let mut byte_l = 0;
        let mut byte_h = 0;

        if let Some(v) = cmd.get(1) {
            byte_l = *v;
        }

        if let Some(v) = cmd.get(2) {
            byte_h = *v;
        }

        match cmd_num {
            CMD_INIT => {
                //TODO: init command
                Ok(())
            }
            CMD_WRITE => {
                self._write_byte(byte_l);
                Ok(())
            }
            CMD_SETC => {
                self._set_cursor(byte_l as usize, byte_h as usize)?;
                Ok(())
            }
            CMD_CLEAR => {
                self._clear();
                Ok(())
            }
            CMD_HOME => {
                self._home();
                Ok(())
            }
            CMD_CURSOR => {
                self.cursor = true;
                Ok(())
            }
            CMD_NOCURSOR => {
                self.cursor = false;
                Ok(())
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Invalid command number",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    /*
    Fills display with capital letters, then makes two
    shifts to the right, checks last character on display
    */
    fn test_scroll_right() {
        let mut td = super::Display::new(16, 2);

        for i in (0..32) {
            td._write_byte(65 + (i % 26) );
        }    
    
        let last_char = td.char_buffer[td.rows - 1][td.cols - 1];
        assert_eq!(last_char, 'F' as u8);
        td._scroll_right();
        td._scroll_right();
        let last_char = td.char_buffer[td.rows - 1][td.cols - 1];
        assert_eq!(last_char, 'D' as u8);

    }
}