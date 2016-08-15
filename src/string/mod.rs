use std::mem::{transmute, forget};

pub fn string_to_static_str(s: String) -> &'static str {
    unsafe {
        let ret = transmute(&s as &str);
        forget(s);
        ret
    }
}
