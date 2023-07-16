
RETURN VALUE
       The setenv() function returns zero on success, or  -1  on  error,  with
       errno set to indicate the cause of the error.
       The  unsetenv()  function returns zero on success, or -1 on error, with
       errno set to indicate the cause of the error.
ERRORS
       EINVAL name is NULL, points to a string of length 0, or contains an '='
              character.
       ENOMEM Insufficient memory to add a new variable to the environment.
CONFORMING TO
       4.3BSD, POSIX.1-2001.
BUGS
       POSIX.1-2001  specifies  that  if  name contains an '=' character, then
       setenv() should fail with the error EINVAL; however, versions of  glibc
       before 2.3.4 allowed an '=' sign in name.
