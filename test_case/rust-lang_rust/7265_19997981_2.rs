
329             do Local::borrow::<Scheduler, ()> |sched| {
330                 if sched.in_task_context() {
331                     context.put_back(TaskContext);
332                 } else {
333                     context.put_back(SchedulerContext);
334                 }
335             }
336             return context.take();
337         } else {
338             return GlobalContext;
339         }
340     }
