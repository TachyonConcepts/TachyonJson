#[macro_export]
macro_rules! tachyon_object {
    ($($key:expr => $val:expr),* $(,)?) => {{
        const __TACHYON_PAIRS: &[$crate::TachyonPair<'static>] = &[
            $(
                $crate::TachyonPair {
                    key: $key,
                    value: $val,
                }
            ),*
        ];
        $crate::TachyonValue::Object($crate::TachyonObject {
            ptr: __TACHYON_PAIRS.as_ptr(),
            len: __TACHYON_PAIRS.len(),
        })
    }};
}

#[macro_export]
macro_rules! tachyon_object_noescape {
    ($($key:literal => $val:literal),* $(,)?) => {{
        const COUNT: usize = <[()]>::len(&[$(tachyon_object_noescape!(@count $key)),*]);
        static mut STORAGE: [$crate::TachyonPair; COUNT] = [
            $(
                $crate::TachyonPair {
                    key: $key,
                    value: $crate::TachyonValue::String($val),
                }
            ),*
        ];
        $crate::TachyonValue::Object($crate::TachyonObject {
            ptr: unsafe { STORAGE.as_ptr() },
            len: COUNT,
        })
    }};
    (@count $x:expr) => { () };
}