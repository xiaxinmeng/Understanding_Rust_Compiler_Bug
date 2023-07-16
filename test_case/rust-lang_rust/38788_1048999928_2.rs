cuda
struct foo {
    float array[9];
};

extern "C" __global__ void add( float *a, struct foo b) {
    *a = b.array[5];
}
