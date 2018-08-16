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
}