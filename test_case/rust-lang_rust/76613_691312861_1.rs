rust
u.iter() 
   .filter(|x| x.is_some()) 
   .map(|x| x.unwrap()) 
   .collect()
