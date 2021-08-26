use std::borrow::Borrow;
use std::io::{Read, Write};
use std::iter::Cycle;
use std::slice::Iter;

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: Cycle<Iter<'a, u8>>,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key>(key: &'a Key) -> Xorcism<'a>
    where
        Key: AsRef<[u8]> + ?Sized,
    {
        Self {
            key: key.as_ref().iter().cycle(),
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        data.into_iter()
            .for_each(|b| *b ^= self.key.next().unwrap())
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<'b, Data>(&'b mut self, data: Data) -> impl Iterator<Item = u8> + Captures<'a> + 'b
    where
        Data: IntoIterator,
        Data::Item: Borrow<u8>,
        Data::IntoIter: 'b,
    {
        data.into_iter()
            .map(move |b| b.borrow() ^ self.key.next().unwrap())
    }

    pub fn reader(self, reader: impl Read + 'a) -> impl Read + 'a {
        XorcismReaderWriter {
            rw: reader,
            munger: self,
        }
    }

    pub fn writer(self, writer: impl Write + 'a) -> impl Write + 'a {
        XorcismReaderWriter {
            rw: writer,
            munger: self,
        }
    }
}

pub trait Captures<'a> {}
impl<'a, T: ?Sized> Captures<'a> for T {}

struct XorcismReaderWriter<'a, RW> {
    rw: RW,
    munger: Xorcism<'a>,
}

impl<'a, R: Read> Read for XorcismReaderWriter<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.rw.read(buf)?;
        self.munger.munge_in_place(&mut buf[..n]);
        Ok(n)
    }
}

impl<'a, W: Write> Write for XorcismReaderWriter<'a, W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.rw.write(&self.munger.munge(buf).collect::<Vec<u8>>())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.rw.flush()
    }
}
