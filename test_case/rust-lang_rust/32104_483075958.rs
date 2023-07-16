bash
$ base64 ../images/mypicture.png  | awk ' { print "/// " $1 } ' > myfile.b64
