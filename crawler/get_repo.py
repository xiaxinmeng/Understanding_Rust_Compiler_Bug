import sys
import os

# reponame = "rust-lang/rust"
# git_addr = "https://github.com/rust-lang/rust.git"

reponame = "Rust-GCC/gccrs"
git_addr = "https://github.com/Rust-GCC/gccrs.git"




project_dir = '/home/xxm/Desktop/empirical_bug_rust_compiler/project' + "/" + reponame.replace("/","_")
if not os.path.exists(project_dir):
	os.makedirs(project_dir)



os.system("cd %s;git clone %s"%(project_dir,git_addr))