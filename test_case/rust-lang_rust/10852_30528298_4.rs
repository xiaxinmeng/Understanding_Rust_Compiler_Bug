
Seq:
push front: x +=: buf
push back: buf += x
pop front: buf.remove(0)
pop back: buf.remove(buf.size - 1)
