plain
2019-10-25T00:39:45.2352382Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-10-25T00:39:45.2352446Z 
2019-10-25T00:39:45.3785420Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-10-25T00:39:45.3796492Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-10-25T00:39:45.3797106Z curl: (60) SSL certificate problem: unable to get local issuer certificate
2019-10-25T00:39:45.3797385Z More details here: https://curl.haxx.se/docs/sslcerts.html
2019-10-25T00:39:45.3797562Z 
2019-10-25T00:39:45.3797785Z curl failed to verify the legitimacy of the server and therefore could not
2019-10-25T00:39:45.3798007Z establish a secure connection to it. To learn more about this situation and
2019-10-25T00:39:45.3798262Z how to fix it, please visit the web page mentioned above.
2019-10-25T00:39:45.3833525Z 
2019-10-25T00:39:45.3959057Z ##[error]Bash exited with code '60'.
2019-10-25T00:39:45.4166094Z ##[section]Starting: Upload CPU usage statistics
2019-10-25T00:39:45.4285817Z ==============================================================================
2019-10-25T00:39:45.4285940Z Task         : Bash
2019-10-25T00:39:45.4286229Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-25T00:39:45.7607688Z ========================== Starting Command Output ===========================
2019-10-25T00:39:45.7613432Z [command]D:\a\msys2\usr\bin\bash.exe --noprofile --norc /d/a/_temp/370f548b-b1a8-4127-914f-ea68817d68af.sh
2019-10-25T00:39:45.8030773Z /d/a/_temp/370f548b-b1a8-4127-914f-ea68817d68af.sh: line 1: aws: command not found
2019-10-25T00:39:45.8031337Z 
2019-10-25T00:39:45.8074342Z ##[error]Bash exited with code '127'.
2019-10-25T00:39:45.8158890Z ##[section]Starting: Checkout
2019-10-25T00:39:45.8327168Z ==============================================================================
2019-10-25T00:39:45.8327281Z Task         : Get sources
2019-10-25T00:39:45.8327396Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
