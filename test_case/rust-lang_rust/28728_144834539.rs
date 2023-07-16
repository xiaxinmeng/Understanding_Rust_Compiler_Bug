
 The implementation may assume that any thread will eventually do one of the following:
   - terminate
   - make a call to a library I/O function
   - access or modify a volatile object, or
   - perform a synchronization operation or an atomic operation

 [Note: This is intended to allow compiler transformations such as removal of empty loops, even
  when termination cannot be proven. — end note ]
