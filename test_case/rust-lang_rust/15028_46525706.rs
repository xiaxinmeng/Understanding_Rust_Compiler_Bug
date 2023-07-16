 C++
wstring read() {
    auto in = GetStdHandle(STD_INPUT_HANDLE);
    array<wchar_t, 0x1000> buf;
    unsigned long read;
    ReadConsoleW(in, buf.data(), buf.size(), &read, nullptr);
    return{buf.begin(), buf.begin() + read};
}

void write(wstring p_str) {
    auto out = GetStdHandle(STD_OUTPUT_HANDLE);
    unsigned long wrote;
    WriteConsoleW(out, p_str.c_str(), p_str.size(), &wrote, nullptr);
}

int main() {
    write(read());
}
