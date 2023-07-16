c++
// http://en.cppreference.com/w/cpp/language/storage_duration
// msvc on win64: cl tls.cpp /link
// mingw on linux64: x86_64-w64-mingw32-g++ tls.cpp -static -static-libstdc++

#include <iostream>
#include <string>
#include <thread>
#include <mutex>

thread_local unsigned int rage = 1;
std::mutex cout_mutex;

void increase_rage(const std::string& thread_name)
{
	++rage; // modifying outside a lock is okay; this is a thread-local variable
	std::lock_guard<std::mutex> lock(cout_mutex);
	std::cout << "Rage counter for " << thread_name << ": " << rage << '\n';
}

int main()
{
	std::thread a(increase_rage, "a"), b(increase_rage, "b");

	{
		std::lock_guard<std::mutex> lock(cout_mutex);
		std::cout << "Rage counter for main: " << rage << '\n';
	}

	a.join();
	b.join();

	getchar();
}
