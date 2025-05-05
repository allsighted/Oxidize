use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    const LOREM_IPSUM: &str = "START Lorem ipsum dolor sit amet consectetur adipiscing elit Nullam eget justo vel odio efficitur varius Cras sagittis purus at tempor sollicitudin nulla orci venenatis enim non faucibus ante est vel magna END";
    let words: Vec<&str> = LOREM_IPSUM.split_whitespace().collect();
    let mut word_index = 0;
    let mut displayed_text = String::new();
    
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}Press RIGHT ARROW to see Lorem Ipsum words. Press q to exit.{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();

    for k in stdin.keys() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::CurrentLine
        )
        .unwrap();

        match k.as_ref().unwrap() {
            Key::Char('q') => break,
            Key::Char('c') | Key::Char('C') => {
                displayed_text.clear();
                word_index = 0;
                
                // Clear screen
                write!(
                    stdout,
                    "{}{}Text reset. Press RIGHT ARROW to start again.",
                    termion::clear::All,
                    termion::cursor::Goto(1, 1)
                )
                .unwrap();
            },
            Key::Right => {
                if word_index < words.len() {
                    if !displayed_text.is_empty() {
                        displayed_text.push(' ');
                    }
                    displayed_text.push_str(words[word_index]);
                    word_index += 1;
                    
                    // Clear screen and redisplay all text
                    write!(
                        stdout,
                        "{}{}{}",
                        termion::clear::All,
                        termion::cursor::Goto(1, 1),
                        displayed_text
                    )
                    .unwrap();
                }
            },
            _ => {}
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
