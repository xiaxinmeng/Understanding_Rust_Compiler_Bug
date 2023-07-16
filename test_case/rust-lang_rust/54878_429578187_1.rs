c
for (int i = 0; i < 3; i++) {
    mat[idxs[i]] = mat[i%2]; mat[i%2] = 7;
}
