use melevo::Melevo;
use simple_logger::SimpleLogger;


fn main() {
	SimpleLogger::new().init().unwrap();
	let app = Melevo::init();

	app.run();
}

