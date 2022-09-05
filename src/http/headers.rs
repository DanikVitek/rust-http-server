use std::{
    collections::{HashMap, hash_map::IntoIter},
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Debug)]
pub struct Headers<'buf>(HashMap<&'buf str, Value<'buf>>);

impl<'buf> Headers<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value<'buf>> {
        self.0.get(key)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'buf> IntoIterator for Headers<'buf> {
    type Item = (&'buf str, Value<'buf>);

    type IntoIter = IntoIter<&'buf str, Value<'buf>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'buf> Display for Headers<'buf> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Ok(for (i, (key, value)) in self.0.iter().enumerate() {
            match value {
                Value::Single(val) => write!(f, "{key}: {val}")?,
                Value::Multiple(vec) => {
                    for (i, val) in vec.iter().enumerate() {
                        write!(f, "{key}: {val}")?;
                        if i + 1 < vec.len() {
                            write!(f, "\r\n")?;
                        }
                    }
                }
            }
            if i + 1 < self.0.len() {
                write!(f, "\r\n")?;
            }
        })
    }
}

impl<'buf> TryFrom<&'buf str> for Headers<'buf> {
    type Error = HeadersError<'buf>;

    fn try_from(s: &'buf str) -> Result<Self, Self::Error> {
        let mut data = HashMap::new();

        for pair in s.split("\r\n").map(|header| header.trim()) {
            match pair.find(": ") {
                Some(ind) => {
                    let (key, val) = pair.split_at(ind);
                    add_value(&mut data, key, &val[2..]);
                }
                None => return Err(HeadersError(s)),
            }
        }

        Ok(Self(data))
    }
}

fn add_value<'buf>(
    data: &mut HashMap<&'buf str, Value<'buf>>,
    key: &'buf str,
    value: &'buf str,
) -> () {
    data.entry(key)
        .and_modify(|existing| match existing {
            Value::Single(prev_value) => {
                *existing = Value::Multiple(vec![prev_value, value]);
            }
            Value::Multiple(prev_values) => {
                prev_values.push(value);
            }
        })
        .or_insert(Value::Single(value));
}

#[derive(Debug)]
pub struct HeadersError<'buf>(pub &'buf str);

impl<'buf> Error for HeadersError<'buf> {}

impl<'buf> Display for HeadersError<'buf> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Error parsing headers: {}", &self.0)
    }
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}
