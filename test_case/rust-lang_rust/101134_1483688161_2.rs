c
int foo(int, int);

...
DriverEntry(
    _In_ PDRIVER_OBJECT  DriverObject,
    _In_ PUNICODE_STRING RegistryPath
    )
{
...
foo(1,1);
...
}
