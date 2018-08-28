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
const CMD_SCROLL_RIGHT: u8 = 7;
const CMD_SCROLL_LEFT: u8 = 8;
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

    fn _move_cursor_right(&mut self) {
        self.cursor_c += 1;
        if self.cursor_c == self.cols {
            self.cursor_c = 0;
            self.cursor_r += 1;

            if self.cursor_r == self.rows {
                self.cursor_r = 0;
            }
        };
    }

    fn _move_cursor_left(&mut self) {
        if self.cursor_c > 0 {
            self.cursor_c -= 1;
        } else {
            self.cursor_c = self.cols - 1;
            if self.cursor_r > 0 {
                self.cursor_r -= 1;
            } else {
                self.cursor_r = self.rows - 1;
            }
        }
    }

    fn _write_byte(&mut self, val: u8) {
        self.char_buffer[self.cursor_r][self.cursor_c] = val;
        self._move_cursor_right();
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
        let mut c = self.char_buffer[self.rows - 1][self.cols - 1];

        for row in &mut self.char_buffer {
            row.insert(0, c);
            if let Some(v) = row.pop() {
                c = v;
            }
        }

        self._move_cursor_right();
    }

    fn _scroll_left(&mut self) {
        let mut c = self.char_buffer[0][0];

        {
            let rev_iter = self.char_buffer.iter_mut().rev();

            for mut row in rev_iter {
                row.push(c);
                c = row.remove(0);
            }
        }

        self._move_cursor_left();
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
            },
            CMD_WRITE => {
                self._write_byte(byte_l);
                Ok(())
            },
            CMD_SETC => {
                self._set_cursor(byte_l as usize, byte_h as usize)?;
                Ok(())
            },
            CMD_CLEAR => {
                self._clear();
                Ok(())
            },
            CMD_HOME => {
                self._home();
                Ok(())
            },
            CMD_CURSOR => {
                self.cursor = true;
                Ok(())
            },
            CMD_NOCURSOR => {
                self.cursor = false;
                Ok(())
            },
            CMD_SCROLL_RIGHT => {
                self._scroll_right();
                Ok(())
            },
            CMD_SCROLL_LEFT => {
                self._scroll_left();
                Ok(())
            },
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Invalid command number",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    #[test]
    /*
    Simple write test.
    */
    fn test_write_byte() {
        let mut td = super::Display::new(16, 2);
        td._write_byte('A' as u8);
        td._write_byte('B' as u8);
        td._write_byte('C' as u8);

        assert_eq!(td.char_buffer[0][0], 'A' as u8);
        assert_eq!(td.char_buffer[0][1], 'B' as u8);
        assert_eq!(td.char_buffer[0][2], 'C' as u8);
    }

    #[test]
    /*
    Tests if character is written to correct position
    after valid _set_cursor() call.
     */
    fn test_set_cursor() {
        let mut td = super::Display::new(16, 2);

        //test valid cursor position
        let res = td._set_cursor(5, 1);
        assert_eq!((), res.unwrap());

        td._write_byte('A' as u8);
        assert_eq!(td.char_buffer[1][5], 'A' as u8);
    }

    /*
    Tests if appropriate error is returned
    if cursor is set to invalid position
    (i. e. that exceeding display's rows or cols)
    */
    #[test]
    fn test_set_cursor_invalid() {
        let mut td = super::Display::new(16, 2);
        match td._set_cursor(17, 5) {
            Ok(_) => assert!(false, "Expected error!"),
            Err(e) => assert_eq!(e.kind(), std::io::ErrorKind::Other),
        }
    }

    /*
    Tests for correct cursor overflowing
    in move_cursor methods.
    */
    #[test]
    fn test_move_cursor_right() {
        let mut td = super::Display::new(16, 2);
        //set cursor to last position
        let _res = td._set_cursor(15, 1);
        td._move_cursor_right();
        //cursor now should be at home position
        assert_eq!(0, td.cursor_c);
        assert_eq!(0, td.cursor_r);
    }

    #[test]
    fn test_move_cursor_left() {
        let mut td = super::Display::new(16, 2);
        //set cursor to home position
        let _res = td._set_cursor(0, 0);
        td._move_cursor_left();
        //cursor now should be at the last position
        assert_eq!(15, td.cursor_c);
        assert_eq!(1, td.cursor_r);
    }

    /*
    Fills display with capital letters, then makes two
    shifts to the right, checks last character on display.
    */
    #[test]
    fn test_scroll_right() {
        let mut td = super::Display::new(16, 2);

        for i in 0..32 {
            td._write_byte(65 + (i % 26));
        }

        let last_char = td.char_buffer[td.rows - 1][td.cols - 1];
        assert_eq!(last_char, 'F' as u8);
        td._scroll_right();
        td._scroll_right();
        let last_char = td.char_buffer[td.rows - 1][td.cols - 1];
        assert_eq!(last_char, 'D' as u8);
    }

    /*
    Similar to test_scroll_right() test.
    */
    #[test]
    fn test_scroll_left() {
        let mut td = super::Display::new(16, 2);

        td._write_byte('A' as u8);
        td._write_byte('B' as u8);

        let _res = td._set_cursor(0, 1);
        td._write_byte('C' as u8);

        td._scroll_left();

        td.print_disp();

        let a_char = td.char_buffer[td.rows - 1][td.cols - 1];
        let b_char = td.char_buffer[0][0];
        let c_char = td.char_buffer[0][td.cols - 1];

        assert_eq!(a_char, 'A' as u8);
        assert_eq!(b_char, 'B' as u8);
        assert_eq!(c_char, 'C' as u8);
    }
}
