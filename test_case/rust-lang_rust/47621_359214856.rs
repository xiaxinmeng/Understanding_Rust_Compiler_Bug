diff
- let entry: [u8; 65] = unsafe { mem::transmute(entry) };
- self.write(&entry);
+ let ptr = &entry as *const Entry<T> as *const u8;
+ let size = std::mem::size_of::<Entry<T>>();
+ let bytes = unsafe { std::slice::from_raw_parts(ptr, size) };
+ self.write(bytes);
