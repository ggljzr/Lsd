extern crate std;

#[derive(Debug)]
pub enum Command {
    INIT { cols: u8, rows: u8 }, //command for display initialization
    WRITE(u8),                   //writes byte on screen, increments cursor
    SET_C { col: u8, row: u8 },  //sets screen to position
    CLEAR,                       //clears display
    HOME,                        //sets cursor to 0 0
}

pub fn parse_command(data: &Vec<u8>) -> Result<Command, std::io::Error> {
    let cmd_num = data[0];

    match cmd_num {
        0 => {
            let cols = *data.get(1).unwrap(); // TODO: error handling
            let rows = *data.get(2).unwrap();
            Ok(Command::INIT { cols, rows})
        },
        1 => {
            let val = *data.get(1).unwrap();
            Ok(Command::WRITE(val))
        }
        _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid command number"))
    }
}

const EMPTY_CHAR: u8 = 95; // char used as empty spaces (_)

#[derive(Debug)]
pub struct Display {

    cols: u8,
    rows: u8,
    cursor_c: u8,
    cursor_r: u8,
    char_buffer: Vec<u8>,
}

impl Display {
    pub fn new(cols: u8, rows: u8) -> Display {
        Display {
            cols,
            rows,
            cursor_c: 0,
            cursor_r: 0,
            char_buffer: vec![EMPTY_CHAR; (cols * rows) as usize],
        }
    }

    pub fn write_byte(&mut self, val: u8) {
        let i = (self.cursor_c + (self.cursor_r * self.cols)) as usize;
        self.char_buffer[i] = val;

        self.cursor_c += 1;
        if self.cursor_c == self.cols {
            self.cursor_c = 0;
            self.cursor_r += 1;

            if self.cursor_r == self.rows {
                self.cursor_r = 0;
            }
        }
    }

    pub fn set_cursor(&mut self, col: u8, row: u8) -> Result<(), std::io::Error> {
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
        self.char_buffer = vec![EMPTY_CHAR; (self.cols * self.rows) as usize];
        self.home();
    }

    pub fn print_disp(&self) {
        println!("{}", self.to_string());
    }

    pub fn to_string(&self) -> String {
        let mut res = String::new();

        for (i, byte) in self.char_buffer.iter().enumerate() {
            //print!("{}", *byte as char);
            res.push(*byte as char);
            if i == (self.cols - 1) as usize {
                res.push(' ');
            }
        }
        res
    }
}
