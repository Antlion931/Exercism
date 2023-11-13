use std::fmt::Debug;

#[derive(Debug)]
pub struct CircularBuffer<T> {
    body: Vec<T>,
    capacity: usize,
    size: usize,
    youngest: usize,
    oldest: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

fn circular_next(value: usize, max: usize) -> usize {
    (value + 1) % max
}

impl<T: Clone + Debug> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            body: Vec::with_capacity(capacity),
            capacity,
            youngest: 0,
            oldest: 0,
            size: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.size == self.capacity {
            return Err(Error::FullBuffer);
        }

        if self.size != 0 {
            self.youngest = circular_next(self.youngest, self.capacity);
        }

        self.size += 1;

        if let Some(x) = self.body.get_mut(self.youngest) {
            *x = element;
        } else {
            self.body.push(element);
        }

        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.size == 0 {
            return Err(Error::EmptyBuffer);
        }

        self.size -= 1;

        let result = self.body[self.oldest].clone();
        self.oldest = circular_next(self.oldest, self.capacity);
        Ok(result)
    }

    pub fn clear(&mut self) {
        self.oldest = 0;
        self.youngest = 0;
        self.size = 0;
        self.body.clear();
    }

    pub fn overwrite(&mut self, element: T) {
        if self.size != 0 {
            self.youngest = circular_next(self.youngest, self.capacity);
        }

        if self.size == self.capacity {
            self.oldest = circular_next(self.oldest, self.capacity);
        } else {
            self.size += 1;
        }

        if let Some(x) = self.body.get_mut(self.youngest) {
            *x = element;
        } else {
            self.body.push(element);
        }
    }
}
