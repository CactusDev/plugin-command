
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

}

impl CommandParser {

	pub fn new() -> Self {
		CommandParser {

		}
	}

	pub fn parseCommandIntoTerms(&self, command: String) -> CommandTerms {
		// Start by splitting the command, and taking out terms that we
		// can immediately identify.

		let parts: Vec<String> = command.split(" ").collect();

		let prefix = &parts[0].chars().next().unwrap();	
		let base: String = &parts[0].chars().skip(1).collect();

		parts.remove(0);

		for part in &parts {
			// Now, we need to figure out if this is just regular text,
			// or if this is some kind of a variable.
			if &part.starts_with("")
		}
	}
}
