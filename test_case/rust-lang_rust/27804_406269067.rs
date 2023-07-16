python
# to_visit = set(...)
while to_visit:  # while non-empty:
    current_one = to_visit.pop()
    do_slow_stuff_with(current_one)
    if some_cond(current_one):
        another_one = some_fast_function(current_one)
        to_visit.discard(another_one)
