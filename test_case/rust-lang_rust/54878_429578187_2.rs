c
mat[idxs[0]] = mat[0]; mat[0] = 7; /* from copy(&mat[0%2], &mat[idxs[0]]) */
mat[idxs[1]] = mat[1]; mat[1] = 7; /* from copy(&mat[1%2], &mat[idxs[1]]) */
mat[idxs[2]] = mat[0]; mat[0] = 7; /* from copy(&mat[2%2], &mat[idxs[2]]) */
