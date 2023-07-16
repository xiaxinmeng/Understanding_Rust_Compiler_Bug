
$ A=b bash -c 'echo ${A-not set}'
b
$ A=b && bash -c 'echo ${A-not set}'
not set
