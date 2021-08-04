use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    r: R,
    reads: usize,
    bytes_through: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(r: R) -> ReadStats<R> {
        ReadStats {
            r,
            reads: 0,
            bytes_through: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.r
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let size = self.r.read(buf)?;
        self.bytes_through += size;
        self.reads += 1;

        Ok(size)
    }
}

pub struct WriteStats<W> {
    w: W,
    writes: usize,
    bytes_through: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(w: W) -> WriteStats<W> {
        WriteStats {
            w,
            writes: 0,
            bytes_through: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.w
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let size = self.w.write(buf)?;
        self.bytes_through += size;
        self.writes += 1;

        Ok(size)
    }

    fn flush(&mut self) -> Result<()> {
        self.w.flush()
    }
}
