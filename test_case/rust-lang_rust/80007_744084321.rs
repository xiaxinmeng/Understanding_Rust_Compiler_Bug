diff
- let arc = Arc::new(Mutex::new(value));
+ let arc = Mutex::arc(value);
