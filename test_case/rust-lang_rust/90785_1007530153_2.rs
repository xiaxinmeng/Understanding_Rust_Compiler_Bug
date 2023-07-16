c
old_stdout = dup(1);
old_stderr = dup(2);

tmp = mktemp(…);
tmp_fd = open(tmp);

dup2(tmp_fd, 1);
dup2(tmp_fd, 2);

success = run_the_test();

dup2(old_stdout, 1);
dup2(old_stderr, 2);

if !success {
    fseek(tmp_fd, 0);
    cat(tmp_fd);
}

close(tmp_fd);
unlink(tmp);
