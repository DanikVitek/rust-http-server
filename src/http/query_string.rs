use std::collections::HashMap;

// a=1&b=2&c&d=&e===&d=7&d=abc
// ("a", "1")
// ("b", "2")
// ("c", "")
// ("d". ["", "7", "abc"])
// ("e". "==")

#[derive(Debug)]
pub struct QueryString<'buf>(HashMap<&'buf str, Value<'buf>>);

impl<'buf> std::fmt::Display for QueryString<'buf> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(for (i, (key, value)) in self.0.iter().enumerate() {
            match value {
                Value::Single(val) => write!(f, "{}={}", key, val)?,
                Value::Multiple(vec) => {
                    for (i, val) in vec.iter().enumerate() {
                        write!(f, "{}={}", key, val)?;
                        if i + 1 < vec.len() {
                            write!(f, "&")?;
                        }
                    }
                }
            }
            if i + 1 < self.0.len() {
                write!(f, "&")?;
            }
        })
    }
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &'buf str) -> Option<&Value> {
        self.0.get(key)
    }
}

// pretty much every string is a valid query string
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        s.split('&').for_each(|pair| match pair.find('=') {
            Some(amp_ind) => {
                let (key, val) = pair.split_at(amp_ind);
                add_value(&mut data, key, &val[1..]);
            }
            None => add_value(&mut data, pair, ""),
        });

        Self(data)
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
