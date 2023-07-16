
error: `y` does not live long enough
 --> 1.rs:5:11
  |
5 |      x = &y;
  |           ^ does not live long enough
6 |    }
  |    - borrowed value only lives until here
7 | }
  | - borrowed value needs to live until here
