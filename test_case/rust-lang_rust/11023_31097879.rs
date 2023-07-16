 c
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <uv.h>
#include <uv-errno.h>

int main() {
    uv_loop_t *loop = uv_loop_new();
    int uv_err;

    uv_process_t child_req;
    uv_process_options_t spawn_options = { 0 };
    char *args[] = { "no-binary-by-this-name-should-exist", NULL };
    spawn_options.file = args[0];
    spawn_options.args = args;
    if (uv_err = uv_spawn(loop, &child_req, &spawn_options)) {
        fprintf(stderr, "%s (%d)\n", uv_strerror(uv_err), uv_err);
        return EXIT_FAILURE;
    }

    return uv_run(loop, UV_RUN_DEFAULT);
}
