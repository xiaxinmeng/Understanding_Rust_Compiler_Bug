
typedef int (__stdcall *PFN_ADD) (int x, int y);
PFN_ADD pAdd = (PFN_ADD) GetProcAddress(hDllModule, "add"); // load fn by name
pAdd(1, 2);
