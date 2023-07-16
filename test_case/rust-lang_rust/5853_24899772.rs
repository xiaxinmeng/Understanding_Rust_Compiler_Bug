 c++
class S {
  int x;
  virtual void foo();
  void bar();
}
class T : public S {
  int y;
  void baz();
}
void process(S* arg);
S* getS();
