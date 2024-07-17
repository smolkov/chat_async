use log::{Log,Level};

pub struct Logger {

}


impl Logger {
	pub fn new() -> Logger {
		Logger{}
	}
}

impl Log for Logger{
	fn enabled(&self, metadata: &log::Metadata) -> bool {
		metadata.level() >= Level::Debug
	}
	fn log(&self, record: &log::Record) {
		if self.enabled(record.metadata())	 {
			println!("[{}] - {}",record.level(),record.args());
		}
	}
	fn flush(&self) {}
}

pub fn init() -> Result<(),log::SetLoggerError> {
	log::set_boxed_logger(Box::new(Logger::new()))?;
	Ok(())
}

