 c
#include<sys/stat.h>

int main(){
    struct stat sb;
    stat("tc", &sb);
    printf("%d %d\n", S_ISLNK(sb.st_mode), (sb.st_mode & S_IFMT) == S_IFLNK);
    return 0;
}
