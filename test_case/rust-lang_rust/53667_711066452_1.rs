
if parts.len() == 2 { if let Some(n) = name { if let Some(Ok(a)) = age { 
            return Person{name: n.to_owned().to_string(),  age : a,}; 
        }}}
