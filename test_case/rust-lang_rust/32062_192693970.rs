
$0 <=> $1 // $1 becomes the "parent"
normalize_projection_type($0::Type) // Cache miss, inserts $1::Type
normalize_projection_type($1::Type) // Cache hit on $1::Type
