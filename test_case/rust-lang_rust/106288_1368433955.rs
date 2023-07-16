
  %0 = getelementptr inbounds i32, ptr %s.0, i64 %s.1
  %1 = ptrtoint ptr %0 to i64
  %2 = ptrtoint ptr %s.0 to i64
  %3 = sub nuw nsw i64 -4, %2
  %4 = add i64 %3, %1
