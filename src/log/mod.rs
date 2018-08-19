
pub mod default;
pub mod default_one;

use log_lock::LogSafeLock;
use log_addition::union::LogUnionConst;
use std::fmt::Arguments;
use std::io;


#[allow(non_camel_case_types)]
///An empty implementation allows you to use the current log system as the main
pub trait LogStatic<'a>: LogBase<'a> + LogLockIO<'a> + LogFlush {

}

#[allow(non_camel_case_types)]
///Empty implementation allows you to fully manipulate the current system of journals
pub trait LogExtend<'a>: LogBase<'a> + LogLockIO<'a> + LogFlush + LogUnionConst<'a> /*+ LogEmptyConst*/ {

}

///Generalization of the basic methods of information output
pub trait LogBase<'a> {
	fn warning<'s>(&'a self, args: Arguments<'s>) -> io::Result<()>;
	
	fn info<'s>(&'a self, args: Arguments<'s>) -> io::Result<()>;
	
	fn error<'s>(&'a self, args: Arguments<'s>) -> io::Result<()>;
	
	fn panic<'s>(&'a self, args: Arguments<'s>) -> io::Result<()>;
	
	fn unknown<'s>(&'a self, name: &'static str, args: Arguments<'s>) -> io::Result<()>;

	fn trace<'s>(&'a self, line: u32, pos: u32, file: &'static str, args: Arguments<'s>) -> io::Result<()>;
	
	fn print<'s>(&'a self, args: Arguments<'s>) -> io::Result<()>;

	fn eprint<'s>(&'a self, args: Arguments<'s>) -> io::Result<()>;
}

///Secure outflow blocking
#[allow(non_camel_case_types)]
pub trait LogLockIO<'a> {
	///Blocking threads with automatic cleaning
	fn lock_out(&'a self) -> Box<LogSafeLock<'a> + 'a>;

	///Blocking threads with automatic cleaning
	fn lock_err(&'a self) -> Box<LogSafeLock<'a> + 'a>;

	///Flow blocking without self-cleaning
	fn no_flush_lock_out(&'a self) -> Box<LogSafeLock<'a> + 'a>;

	///Flow blocking without self-cleaning
	fn no_flush_lock_err(&'a self) -> Box<LogSafeLock<'a> + 'a>;
}

///Generalization for cleaning output streams
#[allow(non_camel_case_types)]
pub trait LogFlush {
	///Flush the output stream
	fn flush_out(&mut self) -> io::Result<()>;

	///Flush the err-output stream
	fn flush_err(&mut self) -> io::Result<()>;
	
	///Flush Out stream and Err stream
	#[inline]
	fn flush(&mut self) -> io::Result<()> {
		if let Err(e) = self.flush_out() {
			return Err(e);
		}
		self.flush_err()
	}
}