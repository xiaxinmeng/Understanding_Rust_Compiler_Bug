from github import Github
import random
import sys
import os
import json
import requests


# reponame = "rust-lang/rust"
reponame = "Rust-GCC/gccrs"

commit_sha_file = "/home/xxm/Desktop/empirical_bug_rust_compiler/commit_sha/" + reponame.replace("/","_")+ ".json"
save_commit_path = '/home/xxm/Desktop/empirical_bug_rust_compiler/commit'

save_commit_dir = save_commit_path +"/"+reponame.replace("/","_")
if not os.path.exists(save_commit_dir):
	os.makedirs(save_commit_dir)



use_agent=[              "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/536.6 (KHTML, like Gecko) Chrome/20.0.1092.0 Safari/536.6",
                         "Mozilla/5.0 (Windows NT 6.0) AppleWebKit/536.5 (KHTML, like Gecko) Chrome/19.0.1084.36 Safari/536.5",
                         "Mozilla/5.0 (Windows NT 6.1) AppleWebKit/536.3 (KHTML, like Gecko) Chrome/19.0.1061.1 Safari/536.3",
                         "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/535.24 (KHTML, like Gecko) Chrome/19.0.1055.1 Safari/535.24",
                         "Mozilla/5.0 (Windows NT 6.2) AppleWebKit/536.3 (KHTML, like Gecko) Chrome/19.0.1061.0 Safari/536.3",
                         "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/535.1 (KHTML, like Gecko) Chrome/14.0.835.163 Safari/535.1",
						 "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/73.0.3683.103 Safari/537.36",
						 "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_7_0) AppleWebKit/535.11 (KHTML, like Gecko) Chrome/17.0.963.56 Safari/535.11",
						 "Mozilla/5.0 (Windows NT 6.1; WOW64; rv:6.0) Gecko/20100101 Firefox/6.0",
						 "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:2.0.1) Gecko/20100101 Firefox/4.0.1",
						 "Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10_6_8; en-us) AppleWebKit/534.50 (KHTML, like Gecko) Version/5.1 Safari/534.50",
						 "Mozilla/5.0 (Windows; U; Windows NT 6.1; en-us) AppleWebKit/534.50 (KHTML, like Gecko) Version/5.1 Safari/534.50",
						 "Opera/9.80 (Macintosh; Intel Mac OS X 10.6.8; U; en) Presto/2.8.131 Version/11.11",
						 "Opera/9.80 (Windows NT 6.1; U; en) Presto/2.8.131 Version/11.11",
						 "Mozilla/5.0 (Windows NT 10.0; WOW64; Trident/7.0; rv:11.0) like Gecko",
						 "Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; Trident/5.0)",
						 "Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.0; Trident/4.0)",
						 "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 6.0)",
						 "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1)",
						 "Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 5.1; Trident/4.0; GTB7.0)",
						 "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1)",
						 "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; SV1)",
						 "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1; Maxthon 2.0)",
						 "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1; Trident/4.0; SE 2.X MetaSr 1.0; SE 2.X MetaSr 1.0; .NET CLR 2.0.50727; SE 2.X MetaSr 1.0)",
						 "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1; 360SE)",
						 "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1; TencentTraveler 4.0)"]
      
#xiaxinmeng1993@hotmail.com
authenticity_token = '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'   

#663979162@qq.com
# authenticity_token = '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'    

#xiaxm@smail.nju.edu.cn
#authenticity_token = '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' 

# huantan628   
# authenticity_token =  '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'      
                    

g = Github(login_or_token=authenticity_token,user_agent= use_agent[random.randint(0,25)])

print(g.rate_limiting,g.get_rate_limit())

repo = g.get_repo(reponame)




commits_sha = open(commit_sha_file,'r').read().split("\n")

commits_sha.sort()



count =43410
maxissue = 50000
while count <= maxissue:
	commit_sha = commits_sha[count-1]
	print(len(commits_sha) - count, "left.., Now it is ", count )
	if commit_sha:
		savefile = save_commit_dir +"/"+commit_sha+".json"
		commit = repo.get_commit(sha=commit_sha)
		json.dump(commit.raw_data,open(savefile,'w'))

	count = count + 1
	
	print(g.rate_limiting,g.get_rate_limit())










