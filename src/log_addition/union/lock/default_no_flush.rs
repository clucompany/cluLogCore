
use log_addition::empty::LogEmptyConst;
use std::fmt::Debug;
use log_addition::empty::empty_write::EmptyWrite;
use std::marker::PhantomData;
use std::io::Write;
use std::io;

#[allow(non_camel_case_types)]
pub struct UnionLockNoFlush<'a, W: Write + 'a, W2: Write + 'a>(W,W2, PhantomData<&'a ()>);

impl<'a, W: Write + 'a, W2: Write + 'a> UnionLockNoFlush<'a, W, W2> {
	#[inline]
	pub fn new(out: W, out2: W2) -> Self {
		UnionLockNoFlush(out, out2, PhantomData)
	}
}

impl<'a> LogEmptyConst for UnionLockNoFlush<'a, EmptyWrite, EmptyWrite> {
	#[inline]
	fn empty() -> Self {
		Self::new(EmptyWrite, EmptyWrite)
	}
}



impl<'a, W: Write + 'a, W2: Write + 'a> Debug for UnionLockNoFlush<'a, W, W2> {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		f.pad("UnionLockNoFlush { .. }")
	}
}

impl<'a, W: Write + 'a, W2: Write + 'a> Write for UnionLockNoFlush<'a, W, W2> {
     #[inline(always)]
     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
          match self.0.write(buf) {
               Ok(s) => {
                    match self.1.write(buf) {
                         Ok(s2) => return Ok(s+s2),
                         a => return a,
                    }
               },
               a => return a,
          }
     }
     #[inline(always)]
     fn flush(&mut self) -> io::Result<()> {
          if let Err(e) = self.0.flush() {
               return Err(e);
          }
          if let Err(e) = self.1.flush() {
               return Err(e);
          }
          Ok( () )
     }
}