extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use std::io::stdin;

use lexer::lexer::Lexer;

mod lexer;

fn main() {
    pretty_env_logger::init();

    let mut string = String::new();
    let mut lex = Lexer::new();
    info!("start");

    stdin()
        .read_line(&mut string)
        .ok()
        .expect("Failde to read line");
    if string != "" {
        match lex.parse(string.trim().into()) {
            Ok(t) => t.iter().for_each(|f| info!("{}", f)),
            Err(e) => error!("{}", e),
        }
    }
}
