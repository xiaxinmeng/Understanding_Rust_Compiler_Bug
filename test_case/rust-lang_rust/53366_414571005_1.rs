
if state.cas(NOTIFIED, EMPTY):
    return
state = PARKED
with mutex:
    while not state.cas(NOTIFIED, EMPTY):
        wait
