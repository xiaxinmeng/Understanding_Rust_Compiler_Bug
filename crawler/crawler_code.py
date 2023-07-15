from github import Github
import random
import sys
import os
import json
# import urllib.request
# import ssl
import requests


reponame = "rust-lang/rust"
savepath = '/home/xxm/Desktop/cross-testing/rust' 


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


def getCode(bodylist):
    clist = []
    if len(bodylist) > 1:
        i = 0
        for item in bodylist:
            if i%2 != 0:
                clist.append(item)
            i = i + 1
    return clist





count = 95195
maxissue = 96000
while count <= maxissue:
	print(count)
	issue = repo.get_issue(number = count)
	print(issue)
	issueCommentUrl = issue.comments_url
	# f = requests.get(issueCommentUrl, params= {}, headers = {'User-Agent', use_agent[random.randint(0,25)]})
	# f = urllib.request.urlopen(issueCommentUrl)

	
	headers = {'User-Agent': use_agent[random.randint(0,25)],'Authorization': 'token %s'%authenticity_token}
	cookies = {'user_session': 'Ep7pf6npoofRzwWjUIFt7u8plrmYjFVPoOVy0-j8e9KtkvCM'}
	# auth = ('xiaxinmeng1993@hotmail.com','xxm930923')
	# data = {
	# # 'commit':'Sign in',
	# 'authenticity_token':authenticity_token,
	# 'login':'xiaxinmeng1993@hotmail.com',
	# 'password':'xxm930923'
	# }

	response = requests.get(issueCommentUrl,headers = headers, cookies=cookies)
	comments = response.json()
	# print(comments)
	issueID = count

	for comment in comments:
		body = comment['body']
		ID = comment['id']

		bodylist=body.split('```')
		codelist = getCode(bodylist)
		if codelist:
			print("======================")
			i = 0
			for item in codelist:
				print(item)
				print("======================")

				savedir = savepath+ '/'+reponame.replace("/",'_')
				if not os.path.exists(savedir):
					os.makedirs(savedir)
				if i == 0:
					savefile = savedir +"/"+ str(issueID)+"_"+str(ID)+".rs"
				else:
					savefile = savedir +"/"+ str(issueID)+"_"+str(ID)+"_"+ str(i)+ ".rs"
				f= open(savefile,'w')
				f.write(item)
				i = i + 1


	count = count + 1
	
print(g.rate_limiting,g.get_rate_limit())













