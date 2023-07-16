
       A  successful  close  does not guarantee that the data has been success‐
       fully saved to disk, as the  kernel  uses  the  buffer  cache  to  defer
       writes.   Typically,  filesystems  do  not  flush buffers when a file is
       closed.  If you need to be sure that the data is  physically  stored  on
       the underlying disk, use fsync(2).  (It will depend on the disk hardware
       at this point.)
