
$ g++ -c a.cpp
$ g++ -c b.cpp
$ g++ a.o b.o -o foo
$ ./foo
caught exception in catch (...)
terminate called after throwing an instance of 'int'
Aborted (core dumped)
