
pub trait BaseCommand {
	fn on(arguments: Vec<&str>, handler: Box<Fn(Vec<String>, String, String, bool) -> Option<String>>);
	fn command_name() -> &'static str;
}
