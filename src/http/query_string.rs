use std::collections::HashMap;

// a=1&b=2&c&d=&e===&d=7&d=abc
// ("a", "1")
// ("b", "2")
// ("c", "")
// ("d". ["", "7", "abc"])
// ("e". "==")

pub struct QueryString<'buf>(HashMap<&'buf str, Value<'buf>>);

pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>)
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &'buf str) -> Option<&Value> {
        self.0.get(key)
    }
}