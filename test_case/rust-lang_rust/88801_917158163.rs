C
// $ cat calling_conventions.c
__cdecl __declspec(dllexport) int cdeclSum(int a, int b)
{
  return a + b;
}

__stdcall __declspec(dllexport) int stdcallSum(int a, int b)
{
  return a + b;
}

__fastcall __declspec(dllexport) int fastcallSum(int a, int b)
{
  return a + b;
}
