#[derive(Clone, Copy, Debug)]
pub struct TachyonBuffer<const N: usize> {
    pub(super) buf: [u8; N],
    pub(super) pos: usize,
    pub(super) error: bool,
}
impl<const N: usize> Default for TachyonBuffer<N> {
    #[inline(always)]
    fn default() -> Self {
        Self {
            buf: [0u8; N],
            pos: 0,
            error: false,
        }
    }
}

impl<const N: usize> TachyonBuffer<N> {
    #[inline(always)]
    pub fn reset_pos(&mut self) {
        self.pos = 0;
        return;
    }
    #[inline(always)]
    pub unsafe fn write(&mut self, s: &[u8]) {
        if self.error || self.pos + s.len() > N {
            self.error = true;
            return;
        }
        std::ptr::copy_nonoverlapping(s.as_ptr(), self.buf.as_mut_ptr().add(self.pos), s.len());
        self.pos += s.len();
    }

    #[inline(always)]
    pub unsafe fn write_str(&mut self, s: &str) {
        self.write(s.as_bytes());
    }

    #[inline(always)]
    pub unsafe fn write_char(&mut self, c: u8) {
        self.write(std::slice::from_ref(&c));
    }

    #[inline(always)]
    pub unsafe fn as_slice(&self) -> &[u8] {
        &self.buf[..self.pos]
    }

    #[inline(always)]
    pub unsafe fn as_str(&self) -> &str {
        std::str::from_utf8(&self.buf[..self.pos]).unwrap_or("")
    }

    #[inline(always)]
    pub unsafe fn to_vec(&self) -> Vec<u8> {
        self.as_slice().to_vec()
    }
}

impl<const N: usize> TachyonBuffer<N> {
    #[inline(always)]
    pub unsafe fn as_mut_ptr_offset(&mut self) -> *mut u8 {
        unsafe { self.buf.as_mut_ptr().add(self.pos) }
    }

    #[inline(always)]
    pub unsafe fn set_offset_from_ptr(&mut self, ptr: *mut u8) {
        self.pos = ptr.offset_from(self.buf.as_ptr()) as usize;
    }
}

impl<const N: usize> TachyonBuffer<N> {
    #[inline(always)]
    pub unsafe fn write_raw_bytes(&mut self, src: *const u8, len: usize) {
        let dst = self.buf.as_mut_ptr().add(self.pos);
        std::ptr::copy_nonoverlapping(src, dst, len);
        self.pos += len;
    }

    #[inline(always)]
    pub unsafe fn write_char_fast(&mut self, c: u8) {
        *self.buf.as_mut_ptr().add(self.pos) = c;
        self.pos += 1;
    }
}

#[macro_export(local_inner_macros)]
macro_rules! tcopy {
    ($len:expr, $this:expr, $dst:expr, $src:expr) => {{
        match $len {
            7 => {
                use core::ptr;
                let val = ptr::read_unaligned($src as *const u64);
                ptr::write_unaligned($dst as *mut u64, val);
                $this.pos += 7;
            }
            13 => {
                use core::ptr;
                let val1 = ptr::read_unaligned($src as *const u64);
                ptr::write_unaligned($dst as *mut u64, val1);
                let val2 = ptr::read_unaligned($src.add(8) as *const u32);
                ptr::write_unaligned($dst.add(8) as *mut u32, val2);
                *$dst.add(12) = *$src.add(12);
                $this.pos += 13;
            }
            _ => {
                std::ptr::copy_nonoverlapping($src, $dst, $len);
                $this.pos += $len;
            }
        }
    }};
}
