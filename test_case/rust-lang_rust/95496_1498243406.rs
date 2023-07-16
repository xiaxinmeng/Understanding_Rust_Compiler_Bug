
       The dirent structure definition shown above is taken from the
       glibc headers, and shows the d_name field with a fixed size.

       Warning: applications should avoid any dependence on the size of
       the d_name field.  POSIX defines it as char d_name[], a character
       array of unspecified size, with at most NAME_MAX characters
       preceding the terminating null byte ('\0').
