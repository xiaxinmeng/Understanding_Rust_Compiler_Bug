
task::task().linked().spawn() // Its own group, whose handle we never see
let tg = task::taskgroup();
task::task().linked_to(tg).spawn() // Explicit handle management
