
extern crate cereus;

#[macro_use]
extern crate plugin_command_derive;

extern crate serde_json as json;
extern crate ws;
extern crate regex;

mod client;
mod parser;
mod derive;
mod command;

use derive::BaseCommand;

fn main() {
	let a = parser::CommandParser::new();
    println!("{}", a.parse_text_into_terms(String::from("!command add test This is a test, %thing|a|b|c%")).unwrap());
    println!("{}", a.parse_text_into_terms(String::from("!command add test This is a test, %test|a|b|c%")).unwrap());

    command::Test::execute();
}
