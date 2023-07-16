
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
0.04s$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Oct  2 20:13:25 UTC 2017
Mon, 02 Oct 2017 20:13:25 GMT
