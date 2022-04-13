// This file holds query_string struct and its functionality

use std::collections::HashMap;


pub struct QueryString<'buf>
{
    // Hashmap
    data: HashMap<&'buf str, Value<'buf>>,
}

pub enum Value<'buf>
{   
    Single(&'buf str),
    // Heap allocated dynamic array = Vector 
    Multiple(Vec<&'buf str>),
}

// Function on query string that reads keys from the hash map
impl<'buf> QueryString<'buf>
{
    pub fn get(&self, key: &str) -> Option<&Value>
    {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf>
{
    fn from(s: &'buf str) -> Self
    {
        // Empty hash map to fill
        let mut data = HashMap::new();

        // Splitting string on some pattern
        for sub_str in s.split('&')
        {
            let mut key = sub_str;
            let mut val = "";

            // Look for an equal sign
            if let Some(i) = sub_str.find('=')
            {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            // Checking if they key already exists in hash map
            data.entry(key)
            .and_modify(|existing: &mut Value| match existing
            {
                Value::Single(prev_value) => {
                                                *existing = Value::Multiple(vec![prev_value, val]); 
                                             }
                Value::Multiple(vec) => vec.push(val)
            })
            .or_insert(Value::Single(val));
            
        
        }

        QueryString { data }

    }
}