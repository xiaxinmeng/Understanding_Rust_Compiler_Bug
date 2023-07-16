
Array:

java.util.Collection:
push: collection.add(x)

java.util.List:
push front: list.add(0, x)
push back: list.add(x)
pop front: list.remove(0)
pop back: list.remove(ArrayList.size() - 1)

java.util.Stack:
push back’: stack.push(x)
pop back: stack.pop()

java.util.Queue:
push front: queue.add(x)
pop back: queue.remove()

java.util.Deque:
push front: deque.addFirst(x)
push back: deque.addLast(x)
pop front: deque.removeFirst()
pop back: deque.removeLast()
