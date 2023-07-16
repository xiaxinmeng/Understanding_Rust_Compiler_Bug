c++
__global__ void foo(float* foo) {
  __shared__ a float[];
  __shared__ b float[];  // ERROR: you can only have one pointer to shared memory per kernel
  foo[0] = a[0] + b[t0]; 
}
