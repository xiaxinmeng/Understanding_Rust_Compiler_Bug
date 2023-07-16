
error[E0597]: `v` does not live long enough
  --> src/main.rs:9:10
   |
9  |         &v
   |          ^ borrowed value does not live long enough
10 |         //~^ ERROR `v` does not live long enough [E0597]
11 |     });
   |     - `v` dropped here while still borrowed
12 |     println!("{:?}", x);
13 | }
   | - borrowed value needs to live until here

error: aborting due to previous error
