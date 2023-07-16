c
    let hNullDev = CreateFileW(L"NUL:", GENERIC_WRITE, FILE_SHARE_READ|FILE_SHARE_WRITE, 0, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, 0);
    if (hNullDev == INVALID_HANDLE_VALUE) return Err(GetLastError());
    if (SetStdHandle(STD_OUTPUT_HANDLE, hNullDev)) {
        int e = GetLastError();
        CloseHandle(hNullDev);
        return Err(e);
    }
    CloseHandle(hNullDev);
    return Ok;
