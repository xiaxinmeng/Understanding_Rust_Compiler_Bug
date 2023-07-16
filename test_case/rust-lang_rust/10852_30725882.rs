
IList:
push front: v.Insert(0, x)
push back: v.Add(x)
pop front: v.RemoveAt(0, x)
pop back: v.RemoveAt(v.Count(), x)

ArrayList: (include IList)
push front all: v.InsertRange(0, collection)
push back all: v.InsertRange(v.Count(), collection)

Queue:
push front: q.Enqueue()
pop back: q.Dequeue()

Stack:
push back: s.Push()
pop back: s.Pop()
