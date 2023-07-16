
  cd bar
  cargo test # passes
  cd ..
  cargo test -p bar # passes 
  rm -rf target
  cargo test -p bar # fails
  