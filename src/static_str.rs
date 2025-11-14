pub struct StaticString<const N: usize, T>
where
    T: Sized + Copy,
{
    buf: [T; N],
    i: usize,
}

impl<const N: usize, T> StaticString<N, T> 
where T: Sized + Copy {
    pub fn new(t: T) -> Self {
        Self {
            buf: [t; N],
            i: 0,
        }
    }

    pub fn push(&mut self, t: T) {
        self.buf[self.i] = t;
        self.i = (self.i + 1) % N;
    }

    pub fn len(&self) -> usize {
        self.i
    }

    #[inline]
    pub fn clear(&mut self) {
        self.i = 0;
    }
}

impl<const N: usize> StaticString<N, u8> {
    pub fn make_printable(&mut self) -> [u8; N] {
        self.push('\0' as u8);
        self.buf
    }
}