
error: couldn't allocate input reg for constraint 'a'
 --> <anon>:7:14
  |
7 |     unsafe { asm!("out %al, %dx" :: "a" (byte), "d" (port) :: "volatile"); }
  |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
