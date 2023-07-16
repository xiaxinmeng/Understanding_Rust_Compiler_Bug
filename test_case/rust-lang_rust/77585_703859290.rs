
   171          if self.head.is_null() {
   172              // allocate the guard node if not present
   173              unsafe {
-> 174                  self.head = Box::into_raw(Box::new(mem::uninitialized()));
   175                  (*self.head).next = self.head;
   176                  (*self.head).prev = self.head;
   177              }
