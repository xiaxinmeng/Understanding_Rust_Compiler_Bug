cpp
#include <windows.h>
#include <tchar.h>

int WINAPI WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR lpCmdLine, int nCmdShow)
{
  MessageBox(NULL, GetCommandLine(), TEXT("Arguments"), MB_OK);
  return 0;
}

