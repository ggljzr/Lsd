extern crate std;

#[derive(Debug)]
pub enum Command {
    INIT { cols: u8, rows: u8 }, //command for display initialization
    WRITE(u8),                   //writes byte on screen, increments cursor
    SETC { col: u8, row: u8 },   //sets screen to position
    CLEAR,                       //clears display
    HOME,                        //sets cursor to 0 0
}

pub fn parse_command(data: &[u8]) -> Result<Command, std::io::Error> {
    let mut cmd_num = -1;

    if let Some(v) = data.get(0) {
        cmd_num = *v as i8;
    }

    match cmd_num {
        0 => {
            let cols = *data.get(1).unwrap(); // TODO: error handling
            let rows = *data.get(2).unwrap();
            Ok(Command::INIT { cols, rows })
        }
        1 => {
            let val = *data.get(1).unwrap();
            Ok(Command::WRITE(val))
        }
        2 => {
            let col = *data.get(1).unwrap();
            let row = *data.get(2).unwrap();
            Ok(Command::SETC { col, row})
        }
        3 => Ok(Command::CLEAR),
        4 => Ok(Command::HOME),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Invalid command number",
        )),
    }
}

const EMPTY_CHAR: u8 = 95; // char used as empty spaces (_)

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
}
