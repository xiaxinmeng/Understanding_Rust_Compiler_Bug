
# netstat  -plan | grep 8080
tcp        0      0 127.0.0.1:8080          0.0.0.0:*               LISTEN      975/streamtest  
# telnet 0 8080
Trying 0.0.0.0...
Connected to 0.
Escape character is '^]'.
Connection closed by foreign host.

# netstat  -plan | grep 8080
tcp        0      0 127.0.0.1:8080          0.0.0.0:*               LISTEN      975/streamtest  
tcp        0      0 127.0.0.1:8080          127.0.0.1:38801         TIME_WAIT   -               
