
  // Pseudo code for iterator
  if (hash.size == 0) return
  i := 0
  do {
    if (slot_occupied(hash, i))
       yield hash.key[i]
    i = (i *5 + 1) & hash.mask
  } until (i != 0) /* returned to first position, so we walked all hash */
