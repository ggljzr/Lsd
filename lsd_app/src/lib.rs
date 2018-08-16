#[derive(Debug)]
pub enum Command {
    INIT { cols: u8, rows: u8 }, //command for display initialization
    WRITE(u8),                   //writes byte on screen, increments cursor
    SET_C { col: u8, row: u8 },  //sets screen to position
    CLEAR,                       //clears display
    HOME,                        //sets cursor to 0 0
}

pub fn parse_command(data: &Vec<u8>) -> Result<Command, std::io::Error> {
    Ok(Command::INIT { cols: 16, rows: 2 })
}

#[derive(Debug)]
pub struct Display {
    cols: u8,
    rows: u8,
    cursor_c: u8,
    cursor_r: u8,
    char_buffer: Vec<u8>
}

impl Display {
    pub fn new(cols: u8, rows: u8) -> Display {
        Display {
            cols, rows,
            cursor_c: 0, cursor_r: 0,
            char_buffer: vec![0; (cols * rows) as usize]
        }
    }

    pub fn write_byte(&mut self, val: u8) -> Result<(), std::io::Error> {        
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

        Ok(())
    }

    pub fn set_cursor(&mut self, col: u8, row: u8) -> Result<(), std::io::Error> {
        if col < self.cols && row < self.rows {
            self.cursor_c = col;
            self.cursor_r = row;
            Ok(())
        }
        else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid cursor position"))
        }
    }

    pub fn print_disp(&self) {
        println!("cols: {}, rows: {}", self.cols, self.rows);

        for (i, byte) in self.char_buffer.iter().enumerate() {
            print!("{}", *byte as char);
            if i == (self.cols - 1) as usize {
                println!()
            }
        }
    }
}