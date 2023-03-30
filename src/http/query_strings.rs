use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buffer> {
    data: HashMap<&'buffer str, Value<'buffer>>
}

#[derive(Debug)]
pub enum Value<'buffer> {
    Single(&'buffer str),
    Multiple(Vec<&'buffer str>),
}

impl<'buffer> QueryString<'buffer> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buffer> From<&'buffer str> for QueryString<'buffer> {
    fn from(s: &'buffer str) -> Self {
        let mut data = HashMap::new();
        for substring in s.split('&') {
            let mut key = substring;
            let mut val = "";
            if let Some(i) = substring.find('=') {
                key = &substring[..i];
                val = &substring[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing| {
                    match existing {
                        Value::Single(prev_value) => {
                            *existing = Value::Multiple(vec![prev_value, val]);
                        },
                        Value::Multiple(vec) => vec.push(val),
                        
                    } 
                })
                .or_insert(Value::Single(val)); 
        }

        QueryString { data }
    } 
}