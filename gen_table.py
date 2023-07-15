import os
import json
from collections import defaultdict
import csv


def gen_issue_table(issue_dir,save_dir,project_name):
	
	attr_counts = defaultdict(int)
	save_data = []

	save_data.append(("project_name", "issue_number", "state_reason","state","title", "created_at","updated_at","closed_at","assignee","comments",\
		"milestone_title","milestone_open_issues","milestone_closed_issues","milestone_state","milestone_created_at","milestone_updated_at",\
		"milestone_due_on","milestone_closed_at","label_str","lable_len")) 


	for root,dirs,files in os.walk(issue_dir):
		for file in files:
			if file.endswith(".json"):
				issue_data = []

				issue_path = root+"/"+file
				json_data = json.loads(open(issue_path,'r').read() )

				# for key in json_data.keys():
					# testset.add(key)

				project_name = issue_path.split("/")[-2]
				issue_data.append(project_name)

				number = json_data['number'] # issue id
				issue_data.append(number)

				# print(project_name,issue_number)

				state = json_data['state']  # completed, not-planned
				issue_data.append(state)

				state_reason = json_data['state_reason']  # completed, not-planned
				issue_data.append(state_reason)


				title = json_data['title'] 
				issue_data.append(title)



				created_at= json_data['created_at'] 
				issue_data.append(created_at)

				
				updated_at= json_data['updated_at']
				issue_data.append(updated_at) 
				
				closed_at= json_data['closed_at'] 
				issue_data.append(closed_at)

				assignees = json_data['assignees']
				assignee = len(assignees)
				issue_data.append(assignee)


				comments= json_data['comments'] 
				issue_data.append(comments)

				milestone= json_data['milestone'] 
				# issue_data.append(milestone)

				# print(milestone)
				if milestone == None:
					milestone_title = 0
					milestone_open_issues = 0
					milestone_closed_issues = 0
					milestone_state = 0
					milestone_created_at = 0
					milestone_updated_at = 0
					milestone_due_on = 0
					milestone_closed_at = 0
				else:
					milestone_title = milestone['title']
					milestone_open_issues = milestone['open_issues']
					milestone_closed_issues = milestone['closed_issues']
					milestone_state = milestone['state']
					milestone_created_at = milestone['created_at']
					milestone_updated_at = milestone['updated_at']
					milestone_due_on = milestone['due_on']
					milestone_closed_at = milestone['closed_at']

				issue_data.append(milestone_title)
				issue_data.append(milestone_open_issues)
				issue_data.append(milestone_closed_issues)
				issue_data.append(milestone_state)
				issue_data.append(milestone_created_at)
				issue_data.append(milestone_updated_at )
				issue_data.append(milestone_due_on)
				issue_data.append(milestone_closed_at)


				label_name=[]
				label_str =""
				labels= json_data['labels']
				if labels != None:
					for label in labels:
						label_name.append(label["name"])
						label_str = label_str + "|||" + label["name"]
				label_len = len(label_name)


				issue_data.append(label_str)
				issue_data.append(label_len)

				# print(issue_data)
				save_data.append(issue_data)


	# print(save_data)

	if not os.path.exists(save_dir+"/issue/"):
		os.mkdir(save_dir+"/issue/")



	save_file = save_dir + "/issue/" +project_name+".csv"
	writer = csv.writer(open(save_file,'w'))
	writer.writerows(save_data)



def gen_pull_table(pull_dir,save_dir,project_name):
	
	attr_counts = defaultdict(int)
	save_data = []

	save_data.append(("project_name", "pull_number", "state","title", "created_at","updated_at","closed_at","assignee","comments",\
		"milestone_title","milestone_open_issues","milestone_closed_issues","milestone_state","milestone_created_at","milestone_updated_at",\
		"milestone_due_on","milestone_closed_at","label_str","lable_len",'merged', "mergeable", "mergeable_state", "merged_at",\
		 "commits","review_comments","rebaseable","draft","merge_commit_sha","additions", "deletions", "changed_files", "author_association")) 


	for root,dirs,files in os.walk(pull_dir):
		for file in files:
			if file.endswith(".json"):
				pull_data = []

				pull_path = root+"/"+file
				json_data = json.loads(open(pull_path,'r').read() )

				# print(json_data.keys())
				# print(json_data['active_lock_reason'],'\n')
				# # print(json_data["files"],"\n")

				project_name = pull_path.split("/")[-2]
				pull_data.append(project_name)

				number = json_data['number'] # pull id
				pull_data.append(number)

				# print(project_name,pull_number)

				state = json_data['state']  # completed, not-planned
				pull_data.append(state)

				# state_reason = json_data['state_reason']  # completed, not-planned
				# pull_data.append(state_reason)


				title = json_data['title'] 
				pull_data.append(title)



				created_at= json_data['created_at'] 
				pull_data.append(created_at)

				
				updated_at= json_data['updated_at']
				pull_data.append(updated_at) 
				
				closed_at= json_data['closed_at'] 
				pull_data.append(closed_at)

				assignees = json_data['assignees']
				assignee = len(assignees)
				pull_data.append(assignee)


				comments= json_data['comments'] 
				pull_data.append(comments)

				milestone= json_data['milestone'] 
				# pull_data.append(milestone)

				# print(milestone)
				if milestone == None:
					milestone_title = 0
					milestone_open_issues = 0
					milestone_closed_issues = 0
					milestone_state = 0
					milestone_created_at = 0
					milestone_updated_at = 0
					milestone_due_on = 0
					milestone_closed_at = 0
				else:
					milestone_title = milestone['title']
					milestone_open_issues = milestone['open_issues']
					milestone_closed_issues = milestone['closed_issues']
					milestone_state = milestone['state']
					milestone_created_at = milestone['created_at']
					milestone_updated_at = milestone['updated_at']
					milestone_due_on = milestone['due_on']
					milestone_closed_at = milestone['closed_at']

				pull_data.append(milestone_title)
				pull_data.append(milestone_open_issues)
				pull_data.append(milestone_closed_issues)
				pull_data.append(milestone_state)
				pull_data.append(milestone_created_at)
				pull_data.append(milestone_updated_at )
				pull_data.append(milestone_due_on)
				pull_data.append(milestone_closed_at)


				label_name=[]
				label_str =""
				labels= json_data['labels']
				if labels != None:
					for label in labels:
						label_name.append(label["name"])
						label_str = label_str + "|||" + label["name"]
				label_len = len(label_name)


				pull_data.append(label_str)
				pull_data.append(label_len)


				merged= json_data['merged']
				pull_data.append(merged)

				mergeable = json_data['mergeable']
				pull_data.append(mergeable)

				mergeable_state = json_data['mergeable_state']
				pull_data.append(mergeable_state)

				merged_at = json_data['merged_at']
				pull_data.append(merged_at)

				commits = json_data['commits']
				pull_data.append(commits)

				review_comments = json_data['review_comments']
				pull_data.append(review_comments)

				rebaseable = json_data['rebaseable']
				pull_data.append(rebaseable)

				draft = json_data['draft']
				pull_data.append(draft)

				merge_commit_sha = json_data['merge_commit_sha']
				pull_data.append(merge_commit_sha)
			
				additions = json_data['additions']
				pull_data.append(additions)
			
				deletions = json_data['deletions']
				pull_data.append(deletions)

				changed_files = json_data['changed_files']
				pull_data.append(changed_files)
				
				author_association = json_data['author_association']
				pull_data.append(author_association)

				# print(pull_data)
				save_data.append(pull_data)


	# print(save_data)
	if not os.path.exists(save_dir+"/pull/"):
		os.mkdir(save_dir+"/pull/")

	save_file = save_dir + "/pull/" +project_name+".csv"
	writer = csv.writer(open(save_file,'w'))
	writer.writerows(save_data)



def gen_commit_table(commit_dir,save_dir,project_name):
	
	attr_counts = defaultdict(int)
	save_data = []

	save_data.append(("project_name", "sha", "commit_verification_message","commit_verification_comment_count","commit_verification_verified", "commit_verification_reason","commit_verification_signature",\
		"parents_sha","stats_total","stats_additions","stats_deletions","modified_file_sha","file_number")) 


	for root,dirs,files in os.walk(commit_dir):
		for file in files:
			if file.endswith(".json"):
				commit_data = []

				commit_path = root+"/"+file
				json_data = json.loads(open(commit_path,'r').read() )

				project_name = commit_path.split("/")[-2]
				commit_data.append(project_name)

				sha = json_data['sha'] # commit id
				commit_data.append(sha)

				# print(json_data.keys())
				# print(json_data['sha'],'\n')
				# print(json_data["files"],"\n")

				commit_verification_message = json_data["commit"]["message"]
				commit_data.append(commit_verification_message)

				commit_verification_comment_count = json_data["commit"]["comment_count"]
				commit_data.append(commit_verification_comment_count)

				commit_verification_verified = json_data["commit"]["verification"]["verified"]
				commit_data.append(commit_verification_verified)

				commit_verification_reason = json_data["commit"]["verification"]["reason"]
				commit_data.append(commit_verification_reason)

				commit_verification_signature = json_data["commit"]["verification"]["signature"]
				commit_data.append(commit_verification_signature)


				parents_sha = ""
				if json_data["parents"]:
					for item in json_data["parents"]:
						parents_sha = parents_sha + "|||"+ item['sha'] 
				# print(parents_sha)
				commit_data.append(parents_sha)


				stats_total = json_data["stats"]["total"]
				commit_data.append(stats_total)

				stats_additions = json_data["stats"]["additions"]
				commit_data.append(stats_additions)

				stats_deletions = json_data["stats"]["deletions"]
				commit_data.append(stats_deletions)


				modified_file_sha = ""
				if json_data["files"]:
					for item in json_data["files"]:
						modified_file_sha = modified_file_sha + "|||"+ str(item['sha']) 
				commit_data.append(modified_file_sha)

				file_number = len(json_data["files"])
				commit_data.append(file_number)


				save_data.append(commit_data)


	# print(save_data)

	if not os.path.exists(save_dir+"/commit/"):
		os.mkdir(save_dir+"/commit/")



	save_file = save_dir + "/commit/" +project_name+".csv"
	writer = csv.writer(open(save_file,'w'))
	writer.writerows(save_data)




def gen_commit_file_table(commit_dir,save_dir,project_name):
	
	attr_counts = defaultdict(int)
	save_data = []

	save_data.append(("project_name", "file_name", "file_sha","commit_sha","file_status", "file_additions","file_deletions","file_changes")) 


	for root,dirs,files in os.walk(commit_dir):
		for file in files:
			if file.endswith(".json"):

				# print(json_data.keys())
				# print(json_data['sha'],'\n')
				# print(json_data["files"],"\n")

				commit_path = root+"/"+file
				json_data = json.loads(open(commit_path,'r').read() )

				project_name = commit_path.split("/")[-2]
		
				commit_sha = json_data['sha'] # commit id



				for file_data in json_data["files"]:
					# print(file_data.keys())
					commit_file_data = []

					commit_file_data.append(project_name)
					file_name = file_data['filename']
					commit_file_data.append(file_name)

					file_sha = file_data['sha']
					commit_file_data.append(file_sha )

					commit_file_data.append(commit_sha)

					file_status = file_data['status']
					commit_file_data.append(file_status)

					file_additions = file_data['additions']
					commit_file_data.append(file_additions)

					file_deletions = file_data['deletions']
					commit_file_data.append(file_deletions)

					file_changes = file_data['changes']
					commit_file_data.append(file_changes)


					save_data.append(commit_file_data)


	# print(save_data)

	if not os.path.exists(save_dir+"/commit_file/"):
		os.mkdir(save_dir+"/commit_file/")



	save_file = save_dir + "/commit_file/" +project_name+".csv"
	writer = csv.writer(open(save_file,'w'))
	writer.writerows(save_data)






def gen_pull_to_commit_table(pull_to_commit_dir,save_dir,project_name):
	
	attr_counts = defaultdict(int)
	save_data = []

	save_data.append(("pull_number", "commit_sha")) 


	for root,dirs,files in os.walk(pull_to_commit_dir):
		for file in files:
			if file.endswith(".json"):
				pull_to_commit_data = []

				pull_to_commit_path = root+"/"+file
				json_data = json.loads(open(pull_to_commit_path,'r').read() )

				for key in json_data:
					# print(key,json_data[key])
					commits = ""
					if json_data[key]:
						for commit in json_data[key]:
							commits = commits + "|||"+ commit



					pull_to_commit_data.append(key)
					pull_to_commit_data.append(commits)

				# print(pull_to_commit_data)
				save_data.append(pull_to_commit_data)


	# print(save_data)

	if not os.path.exists(save_dir+"/pull_to_commit/"):
		os.mkdir(save_dir+"/pull_to_commit/")

	save_file = save_dir + "/pull_to_commit/" +project_name+".csv"
	writer = csv.writer(open(save_file,'w'))
	writer.writerows(save_data)



def gen_pull_to_issue_table(pull_to_issue_dir,save_dir,project_name):
	
	attr_counts = defaultdict(int)
	save_data = []

	save_data.append(("pull_number", "issue_number")) 


	for root,dirs,files in os.walk(pull_to_issue_dir):
		for file in files:
			if file.endswith(".json"):
				pull_to_issue_data = []

				pull_to_issue_path = root+"/"+file
				json_data = json.loads(open(pull_to_issue_path,'r').read() )

				for key in json_data:
					# print(key,json_data[key])
					commits = ""
					if json_data[key]:
						for commit in json_data[key]:
							commits = commits + "|||"+ commit



					pull_to_issue_data.append(key)
					pull_to_issue_data.append(commits)

				# print(pull_to_issue_data)
				save_data.append(pull_to_issue_data)


	# print(save_data)

	if not os.path.exists(save_dir+"/pull_to_issue/"):
		os.mkdir(save_dir+"/pull_to_issue/")

	save_file = save_dir + "/pull_to_issue/" +project_name+".csv"
	writer = csv.writer(open(save_file,'w'))
	writer.writerows(save_data)






def gen_test_case_table(test_case_dir,save_dir,project_name):
	
	attr_counts = defaultdict(int)
	save_data = []

	save_data.append(("issue_number", "message_number", "The i_th code", "code")) 


	for root,dirs,files in os.walk(test_case_dir):
		for file in files:
			if file.endswith(".rs"):
				test_case_data = []


				info = file.replace(".rs","").split("_")

				issue_number = info[0]
				message_number  = info[1]

				mid = ""
				if len(info) > 2:
					mid = info[2]


				test_case_path = root+"/"+file
				code = open(test_case_path,'r').read()


				test_case_data.append(issue_number)
				test_case_data.append(message_number)
				test_case_data.append(mid)
				test_case_data.append(code)				

				# print(test_case_data)
				save_data.append(test_case_data)


	# print(save_data)

	if not os.path.exists(save_dir+"/test_case/"):
		os.mkdir(save_dir+"/test_case/")

	save_file = save_dir + "/test_case/" +project_name+".csv"
	writer = csv.writer(open(save_file,'w'))
	writer.writerows(save_data)




# reponame = "Rust-GCC/gccrs"
reponame = "rust-lang_rust"

project = reponame.replace("/","_")

current_path = os.getcwd()
print("Current path:",current_path)

save_dir = os.getcwd() + "/table"
print("Save directory:",save_dir)


issue_dir = current_path + "/issue/"+ project
pull_dir = current_path + "/pull/"+ project
commit_dir = current_path + "/commit/"+ project
pull_to_issue_dir = current_path + "/pull_to_issue/"+ project
pull_to_commit_dir = current_path + "/pull_to_commit/"+ project
test_case_dir = current_path + "/test_case/"+ project
project_dir = current_path + "/project/"+ project


# print(issue_dir,pull_dir,commit_dir,pull_to_issue_dir, pull_to_commit_dir, test_case_dir, project_dir)

print("gen_issue_table...")
gen_issue_table(issue_dir,save_dir,project)
print("gen_pull_table...")
gen_pull_table(pull_dir,save_dir,project)
print("gen_pull_to_commit_table...")
gen_pull_to_commit_table(pull_to_commit_dir,save_dir,project)
print("gen_pull_to_issue_table...")
gen_pull_to_issue_table(pull_to_issue_dir,save_dir,project)
print("gen_test_case_table...")
gen_test_case_table(test_case_dir,save_dir,project)
print("gen_commit_table...")
gen_commit_table(commit_dir,save_dir,project)
print("gen_commit_file_table...")
gen_commit_file_table(commit_dir,save_dir,project)
print("All finished...")