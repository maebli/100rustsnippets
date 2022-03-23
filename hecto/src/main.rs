use std::io::stdout;

use crossterm::event::poll;
use crossterm::{
    cursor::position,
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

use crossterm::event::KeyCode::Char;
use crossterm::style::Print;
use std::time::Duration;

fn print_events() -> Result<()> {
    loop {
        let event = read()?;
        if let Event::Key(x) = event {
            match x.code {
                Char('q') => return Ok(()),
                Char(x) => {
                    execute!(stdout(), Print(x.to_string()));
                },
                _Enter => {
                    execute!(stdout(), Print("\n\r".to_string()));
                }
                _ => {
                    execute!(
                        stdout(),
                        Print("special character".to_string())
                    );
                }
            }
        }
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;

    if let Err(e) = print_events() {
        println!("Error: {:?}\r", e);
    }

    disable_raw_mode()
}
