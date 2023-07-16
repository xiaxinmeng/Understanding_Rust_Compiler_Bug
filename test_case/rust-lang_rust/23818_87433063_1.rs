
 At program startup, three text streams are predefined and need not be opened explicitly
— standard input (for reading conventional input), standard output (for writing
conventional output), and standard error (for writing diagnostic output). As initially
opened, the standard error stream is not fully buffered; the standard input and standard
output streams are fully buffered if and only if the stream can be determined not to refer
to an interactive device.
