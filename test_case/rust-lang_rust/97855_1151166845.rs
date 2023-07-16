
       Since the standard input of a command opened for reading shares its seek offset with the process that  called  popen(),  if  the
       original  process  has  done a buffered read, the command's input position may not be as expected.  Similarly, the output from a
       command opened for writing may become intermingled with that of the original process.  The latter  can  be  avoided  by  calling
       fflush(3) before popen().
