import os


# reponame = "rust-lang/rust"
reponame = "Rust-GCC/gccrs"

project_name = reponame.split("/")[1]


project_path = "/home/xxm/Desktop/empirical_bug_rust_compiler/project" + "/" + reponame.replace("/","_") + "/" + project_name 



save_commit_sha_path = '/home/xxm/Desktop/empirical_bug_rust_compiler/commit_sha' + "/"+  reponame.replace("/","_") + ".json"

os.system("cd {} && git log --pretty=format:'%H' > {}".format(project_path, save_commit_sha_path))
