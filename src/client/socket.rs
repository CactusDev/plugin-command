
use json;
use ws;

use cereus::*;

pub struct CereusClient {
	ws: ws::Sender
}

impl ws::Handler for CereusClient {
	fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
		println!("Connected to Cereus!");
	
		Ok(())
	}

	fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
		println!("Got: {}", msg);

		// Attempt to parse it into JSON.
		// let parsed: json::Result<
		Ok(())
	}
}
