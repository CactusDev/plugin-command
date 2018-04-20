
extern crate regex;

const ARGN_EXPR

enum CommandArgument {
	Text { text: String },
	Variable { variable: String, modifiers: Vec<String> }
}

struct CommandTerms {
	pub prefix:    String,
	pub base:      String,
	pub arguments: Vec<CommandArgument>
}

pub struct CommandParser {
	args_regex: regex::Regex,
	argn_regex: regex::Regex
}

impl CommandParser {

	pub fn new() -> Self {
		CommandParser {
			args_regex: regex::Regex::new("%ARGS(?:=([^|]+))?((?:\|\w+)+)?%").unwrap(),
			argn_regex: regex::Regex::new("%ARG(\d+)(?:=([^|]+))?(?:((?:\|\w+)+))?%").unwrap()
		}
	}

	pub fn parseTextIntoTerms(&self, command: String) -> Option<CommandTerms> {
		// Make sure this even looks like a command.
		let chars = command.chars();
		let prefix = chars.next();
		let next = chars.next();

		if let Some(prefix) = prefix && let Some(next) = next {
			// Since we have at least two characters, then this might actually be
			// a command. But, if the next character is a space then we don't care
			if next == ' ' {
				return None
			}
		}

		// Start by splitting the command, and taking out terms that we
		// can immediately identify.
		let mut parts: Vec<&str> = command.split(" ").collect();

		let prefix = prefix.unwrap();
		let base: Vec<char> = parts[0].chars().skip(1).collect();

		parts.remove(0);

		for part in &parts {
			// Now, we need to figure out if this is just regular text,
			// or if this is some kind of a variable.
			if part.starts_with("%") {

			}
		}
		None
	}
}
