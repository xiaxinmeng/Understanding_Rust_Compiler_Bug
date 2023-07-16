

"..".split('.') = ["", "", ""]
 ".".split('.') = ["", ""]
  "".split('.') = [""]

[0, 0].split(|_| true) = [[], [], []]
   [0].split(|_| true) = [[], []]
    [].split(|_| true) = [[]]

"..".split_inclusive('.') = [".", "."]
 ".".split_inclusive('.') = ["."]
  "".split_inclusive('.') = []

[0, 0].split_inclusive(|_| true) = [[0], [0]]
   [0].split_inclusive(|_| true) = [[0]]
    [].split_inclusive(|_| true) = [[]]

"x.x.x".split_inclusive('.') = ["x.", "x.", "x"]
  "x.x".split_inclusive('.') = ["x.", "x"]
    "x".split_inclusive('.') = ["x"]

[1, 0, 1, 0, 1].split_inclusive(|x| *x == 0) = [[1, 0], [1, 0], [1]]
      [1, 0, 1].split_inclusive(|x| *x == 0) = [[1, 0], [1]]
            [1].split_inclusive(|x| *x == 0) = [[1]]
