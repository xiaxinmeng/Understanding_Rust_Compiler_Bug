
/tmp$ (sleep 1000 ; echo return code: $?) &
[1] 3006
/tmp$ pidof sleep
3008
/tmp$ kill -ABRT 3008
bash: line 6:  3008 Aborted                 sleep 1000
return code: 134
[1]+  Done                    ( sleep 1000; echo return code: $? )
