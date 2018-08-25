extern crate std;

pub const COMMAND_SIZE: usize = 3; //size of command in bytes

const EMPTY_CHAR: u8 = 95; // char used as empty spaces (_)

//commands
const CMD_INIT: u8 = 0;
const CMD_WRITE: u8 = 1;
const CMD_SETC: u8 = 2;
const CMD_CLEAR: u8 = 3;
const CMD_HOME: u8 = 4;
const CMD_INVALID: u8 = 255;

#[derive(Debug)]
pub struct Display {
    cols: usize,
    rows: usize,
    cursor_c: usize,
    cursor_r: usize,
    char_buffer: Vec<Vec<u8>>,
}

impl Display {
    pub fn new(cols: usize, rows: usize) -> Display {
        Display {
            cols,
            rows,
            cursor_c: 0,
            cursor_r: 0,
            char_buffer: vec![vec![EMPTY_CHAR; cols]; rows],
        }
    }

    pub fn write_byte(&mut self, val: u8) {
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

    pub fn set_cursor(&mut self, col: usize, row: usize) -> Result<(), std::io::Error> {
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

    pub fn home(&mut self) {
        self.cursor_c = 0;
        self.cursor_r = 0;
    }

    pub fn clear(&mut self) {
        self.char_buffer = vec![vec![EMPTY_CHAR; self.cols]; self.rows];
        self.home();
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

    pub fn get_buffer(&self) -> &Vec<Vec<u8>> {
        &self.char_buffer
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
                self.write_byte(byte_l);
                Ok(())
            }
            CMD_SETC => {
                self.set_cursor(byte_l as usize, byte_h as usize)?;
                Ok(())
            }
            CMD_CLEAR => {
                self.clear();
                Ok(())
            }
            CMD_HOME => {
                self.home();
                Ok(())
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Invalid command number",
            )),
        }
    }
}
