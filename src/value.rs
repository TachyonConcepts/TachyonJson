use crate::buffer::TachyonBuffer;
use crate::tcopy;

const CONTROL_ESCAPES: [&[u8]; 32] = [
    br"\u0000", br"\u0001", br"\u0002", br"\u0003", br"\u0004", br"\u0005", br"\u0006", br"\u0007",
    br"\b", br"\t", br"\n", br"\u000B", br"\f", br"\r", br"\u000E", br"\u000F", br"\u0010",
    br"\u0011", br"\u0012", br"\u0013", br"\u0014", br"\u0015", br"\u0016", br"\u0017", br"\u0018",
    br"\u0019", br"\u001A", br"\u001B", br"\u001C", br"\u001D", br"\u001E", br"\u001F",
];

#[repr(u8)]
pub enum TachyonValue<'a> {
    String(&'a str) = 0,
    Number(f64) = 1,
    Object(TachyonObject<'a>) = 2,
    Array(&'a [TachyonValue<'a>]) = 3,
    True = 4,
    False = 5,
    Null = 6,
    Undefined = 7,
}

pub struct TachyonObject<'a> {
    pub ptr: *const TachyonPair<'a>,
    pub len: usize,
}

pub struct TachyonPair<'a> {
    pub key: &'a str,
    pub value: TachyonValue<'a>,
}

impl<'a> TachyonValue<'a> {
    #[inline(always)]
    pub unsafe fn encode<const N: usize>(&self, b: &mut TachyonBuffer<N>, no_escape: bool) {
        match self {
            TachyonValue::String(s) => {
                if no_escape {
                    b.write_char_fast(b'"');
                    let src = s.as_ptr();
                    let len = s.len();
                    let dst = b.buf.as_mut_ptr().add(b.pos);
                    tcopy!(len, b, dst, src);
                    b.write_char_fast(b'"');
                } else {
                    b.write_char_fast(b'"');
                    let bytes = s.as_bytes();
                    let mut i = 0;

                    while i < bytes.len() {
                        let c = bytes[i];
                        match c {
                            b'"' => b.write(b"\\\""),
                            b'\\' => b.write(b"\\\\"),
                            0x00..=0x1F => b.write(CONTROL_ESCAPES[c as usize]),
                            _ => b.write_char_fast(c),
                        }
                        i += 1;
                    }
                    b.write_char_fast(b'"');
                }
            }

            TachyonValue::Number(n) => {
                let mut buf = ryu::Buffer::new();
                b.write_str(buf.format(*n));
            }

            TachyonValue::Object(obj) => {
                b.write_char_fast(b'{');
                let mut first = true;

                let mut i = 0;
                while i < obj.len {
                    let pair = unsafe { &*obj.ptr.add(i) };
                    i += 1;

                    if let TachyonValue::Undefined = pair.value {
                        continue;
                    }

                    if !first {
                        b.write_char_fast(b',');
                    }
                    first = false;

                    let key = pair.key.as_bytes();
                    b.write_char_fast(b'"');

                    let src = key.as_ptr();
                    let len = key.len();
                    let dst = b.buf.as_mut_ptr().add(b.pos);
                    tcopy!(len, b, dst, src);

                    b.write_char_fast(b'"');
                    b.write_char_fast(b':');
                    pair.value.encode::<N>(b, no_escape);
                }

                b.write_char_fast(b'}');
            }

            TachyonValue::Array(items) => {
                b.write_char_fast(b'[');
                let mut first = true;

                for item in items.iter() {
                    if matches!(item, TachyonValue::Undefined) {
                        continue;
                    }

                    if !first {
                        b.write_char_fast(b',');
                    }
                    first = false;

                    item.encode::<N>(b, no_escape);
                }

                b.write_char_fast(b']');
            }

            TachyonValue::True => b.write_str("true"),
            TachyonValue::False => b.write_str("false"),
            TachyonValue::Null => b.write_str("null"),
            TachyonValue::Undefined => b.error = true,
        }
    }
}

#[repr(transparent)]
pub struct FastStr(pub *const u8);
impl Copy for FastStr {}
impl Clone for FastStr {
    fn clone(&self) -> Self {
        *self
    }
}
impl FastStr {
    #[inline(always)]
    pub fn new(s: &'static str) -> Self {
        let ptr = (s.as_ptr() as usize | 1) as *const u8;
        Self(ptr)
    }
    #[inline(always)]
    pub fn as_str(&self, len: usize) -> &'static str {
        let ptr = (self.0 as usize & !1) as *const u8;
        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len)) }
    }
}
