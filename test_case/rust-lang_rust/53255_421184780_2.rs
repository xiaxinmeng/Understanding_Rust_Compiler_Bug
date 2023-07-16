text
            List<i32> as Send (Error)
               /         \
              /           \
             /             \
 Rc<T> as Send (Error)     Option<Arc<List<T>>> as Send
