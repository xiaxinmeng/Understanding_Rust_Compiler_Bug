
#include <iostream>

using namespace std;

struct MyStruct {
  bool a;
};

MyStruct GetMyStruct()
{
  MyStruct a;
  a.a=true;
  return a;
}

int main()
{
  const MyStruct mystruct = GetMyStruct();
  cout<<mystruct.a<<"\n";
  return 0;
}
