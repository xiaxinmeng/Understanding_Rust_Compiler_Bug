cuda
#include <cstdint>
#include <cstdio>

__global__ void foo(uint32_t x, uint64_t * y) {
	asm volatile (
		"mov.u64 %0, %1;"
		: "=l"(*y) : "l"(x)
	);
}

__host__ int main() {
	uint64_t y = UINT64_MAX;
	uint64_t * yd;
	cudaMalloc(&yd, sizeof(uint64_t));
	cudaMemcpy(yd, &y, sizeof(uint64_t), cudaMemcpyHostToDevice);
	foo<<<1, 1>>>(0xBEEF, yd);
	cudaMemcpy(&y, yd, sizeof(uint64_t), cudaMemcpyDeviceToHost);
	cudaFree(yd);
	printf("%lx\n", y);
}
