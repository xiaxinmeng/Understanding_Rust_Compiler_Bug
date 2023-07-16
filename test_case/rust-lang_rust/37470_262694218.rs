
  // Pseudo code for iterator
  if (hash.size == 0) return
  ptis = ptis * 5 + 1 //per_thread_iteration_seed
  delta := ptis * 2 + 1
  mult := (ptis ^ (ptis>>16)) * 4 + 1
  i := 0
  do {
    if (slot_occupied(hash, i))
       yield hash.key[i]
    i = (i * mult + delta) & hash.mask
   } until (i != 0)
