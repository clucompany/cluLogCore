

use log_addition::union::LogUnionConst;
use log_addition::union::default::LogUnion;
use log::lock::LogLockIO;
use log::LogFlushIO;
use std::io::Write;
use std::fmt::Arguments;
use log::cluLog;
use std::io;
use log::lock::default::LogLock;
use log::lock::default_no_flush::LogLockNoFlush;
use log_addition::empty::LogEmptyConst;

#[derive(Debug)]
pub struct LogTotalEmpty;

impl LogTotalEmpty {
	pub fn new() -> Self {
		LogTotalEmpty
	}
	#[inline]
	pub fn union<'a>() -> LogUnion<'a, LogTotalEmpty, LogTotalEmpty> {
		LogUnion::total_empty()
	}
}


impl<'l> cluLog<'l> for LogTotalEmpty {
	#[inline(always)]
	fn warning<'a>(&self, _args: Arguments<'a>) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]
	fn info<'a>(&self, _args: Arguments<'a>) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]
	fn error<'a>(&self, _args: Arguments<'a>) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]
	fn panic<'a>(&self, args: Arguments<'a>) -> io::Result<()> {
		panic!("{}", args);
	}
	
	#[inline(always)]	
	fn unknown<'a>(&self, _name: &'static str, _args: Arguments<'a>) -> io::Result<()> {
		Ok( () )
	}

	#[inline(always)]
	fn trace<'a>(&self, _line: u32, _pos: u32, _file: &'static str, _args: Arguments<'a>) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]	
	fn print<'a>(&self, _args: Arguments<'a>) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]	
	fn eprint<'a>(&self, _args: Arguments<'a>) -> io::Result<()> {
		Ok( () )
	}
}

impl LogFlushIO for LogTotalEmpty {
	#[inline(always)]	
	fn flush_out(&mut self) -> io::Result<()> {
		Ok( () )
	}
	
	#[inline(always)]
	fn flush_err(&mut self) -> io::Result<()> {
		Ok( () )
	}
}

impl<'a> LogLockIO<'a> for LogTotalEmpty {
	fn lock_out(&'a self) -> Box<Write + 'a> {
		LogLock::empty_boxed()
	}

	fn lock_err(&'a self) -> Box<Write + 'a> {
		LogLock::empty_boxed()
	}

	fn no_flush_lock_out(&'a self) -> Box<Write + 'a> {
		LogLockNoFlush::empty_boxed()
	}

	fn no_flush_lock_err(&'a self) -> Box<Write + 'a> {
		LogLockNoFlush::empty_boxed()
	}
}


impl<'a> LogUnionConst<'a> for LogTotalEmpty {}