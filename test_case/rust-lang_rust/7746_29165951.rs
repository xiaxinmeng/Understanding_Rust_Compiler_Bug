
goForwards = yield goForwards ? start : end;
if goForwards {
  start += 1;
} else {
  end -= 1;
}
