 c
typedef struct {
  uv_work_t work_req;
  int errorno;
  char resolved_path[PATH_MAX];
  char path[1];
} realpath_req;

static void work_cb(uv_work_t *req);
static void done_cb(uv_work_t *req, int status);

void fs_realpath(const char *path) {
  size_t len = strlen(path);
  realpath_req *req = malloc(sizeof(*req) + len);
  memcpy(req->path, path, len + 1);
  uv_queue_work(uv_default_loop(), &req->work_req, work_cb, done_cb);
}

static void work_cb(uv_work_t *work_req) {
  realpath_req *req = container_of(work_req, realpath_req, work_req);
  req->errno = 0;
  if (realpath(req->path, req->resolved_path) == NULL)
    req->errorno = errno;
}

static void done_cb(uv_work_t *req, int status) {
  realpath_req *req = container_of(work_req, realpath_req, work_req);
  (void) &status;  // Only relevant when calling uv_cancel().
  // ...
  free(req);
}
