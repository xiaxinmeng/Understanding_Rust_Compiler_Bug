
 29         if (task->blocked_on(&detach_cond)) {
 30             // XXX race here
 31             task->wakeup(&detach_cond);
 32         }
