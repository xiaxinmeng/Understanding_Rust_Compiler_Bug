
  void uv__run_idle(uv_loop_t* loop) {                                      \
    uv_idle_t* h;                                                         \
    ngx_queue_t* q;                                                           \
    ngx_queue_foreach(q, &loop->idle_handles) {                             \
      h = ngx_queue_data(q, uv_idle_t, queue);                            \
      h->idle_cb(h, 0);                                                     \
    }                                                                         \
  } 
