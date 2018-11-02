

use log_lock::FlushOut;
use log_lock::LogSafeLock;

///Secure outflow blocking
#[allow(non_camel_case_types)]
pub trait LogLockIO<'a> {
	///Blocking threads with automatic cleaning
	fn lock_out(&'a self) -> FlushOut<Box<LogSafeLock<'a> + 'a>> {
		FlushOut::new(self.no_flush_lock_out())
	}

	///Blocking threads with automatic cleaning
	fn lock_err(&'a self) -> FlushOut<Box<LogSafeLock<'a> + 'a>> {
		FlushOut::new(self.no_flush_lock_err())	
	}

	///Flow blocking without self-cleaning
	fn no_flush_lock_out(&'a self) -> Box<LogSafeLock<'a> + 'a>;

	///Flow blocking without self-cleaning
	fn no_flush_lock_err(&'a self) -> Box<LogSafeLock<'a> + 'a>;
}


impl<'a, A: LogLockIO<'a>> LogLockIO<'a> for &'a A {
	///Blocking threads with automatic cleaning
	#[inline(always)]
	fn lock_out(&'a self) -> FlushOut<Box<LogSafeLock<'a> + 'a>> {
		(**self).lock_out()
	}

	///Blocking threads with automatic cleaning
	#[inline(always)]
	fn lock_err(&'a self) -> FlushOut<Box<LogSafeLock<'a> + 'a>> {
		(**self).lock_err()
	}

	///Flow blocking without self-cleaning
	#[inline(always)]
	fn no_flush_lock_out(&'a self) -> Box<LogSafeLock<'a> + 'a> {
		(**self).no_flush_lock_out()
	}

	///Flow blocking without self-cleaning
	#[inline(always)]
	fn no_flush_lock_err(&'a self) -> Box<LogSafeLock<'a> + 'a> {
		(**self).no_flush_lock_err()
	}
}


impl<'a, A: LogLockIO<'a>> LogLockIO<'a> for &'a mut A {
	///Blocking threads with automatic cleaning
	#[inline(always)]
	fn lock_out(&'a self) -> FlushOut<Box<LogSafeLock<'a> + 'a>> {
		(**self).lock_out()
	}

	///Blocking threads with automatic cleaning
	#[inline(always)]
	fn lock_err(&'a self) -> FlushOut<Box<LogSafeLock<'a> + 'a>> {
		(**self).lock_err()
	}

	///Flow blocking without self-cleaning
	#[inline(always)]
	fn no_flush_lock_out(&'a self) -> Box<LogSafeLock<'a> + 'a> {
		(**self).no_flush_lock_out()
	}

	///Flow blocking without self-cleaning
	#[inline(always)]
	fn no_flush_lock_err(&'a self) -> Box<LogSafeLock<'a> + 'a> {
		(**self).no_flush_lock_err()
	}
}