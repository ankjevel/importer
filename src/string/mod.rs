use std::mem::{transmute, forget};

pub fn string_to_static_str(s: String) -> &'static str {
    unsafe {
        let ret = transmute(&s as &str);
        forget(s);
        ret
    }
}

pub fn borrowed_string_to_static_str<'a>(s: &'a str) -> &'static str {
    unsafe {
        let ret = transmute(&s as &str);
        forget(s);
        ret
    }
}

#[cfg(test)]
mod tests {
    use string;

    #[test]
    fn takes_ownership() {
        let foo = String::from("foo bar");
        assert!(string::string_to_static_str(foo) == "foo bar")
    }

    #[test]
    fn uses_reference() {
        let foo = String::from("foo bar");
        assert!(string::borrowed_string_to_static_str(&foo) == "foo bar")
    }
}
