
#define NO_EINTR(stmt) while ((stmt) == -1 && errno == EINTR);
