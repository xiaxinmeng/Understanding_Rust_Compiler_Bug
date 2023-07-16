
if state.swap(NOTIFIED) != PARKED:
    return
with mutex:
    notify
