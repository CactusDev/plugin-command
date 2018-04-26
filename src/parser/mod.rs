 
extern crate regex;
use regex::Regex;

use std::fmt;

#[derive(Debug)]
pub enum TextTerm {
	Text { text: String },
	Variable { variable: String, modifiers: Vec<String> }
}

impl fmt::Display for TextTerm {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&TextTerm::Text {ref text} => {
				write!(f, "Text argument: {}", &text)
			},
			&TextTerm::Variable {ref variable, ref modifiers} => {
				write!(f, "Variable Argument: {}, with modifiers: {}", &variable, modifiers.join(", "))
			}
		}
	}
}

pub struct CommandParser {
	args_regex: Regex,
	argn_regex: Regex
}

impl CommandParser {
	pub fn new() -> Self {
		CommandParser {
			args_regex: Regex::new("%ARGS(?:=([^|]+))?((?:\\|\\w+)+)?%").unwrap(),
			argn_regex: Regex::new("%ARG(\\d+)(?:=([^|]+))?(?:((?:\\|\\w+)+))?%").unwrap()
		}
	}
}

pub struct TermParser;

impl TermParser {
	pub fn new() -> Self {
		TermParser {}
	}

	fn is_valid_variable(&self, variable: &str) -> bool {
		match variable {
			"test" => true,
			_      => false
		}
	}

	pub fn parse_text_into_terms(&self, command: String) -> Vec<TextTerm> {
		// Make sure this even looks like a command.
		let mut chars = command.chars();
		let prefix = chars.next();
		let next = chars.next();

		if let Some(_) = prefix {
			if let Some(next) = next {
				// Since we have at least two characters, then this might actually be
				// a command. But, if the next character is a space then we don't care
				if next == ' ' {
					return vec! []
				}
			}
		}

		// Start by splitting the command, and taking out terms that we
		// can immediately identify.
		let parts: Vec<&str> = command.split(" ").collect();
		let mut finished: Vec<TextTerm> = Vec::new();

		for part in &parts {
			// Now, we need to figure out if this is just regular text,
			// or if this is some kind of a variable.
			if part.starts_with("%") {
				// This could potentially be a variable.
				if !part.ends_with("%") {
					// If it doesn't end with a %, then it can't be one
					// So, we're going to call this a regular text block.
					//
					finished.push(TextTerm::Text {text: part.to_string()});
					continue;
				}
				// Since it starts and ends with a %, we can infer that this is
				// most likely a variable, so we'll move on with parsing and
				// see what we can find out about it.
				//
				let part = part.replace("%", "");
				// Before we can check if this is a valid variable, we need to
				// take it away from any of the modifiers that might be present
				//
				let (variable, modifiers): (&str, Vec<&str>) = {
					let split: Vec<&str> = part.split("|").collect();
					(split[0], split[1..].to_vec())
				};
				// Now we can actually check if this is a valid variable.
				if !self.is_valid_variable(variable) {
					// It's not, so we're going to call this a regular text
					// block.
					finished.push(TextTerm::Text {text: part.to_string()});
					continue;
				}
				// Valid variable, so we can finally commit this one.
				finished.push(TextTerm::Variable {
					variable: variable.to_string(),
					modifiers: modifiers.iter().map(|s| s.to_string()).collect()
				});
				continue;
			}
			// This segment is not a variable, thus it must be text.
			finished.push(TextTerm::Text {
				text: part.to_string()
			});
		}
		finished
	}
}
