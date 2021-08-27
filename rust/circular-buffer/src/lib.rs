pub struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    oldest: usize,
    length: usize,
    capacity: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: (0..capacity).map(|_| None).collect(),
            oldest: 0,
            length: 0,
            capacity,
        }
    }

    fn is_full(&self) -> bool {
        self.length == self.capacity
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::FullBuffer);
        }
        self.length += 1;
        self.buffer[(self.oldest + self.length - 1) % self.capacity] = Some(element);
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.is_empty() {
            return Err(Error::EmptyBuffer);
        }
        let element = self.buffer[self.oldest].take().unwrap();
        self.oldest = (self.oldest + 1) % self.capacity;
        self.length -= 1;
        Ok(element)
    }

    pub fn clear(&mut self) {
        while !self.is_empty() {
            self.read().unwrap();
        }
    }

    pub fn overwrite(&mut self, element: T) {
        if self.is_full() {
            self.buffer[self.oldest] = Some(element);
            self.oldest = (self.oldest + 1) % self.capacity;
        } else {
            self.length += 1;
            self.buffer[(self.oldest + self.length - 1) % self.capacity] = Some(element);
        }
    }
}
