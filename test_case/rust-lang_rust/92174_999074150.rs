rust
for (i, vi) in arr.iter().enumerate(){
   for (j, vj) in arr[..i].iter().enumerate(){
        ...
   }
}
