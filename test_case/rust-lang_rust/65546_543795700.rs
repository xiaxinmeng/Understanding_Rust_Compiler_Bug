c
fiftyoneDegreesWorkset *ws = NULL;
if (pool->available > 0) {
    // Worksets are available. Take one from the end of the array.
    ws = pool->worksets[pool->available - 1];
    pool->available--;
}
return ws;
