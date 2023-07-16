c
#include <windows.h>
#include <stdio.h>

int main() {
	HMODULE lib = LoadLibraryA("ecsde/target/release/ecs_game.dll");
	if (lib) {
		printf("Hurray!\n");
	} else {
		DWORD last_err = GetLastError();
		printf("Failed to load. Last err = %d\n", last_err);
	}
}
