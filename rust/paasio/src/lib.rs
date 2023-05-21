use std::io::{Read, Result, Write};

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub struct ReadStats<R> {
    r: R,
    operation_count: usize,
    bytes_count: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            r: wrapped,
            operation_count: 0,
            bytes_count: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.r
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_count
    }

    pub fn reads(&self) -> usize {
        self.operation_count
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.operation_count += 1;

        let result = self.r.read(buf);

        if let Ok(readed) = result {
            self.bytes_count += readed;
        }

        result
    }
}

pub struct WriteStats<W: Write> {
    w: W,
    operation_count: usize,
    bytes_count: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            w: wrapped,
            operation_count: 0,
            bytes_count: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.w
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_count
    }

    pub fn writes(&self) -> usize {
        self.operation_count
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.operation_count += 1;
        self.bytes_count += buf.len();

        self.w.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.w.flush()
    }
}
