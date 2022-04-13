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

// Function on query string that reads keys from the hashmap
impl<'buf> QueryString<'buf>
{
    pub fn get(&self, key: &str) -> Option<&Value>
    {
        self.data.get(key)
    }
}