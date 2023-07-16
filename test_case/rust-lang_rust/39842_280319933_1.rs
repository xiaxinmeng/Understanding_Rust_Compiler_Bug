
If you use pipe between parent and child processes, maybe

what you really need is `wait_with_output` rather than `wait` 

because if the pipe is full, the child process maybe block forever. 
