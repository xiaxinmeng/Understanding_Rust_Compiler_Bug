
/* When static files are updated, their suffixes need to be updated.
	1. In the top directory run:
		./x.py doc --stage 1 library/core
	2. Find the directory containing files named with updated suffixes:
		find build -path '*'/stage1-std/'*'/static.files
	3. Copy the filenames with updated suffixes from the directory.
*/
