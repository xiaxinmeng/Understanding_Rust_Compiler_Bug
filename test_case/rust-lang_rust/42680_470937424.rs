
#include <stdio.h>
#include <stdlib.h>
#include <spawn.h>
#include <unistd.h>
#include <sys/types.h>
#include <signal.h>
#include <errno.h>
#include <string.h>

int main() {
	char buf[401];
	sigset_t s;
	sigset_t olds;
	posix_spawn_file_actions_t action;
	int cout_pipe[2];
	int cin_pipe[2];
	pid_t child;
	char* args[] = {
		"cat",
		NULL,
	};

	sigemptyset(&s);
	sigaddset(&s, SIGINT);
	pthread_sigmask(SIG_SETMASK, &s, &olds);
	if (pipe(cout_pipe))
		return 1;
	if (pipe(cin_pipe))
		return 1;
	posix_spawn_file_actions_init(&action);
	posix_spawn_file_actions_addclose(&action, cout_pipe[0]);
	posix_spawn_file_actions_adddup2(&action, cout_pipe[1], 1);
	posix_spawn_file_actions_addclose(&action, cin_pipe[1]);
	posix_spawn_file_actions_adddup2(&action, cin_pipe[0], 0);

	if(posix_spawnp(&child, "cat", &action, NULL, args, NULL) != 0) {
		perror("posix_spawnp");
		return 2;
	}
	close(cout_pipe[1]);
	close(cin_pipe[0]);

	kill(child, SIGINT);

	write(cin_pipe[1], "Hello", strlen("Hello"));
	close(cin_pipe[1]);
	int rc = read(cout_pipe[0], buf, 400);
	if (rc == -1) {
		if(errno != EPIPE) {
			perror("read");
			return 3;
		}
	} else if (rc != 0) {
		printf("test failure: read %d\n", rc);
		return 4;
	}
	return 0;
}
