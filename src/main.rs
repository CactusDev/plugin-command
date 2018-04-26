
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

use derive::{BaseCommand};

// TODO: Make this
//
// #[derive(CM)]
// #[Commands = "CactusCommand"]
// struct CommandManager;
//

fn main() {
	let a = parser::TermParser::new();
    println!("{:?}", a.parse_text_into_terms(String::from("!command add test This is a test, %thing|a|b|c%")));
    println!("{:?}", a.parse_text_into_terms(String::from("!command add test This is a test, %test|a|b|c%")));

    command::CactusCommand::on(vec! ["docs"], Box::new(|args, sender, channel, whisper| {
		println!("{:?} {} {} {}", args, sender, channel, whisper);
		None
	}));
	println!("{}", command::CactusCommand::command_name());
}
