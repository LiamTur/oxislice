use core::cmp::min;
use crossterm::event::{
    read, 
    Event::{self, Key}, 
    KeyCode, KeyEvent, KeyModifiers, KeyEventKind,
    };
mod terminal;
use terminal::{Terminal, Size, Pos};
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");


#[derive(Copy, Clone, Default)]
pub struct Location {
    x: u16,
    y: u16,
}
#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::init().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        self.refresh()?;
        let Size{height,width} = Terminal::size()?;
        Terminal::move_m(Pos{x:height/2,y:width/2})?;
        Terminal::print("oxislice 1.0.0")?;
        loop {
            self.refresh()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    //move code here

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }

                    KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::PageDown
                    | KeyCode::PageUp
                    | KeyCode::End
                    | KeyCode::Home =>{
                        self.movepoint(*code);
                }
                _ => (), // add call to move function
            }
        }
    }
    fn refresh(&self) -> Result <(), std::io::Error>{
        Terminal::hide()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye my beloved")?;
        }
        else {
            Self::draw_rows()?;
            Terminal::move_m(Pos{x:0,y:0});
        }
        Terminal::show()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let Size { height, .. } = Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_line()?;
            if current_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), std::io::Error> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.width as usize;
        let len = welcome_message.len();
        let padding = (width - len) / 2;
        let spaces = " ".repeat(padding - 1);
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(&welcome_message)?;
        Ok(())
    }

    fn draw_empty_row() -> Result<(), std::io::Error> {
        Terminal::print("~")?;
        Ok(())
    }

    fn movepoint(&mut self, key: KeyCode) -> Result<(), std::io::Error> {
        let Location {mut x, mut y} = self.location;
        let Size {height, width} = Terminal::size()?;
        match key {
            KeyCode::Up => {
                if y == 0 {
                } else {
                    y+=1;
                }
            }
            KeyCode::Down =>
            {
                if y == height {
                } else {
                    y-=1;
                }
            }
            KeyCode::Left =>
            {
                if x == 0 {
                } else {
                    x-=1;
                }
            }
            KeyCode::Right =>
            {
                if x == width {
                } else {
                    x+=1;
                }
            }
            KeyCode::PageDown =>
            {
                y = height;
            }
            KeyCode::PageUp =>
            {
                y = 0;
            }
            KeyCode::End =>
            {
                x = width;
            }
            KeyCode::Home =>
            {
                x = 0;
            }
                _ => (),
        }
        self.location = Location {x, y};
        Ok(())
    }
}


