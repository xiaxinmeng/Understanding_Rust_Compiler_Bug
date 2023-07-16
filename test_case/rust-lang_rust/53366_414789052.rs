
if state.cas(NOTIFIED, EMPTY): // optional early out
    return
if not state.cas(EMPTY, PARKED):
    state.swap(EMPTY)
    return
with mutex:
    while not state.cas(NOTIFIED, EMPTY):
        wait
