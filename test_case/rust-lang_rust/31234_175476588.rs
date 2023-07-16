 llvm
@ref = constant i64 0
@const = constant i64* @ref
...
  %1 = load i64**, i64*** bitcast (i64** @const to i64***)
  %2 = call i64 @method(i64** %1)
