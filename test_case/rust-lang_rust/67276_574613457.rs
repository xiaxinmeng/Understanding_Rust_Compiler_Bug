
    tokio_timer.rlib         regex_syntax.rlib
         ^                     ^      ^
         |                     |      |
         +-----------+---------+      |
                     |             env_logger.rlib
                 shared.so            ^
                     ^                |
                     |                |
                     +-------+--------+
                             | 
                         serverctl.exe
