text
            List<i32> as Send
               /         \
              /           \
             /             \
 Rc<T> as Send             Option<Arc<List<T>>> as Send
