use std::io::{self, Write};

pub struct Display {
    message: String,
}

impl Display {
    pub fn new() -> Display {
        Self {
            message: "".to_string(),
        }
    }

    fn println(&self) {
        println!("{}", self.message);
        if let Err(e) = io::stdout().flush() {
            println!("Error printing to console: {e}");
        }
    }

    fn add_character(&mut self, c: char) {
        self.message.push(c);
    }

    fn clear(&mut self) {
        self.message = "".to_string();
    }

    pub fn write(&mut self, data: u8) {
        match data {
            // the idea was to create a display thats deletable but too lazy rn
            0x80 => self.clear(),
            _ => {
                self.add_character(data as char);
                print!("{}", data as char);
                if let Err(e) = io::stdout().flush() {
                    println!("Error printing to console: {e}");
                }
            }
        }
        //self.println();
    }
}
