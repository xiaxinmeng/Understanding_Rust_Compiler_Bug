
// [0,3] -> 0,1,2,3 -> count from 0 to 3 including `[]` 0 and 3
// (0,3) ->   1,2   -> count from 0 to 3 excluding `()` 0 and 3
// (0,3] ->   1,2,3 -> combo

// So:
// [0,7) -> 0,1,2,3,4,5,6
// [0,3) -> 0,1,2,        -> ends at 3
// [3,7) ->       3,4,5,6 -> also starts at 3

// Whereas:
// [0,6] -> 0,1,2,3,4,5,6
// [0,2] -> 0,1,2,        -> ends at 2
// [3,6] ->       3,4,5,6 -> starts at 3 which is different from 2
// Which means with `[start,stop]`, you'll always have to work harder to align ranges
// because they don't start and end at the same point

// It also means:
// [2,6) -> 2,3,4,5 -> has 4 (6-2) elements
// [2,5] -> 2,3,4,5 -> has 4 (5-2+1) elements

// [612, 5244) -> how many elements?
