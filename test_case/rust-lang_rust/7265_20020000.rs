
 635             let last_task_context = match last_task {
 636                 Some(t) => Some(&mut t.saved_context), None => None
 637             };
 638             let next_task_context = match self.current_task {
 639                 Some(ref mut t) => Some(&mut t.saved_context), None => None
 640             };
