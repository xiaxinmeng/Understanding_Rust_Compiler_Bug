
-	
-	Another case that causes this error is when a type is imported into a parent
-	module. To fix this, you can follow the suggestion and use File directly or
-	`use super::File;` which will import the types from the parent namespace. An
-	example that causes this error is below:
-	
-	