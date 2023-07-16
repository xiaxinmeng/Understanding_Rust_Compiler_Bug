shell
➜  temp cat i.c
int main() {
  return 0;
}
➜  temp clang -target x86_64-apple-ios-macabi i.c
➜  temp echo $?
0
