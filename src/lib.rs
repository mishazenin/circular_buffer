pub struct CircularBuffer {
    buffer: Vec<Option<T>>,
    size: usize,
    read_cursor: usize,
    write_cursor: usize,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            buffer: vec![None, capacity],
            size: capacity,
            read_cursor: 0,
            write_cursor: 0,
        }
    }
    pub fn read(&mut self) -> Result<T, BufferError> {
        match self.buffer[self.read_cursor] {
            Some(value) => {
                self.bump_read_cursor();
                Ok(value)
            }
            None => BufferError::EmptyBuffer
        }
    }

    pub fn clear(&mut self) {
        self.buffer.iter_mut().for_each(|x| {
            x.take();
        });
        self.read_cursor = 0;
        self.write_cursor = 0
    }

    pub fn write(&mut self, element: T) -> Result<(), BufferError> {
        match self.buffer[self.write_cursor] {
            Some(_) => Err(BufferError::FullBuffer),
            None => {
                self.buffer[self.write_cursor] = Some(element);
                self.bump_write_cursor();
                Ok(())
                /// bump write cursor
            }
        }
    }

    fn bump_read_cursor(&mut self) {
        self.read_cursor += 1;
        if self.read_cursor == self.size {
            self.read_cursor = 0
        }
    }


    fn bump_write_cursor(&mut self) {
        self.write_cursor += 1;
        if self.write_cursor == self.size {
            self.write_cursor = 0
        }
    }
    pub fn overwrite(&mut self, element: T) {
        match self.buffer[self.write_cursor] {
            Some(_) => {
                self.buffer[self.write_cursor] = element;
                self.bump_read_cursor();
                self.bump_write_cursor()
            }
            None => {
                self.write(element).unwrap()
            }
        }
    }
}

pub enum BufferError {
    EmptyBuffer,
    FullBuffer,
}