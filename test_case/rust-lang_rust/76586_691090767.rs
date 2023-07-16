c++
#include <iostream>
#include <fstream>

using namespace std;

int main()
{
    ofstream myFile("C:\\Users\\testing.user\\.bvm\\bins\\nodejs-node-8.12.0\\node-v8.12.0-win-x64\\node_modules\\npm\\test\\npm_cache\\_cacache\\content-v2\\sha512\\32\\c8\\6992ba4671408a292bb3f2fae4cd10416dcedb6a214c169054af6bcc792b6ea56f7245333357cc59e4f84660380604b5ff338258ad46973218d864799b5e");
    myFile << "test";
    myFile.close();
    std::cout << "Wrote\n"; // this outputs, but the file is not created

    return 0;
}
