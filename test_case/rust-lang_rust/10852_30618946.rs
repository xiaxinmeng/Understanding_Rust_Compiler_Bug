
C++:
push front all: list.insert(list.begin(), x.begin(), x.end())  
push back all: list.insert(list.end(), x.begin(), x.end())

Java:
push back all: list.addAll(x)

Python:
push back all: list.extend(xs)

Scala:
push front all: buf ++=: xs
push back all: xs ++= buf
