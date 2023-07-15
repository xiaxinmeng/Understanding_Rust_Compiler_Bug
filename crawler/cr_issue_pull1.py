from github import Github
import random
import sys
import os
import json
import requests


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










reponame = "rust-lang/rust"
# reponame = "Rust-GCC/gccrs"

save_issue_path = '/home/xxm/Desktop/empirical_bug_rust_compiler/issue'
save_pull_path = '/home/xxm/Desktop/empirical_bug_rust_compiler/pull'
save_pull_to_commit_path = '/home/xxm/Desktop/empirical_bug_rust_compiler/pull_to_commit'




save_issue_dir = save_issue_path + '/'+reponame.replace("/",'_')
if not os.path.exists(save_issue_dir):
	os.makedirs(save_issue_dir)

save_pull_dir = save_pull_path + '/'+reponame.replace("/",'_')
if not os.path.exists(save_pull_dir):
	os.makedirs(save_pull_dir)

save_pull_to_commit_dir = save_pull_to_commit_path+ '/'+reponame.replace("/",'_')
if not os.path.exists(save_pull_to_commit_dir ):
	os.makedirs(save_pull_to_commit_dir )

# 通过用户名和仓库名获取仓库对象
repo = g.get_repo(reponame)



count = 84480
maxissue = 84585
while count <= maxissue:
	issueID = count
	print(issueID)

	issue = repo.get_issue(number = issueID)


	if issue.pull_request is None:
		#获取特定的 issue
		issue_raw_data = issue.raw_data
		# print(f"Issue: {issueID}, Title: {issue.title}")

		save_issue_file = save_issue_dir +"/"+ str(issueID)+".json"
		
		json.dump(issue_raw_data,open(save_issue_file,'w'))




	else:
		#获取特定的 pull
		# print(f"Pull Request: {issue.number}, Title: {issue.title}")
		pull = repo.get_pull(number = count)
		pull_raw_data = pull.raw_data
		# print(issue.raw_data,"\n\n\n\n",pull.raw_data)

		save_pull_file = save_pull_dir +"/"+ str(issueID)+".json"
		
		json.dump(pull_raw_data,open(save_pull_file ,'w'))



		#获取特定的 pull - commit
		commits = pull.get_commits()
		pull_to_commit = {}
		sha_message = []
		for commit in commits:
		    sha_message.append(commit.sha)
		
		pull_to_commit[count] = sha_message
		print(pull_to_commit)

		save_pull_to_commit_file = save_pull_to_commit_dir +"/"+ str(issueID)+".json"

		json.dump(pull_to_commit,open(save_pull_to_commit_file ,'w'))



	count = count + 1
	print(g.rate_limiting,g.get_rate_limit())
















