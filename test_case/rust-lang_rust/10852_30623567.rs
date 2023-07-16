
QList:
push front: list.prepend(x)
push back: list.append(x), list += x, list << x
pop front: list.removeFirst()
pop back: list.removeLast()
push back all: list.append(xs), list += xs, list << xs

QVector:
push front: vec.prepend(x)
push back: vec.append(x), vec += x, vec << x
pop front: vec.removeFirst()
pop back: vec.removeLast()
push back all: vec += xs, vec << xs

QLinkedList:
push front: list.prepend(x)
push back: list.append(x), list += x, list << x
pop front: list.removeFirst()
pop back: list.removeLast()
push back all: list += xs, list << xs
