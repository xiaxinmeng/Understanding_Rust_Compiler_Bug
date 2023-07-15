import requests
from bs4 import BeautifulSoup
import re
import os
import json
import random


repo = "rust-lang/rust"

pull_dir = "/home/xxm/Desktop/empirical_bug_rust_compiler/pull/" + repo.replace("/","_")

pull_to_issue_dir= '/home/xxm/Desktop/empirical_bug_rust_compiler/pull_to_issue/'+ repo.replace("/","_")
if not os.path.exists(pull_to_issue_dir):
	os.makedirs(pull_to_issue_dir)





pull_list = []
for root, dirs,files in os.walk(pull_dir):
	for file in files:
		# print(file)
		if file.endswith(".json"):
			# print(file)
			pull_list.append(int(file.split(".")[0]))
			# print("prid", prid)
pull_list.sort()


# print(pull_list)


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
      



count = 1
maxissue = 640000
while count <= maxissue:

	pull_number = str(pull_list[count-1])
	print(count,pull_number)
	


	headers = {'User-Agent': use_agent[random.randint(0,25)],'Authorization': 'token %s'%authenticity_token}
	cookies = {'user_session': 'Ep7pf6npoofRzwWjUIFt7u8plrmYjFVPoOVy0-j8e9KtkvCM'}

	# r = requests.get(f"https://github.com/{repo}/pull/{pull_number}",headers = headers, cookies=cookies)

	r = requests.get(f"https://github.com/{repo}/pull/{pull_number}")
	soup = BeautifulSoup(r.text, 'html.parser')
	issueForm = soup.find("form", { "aria-label": re.compile('Link issues')})


	pull_to_issue = {}
	issue_links = []
	for i in issueForm.find_all("a"):
		issue_links.append(i["href"].split('/')[-1])

	pull_to_issue[pull_number] = issue_links

	

	save_pull_to_issue_file = pull_to_issue_dir + "/"+ pull_number+".json" 
	json.dump(pull_to_issue, open(save_pull_to_issue_file, "w"))


	count =count + 1









