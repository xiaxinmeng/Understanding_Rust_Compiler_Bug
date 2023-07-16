
[root@host ~]# rm -f /proc/version 
rm: cannot remove '/proc/version': Operation not permitted

[root@host ~]# echo "test" >>/proc/version 
-bash: echo: write error: Input/output error
