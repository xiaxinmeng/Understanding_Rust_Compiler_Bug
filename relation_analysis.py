import csv
import os
from scipy.stats import pearsonr,spearmanr
from datetime import datetime
import sys
import re
from collections import Counter
from scipy.stats import fisher_exact
from scipy.stats import kendalltau


csv.field_size_limit(sys.maxsize)


def read_from_csv(path):
	csv_reader = csv.reader(open(path,'r'))

	data = []
	for row in csv_reader:
		data.append(row)
	return(data)



def calculate_time_difference(time1, time2):
    """
    计算两个时间之间的时间差
    
    参数：
    time1 (str): 第一个时间，格式为'YYYY-MM-DDTHH:MM:SSZ'
    time2 (str): 第二个时间，格式为'YYYY-MM-DDTHH:MM:SSZ'
    
    返回值：
    timedelta: 时间差，表示为datetime.timedelta对象
    """
    dt_format = '%Y-%m-%dT%H:%M:%SZ'
    
    # 解析时间字符串为datetime对象
    t1 = datetime.strptime(time1, dt_format)
    t2 = datetime.strptime(time2, dt_format)
    
    # 计算时间差
    time_difference = t2 - t1
    hours = time_difference.total_seconds() / 3600
    
    return hours


# def calculate_time_difference(start_time_str, end_time_str):
#     start_datetime = datetime.fromisoformat(start_time_str)
#     end_datetime = datetime.fromisoformat(end_time_str)

#     time_difference = start_datetime - end_datetime

#     return time_difference


#issue_id -> pull_id
def get_issue_to_pull_dic(pull_to_issues):
	issue_to_pull_dic = {}
	for pull_to_issue in pull_to_issues[1:]:
		pull = pull_to_issue[0]
		issues = pull_to_issue[1]
		if issues:
			issue_list = issues.split("|||")[1:]
			for issue in issue_list:
				if issue not in issue_to_pull_dic.keys():
					issue_to_pull_dic[issue] = set()

				issue_to_pull_dic[issue].add(pull)

				# print(pull, issue)
				# print(issue_to_pull_dic[issue])
	return issue_to_pull_dic



# pull_id -> commit_sha
def get_pull_to_commit_dic(pull_to_commits):
	pull_to_commit_dic={}
	for pull_to_commit in pull_to_commits[1:]:
		commits = pull_to_commit[1]
		pull = pull_to_commit[0]
		commit_set = set()
		for commit in commits.split("|||")[1:]:
			commit_set.add(commit)
		pull_to_commit_dic[pull] = commit_set
	return pull_to_commit_dic



# commit_sha -> file_sha
def get_commit_to_file_dic(commit_files):
	commit_to_file_dic = {}
	for commit_file in commit_files[1:]:
		commit_sha = commit_file[3]
		file_sha = commit_file[2]
		if commit_sha not in commit_to_file_dic.keys():
			commit_to_file_dic[commit_sha] = set()
		commit_to_file_dic[commit_sha].add(file_sha)
	return commit_to_file_dic


# commit_sha -> ['project_name',  'commit_verification_message', 'commit_verification_comment_count', 'commit_verification_verified', \
# 'commit_verification_reason', 'commit_verification_signature', 'parents_sha', 'stats_total', 'stats_additions', 'stats_deletions', 'modified_file_sha', 'file_number']
def get_commit_dic(commits):
	commit_dic = {}
	for commit in commits[1:]:
		commit_sha = commit[1]
		new_list = commit[:1] + commit[2:]
		commit_dic[commit_sha] = new_list
	return commit_dic



# 'pull_number': ['project_name', 'state', 'title', 'created_at', 'updated_at', 'closed_at', 'assignee', 'comments', 'milestone_title', 'milestone_open_issues',\
 # 'milestone_closed_issues', 'milestone_state', 'milestone_created_at', 'milestone_updated_at', 'milestone_due_on', 'milestone_closed_at', 'label_str', 'lable_len', \
 # 'merged', 'mergeable', 'mergeable_state', 'merged_at', 'commits', 'review_comments', 'rebaseable', 'draft', 'merge_commit_sha', 'additions', 'deletions', 'changed_files', 'author_association'
def get_pull_dic(pulls):
	pull_dic = {}
	for pull in pulls[1:]:
		pull_number = pull[1]
		new_list = pull[:1] + pull[2:]
		pull_dic[pull_number] = new_list
	return pull_dic



# issue_number -> ['project_name',  'state_reason', 'state', 'title', 'created_at', 'updated_at',\
# 'closed_at', 'assignee', 'comments', 'milestone_title','milestone_open_issues', 'milestone_closed_issues', \
# 'milestone_state', 'milestone_created_at', 'milestone_updated_at', 'milestone_due_on', 'milestone_closed_at', 'label_str', 'lable_len']
def get_issue_dic(issues):
	issue_dic = {}
	for issue in issues[1:]:
		issue_number = issue[1]
		new_list = issue[:1]+issue[2:]
		issue_dic[issue_number] = new_list
	return issue_dic


#commit_sha -> ['project_name', 'file_name', 'commit_sha', 'file_status', 'file_additions', 'file_deletions', 'file_changes']
def get_commit_file_dic(commit_file):
	commit_file_dic = {}

	commit_file_dic = {}
	for commit_file in commit_file[1:]:
		commit_file_number = commit_file[2]
		new_list = commit_file[:2]+commit_file[3:]
		commit_file_dic[commit_file_number] = new_list
	return commit_file_dic












# reponame = "rust-lang/rust"
reponame = "Rust-GCC/gccrs"



project = reponame.replace("/","_")



current_dir = os.getcwd()
print("current_dir:",current_dir)
table_dir = current_dir + "/table"
project_path = current_dir +"/project/" + project 



commit_path = table_dir + "/commit/" + project+ ".csv" 
commits = read_from_csv(commit_path)
# ['project_name', 'sha', 'commit_verification_message', 'commit_verification_comment_count', 'commit_verification_verified', \
# 'commit_verification_reason', 'commit_verification_signature', 'parents_sha', 'stats_total', 'stats_additions', 'stats_deletions', 'modified_file_sha', 'file_number']



commit_file_path = table_dir + "/commit_file/" + project+ ".csv" 
print(commit_file_path)
commit_files = read_from_csv(commit_file_path)
# ['project_name', 'file_name', 'file_sha', 'commit_sha', 'file_status', 'file_additions', 'file_deletions', 'file_changes']


issue_path = table_dir + "/issue/" + project+ ".csv" 
issues = read_from_csv(issue_path)
# ['project_name', 'issue_number', 'state_reason', 'state', 'title', 'created_at', 'updated_at',\
# 'closed_at', 'assignee', 'comments', 'milestone_title','milestone_open_issues', 'milestone_closed_issues', \
# 'milestone_state', 'milestone_created_at', 'milestone_updated_at', 'milestone_due_on', 'milestone_closed_at', 'label_str', 'lable_len']



pull_path = table_dir + "/pull/" + project+ ".csv" 
pulls = read_from_csv(pull_path)
# ['project_name', 'pull_number', 'state', 'title', 'created_at', 'updated_at', 'closed_at', 'assignee', 'comments', 'milestone_title', 'milestone_open_issues',\
 # 'milestone_closed_issues', 'milestone_state', 'milestone_created_at', 'milestone_updated_at', 'milestone_due_on', 'milestone_closed_at', 'label_str', 'lable_len', \
 # 'merged', 'mergeable', 'mergeable_state', 'merged_at', 'commits', 'review_comments', 'rebaseable', 'draft', 'merge_commit_sha', 'additions', 'deletions', 'changed_files', 'author_association'



test_case_path = table_dir + "/test_case/" + project+ ".csv" 
test_cases = read_from_csv(test_case_path)
# ['issue_number', 'message_number', 'The i_th code', 'code']


pull_to_commit_path = table_dir + "/pull_to_commit/" + project+ ".csv" 
pull_to_commits = read_from_csv(pull_to_commit_path)
# ['pull_number', 'commit_sha']


pull_to_issue_path = table_dir + "/pull_to_issue/" + project+ ".csv" 
pull_to_issues = read_from_csv(pull_to_issue_path)
# ['pull_number', 'issue_number']






issue_to_pull_dic = get_issue_to_pull_dic(pull_to_issues)
pull_to_commit_dic = get_pull_to_commit_dic(pull_to_commits)
commit_to_file_dic = get_commit_to_file_dic(commit_files)
commit_dic = get_commit_dic(commits)
pull_dic = get_commit_dic(pulls)
issue_dic = get_commit_dic(issues)
commit_file_dic = get_commit_file_dic(commit_files)
# print(pull_dic)


# ===========================================================================================================================


def find_top_10_key_value_pairs(dictionary):
    """
    找到字典中值最大的前10个键值对
    
    参数：
    dictionary (dict): 包含键值对的字典
    
    返回值：
    list: 值最大的前10个键值对，每个键值对表示为元组 (key, value)
    """
    sorted_items = sorted(dictionary.items(), key=lambda x: x[1], reverse=True)  # 根据值进行降序排序
    top_10_key_value_pairs = sorted_items[:15]  # 获取前10个键值对
    return top_10_key_value_pairs

# # 测试函数
# my_dict = {'a': 10, 'b': 5, 'c': 15, 'd': 20, 'e': 12, 'f': 8, 'g': 18, 'h': 3, 'i': 7, 'j': 9, 'k': 11}
# top_10_keys = find_top_10_keys(my_dict)
# print('值最大的前10个键:', top_10_keys)


def get_issue_modi_file(issue_to_pull_dic, pull_to_commit_dic, commit_to_file_dic,commit_file_dic):
	file_to_issue = {}
	for issue in issue_to_pull_dic.keys():
		pulls = issue_to_pull_dic[issue]
		# print(pulls)
		for pull in pulls:
			if pull in pull_to_commit_dic.keys():
				commits = pull_to_commit_dic[pull]
				for commit in commits:
					# print(commit)
					if commit in commit_to_file_dic:
						file_shas = commit_to_file_dic[commit]
						# print(file_shas)
						for file_sha in file_shas:
							# print(file_sha)
							# print(commit_file_dic)
							if file_sha in commit_file_dic.keys():
								file = commit_file_dic[file_sha][1]
								# for file in files:
								if not file in file_to_issue.keys():
									file_to_issue[file] = set()
								file_to_issue[file].add(issue)

	file_to_num_issue = {}
	for file in file_to_issue:
		file_to_num_issue[file] = len(file_to_issue[file])
	top_10_modi_file_dic = find_top_10_key_value_pairs(file_to_num_issue)
	print(top_10_modi_file_dic)
	for item in top_10_modi_file_dic:
		file = item[0]
		num = item[1]
		print(file,num)
		# print(file)


def get_top10_bug_module(issue_to_pull_dic, pull_to_commit_dic, commit_to_file_dic,commit_file_dic):
	file_to_issue = {}
	for issue in issue_to_pull_dic.keys():
		pulls = issue_to_pull_dic[issue]
		# print(pulls)
		for pull in pulls:
			if pull in pull_to_commit_dic.keys():
				commits = pull_to_commit_dic[pull]
				for commit in commits:
					# print(commit)
					if commit in commit_to_file_dic:
						file_shas = commit_to_file_dic[commit]
						# print(file_shas)
						for file_sha in file_shas:
							# print(file_sha)
							# print(commit_file_dic)
							if file_sha in commit_file_dic.keys():
								file = commit_file_dic[file_sha][1]
								# for file in files:
								if len(file.split("/")) > 1:
									module = file.split("/")[0]+'/'+file.split("/")[1]
								else:
									module = file.split("/")[0]
								file = module

								if not file in file_to_issue.keys():
									file_to_issue[file] = set()
								file_to_issue[file].add(issue)

	file_to_num_issue = {}
	for file in file_to_issue:
		file_to_num_issue[file] = len(file_to_issue[file])
	top_10_modi_file_dic = find_top_10_key_value_pairs(file_to_num_issue)
	print(top_10_modi_file_dic)
	for item in top_10_modi_file_dic:
		file = item[0]
		num = item[1]
		# print(num)
		print(file,num)










# =========================================================================================



def get_issue_duration_and_issue_comment(issue_dic):

	issue_duration = []
	number_of_issue_comment =[]
	for issue in issue_dic.keys():
		issue_created_time = issue_dic[issue][4]
		issue_closed_time = issue_dic[issue][6]
		if issue_closed_time:
			issue_duration_time = calculate_time_difference(issue_created_time, issue_closed_time)
			number_of_issue_comment.append(int(issue_dic[issue][8]))
			issue_duration.append(issue_duration_time)
	return issue_duration, number_of_issue_comment




def get_issue_duration_and_pull_comment(issue_dic,issue_to_pull_dic,pull_dic):

	issue_duration = []
	number_of_pull_comment =[]
	for issue in issue_dic.keys():
		issue_created_time = issue_dic[issue][4]
		issue_closed_time = issue_dic[issue][6]
		if issue_closed_time:
			issue_duration_time = calculate_time_difference(issue_created_time, issue_closed_time)
			if issue in issue_to_pull_dic.keys():
				pull_set = issue_to_pull_dic[issue]
				# print(pull_set)
				for pull in pull_set:
					number_of_pull_comment.append(int(pull_dic[pull][7]))
					issue_duration.append(issue_duration_time)
	return issue_duration, number_of_pull_comment


def get_issue_duration_and_change_files(issue_dic,issue_to_pull_dic,pull_dic):

	issue_duration = []
	change_files =[]
	for issue in issue_dic.keys():
		issue_created_time = issue_dic[issue][4]
		issue_closed_time = issue_dic[issue][6]
		if issue_closed_time:
			issue_duration_time = calculate_time_difference(issue_created_time, issue_closed_time)
			if issue in issue_to_pull_dic.keys():
				pull_set = issue_to_pull_dic[issue]
				# print(pull_set)
				for pull in pull_set:
					change_files.append(int(pull_dic[pull][-2]))
					issue_duration.append(issue_duration_time)
	return issue_duration, change_files



def get_issue_duration_and_pull_duration(issue_dic,issue_to_pull_dic,pull_dic):

	issue_duration = []
	pull_duration =[]
	for issue in issue_dic.keys():
		issue_created_time = issue_dic[issue][4]
		issue_closed_time = issue_dic[issue][6]
		if issue_closed_time:
			issue_duration_time = calculate_time_difference(issue_created_time, issue_closed_time)
			if issue in issue_to_pull_dic.keys():
				pull_set = issue_to_pull_dic[issue]
				# print(pull_set)
				for pull in pull_set:
					pull_created_time = pull_dic[pull][3]
					pull_closed_time = pull_dic[pull][5]
					if pull_closed_time:
						# print(pull_closed_time)
						pull_duration_time = calculate_time_difference(pull_created_time, pull_closed_time)
						pull_duration.append(pull_duration_time)

					# change_files.append(int(pull_dic[pull][-2]))
						issue_duration.append(issue_duration_time)
	return issue_duration,pull_duration


def get_issue_duration_and_pull_comments(issue_dic,issue_to_pull_dic,pull_dic):

	issue_duration = []
	pull_comments =[]
	for issue in issue_dic.keys():
		issue_created_time = issue_dic[issue][4]
		issue_closed_time = issue_dic[issue][6]
		if issue_closed_time:
			issue_duration_time = calculate_time_difference(issue_created_time, issue_closed_time)
			if issue in issue_to_pull_dic.keys():
				pull_set = issue_to_pull_dic[issue]
				# print(pull_set)
				for pull in pull_set:
					pull_comments.append(int(pull_dic[pull][7]))
					issue_duration.append(issue_duration_time)
	return issue_duration, pull_comments 




def get_issue_duration_and_pull_review_comments(issue_dic,issue_to_pull_dic,pull_dic):

	issue_duration = []
	pull_review_comments =[]
	for issue in issue_dic.keys():
		issue_created_time = issue_dic[issue][4]
		issue_closed_time = issue_dic[issue][6]
		if issue_closed_time:
			issue_duration_time = calculate_time_difference(issue_created_time, issue_closed_time)
			if issue in issue_to_pull_dic.keys():
				pull_set = issue_to_pull_dic[issue]
				# print(pull_set)
				for pull in pull_set:
					pull_review_comments.append(int(pull_dic[pull][-8]))
					issue_duration.append(issue_duration_time)
	return issue_duration, pull_review_comments 




def get_issue_duration_and_pull_commits(issue_dic,issue_to_pull_dic,pull_dic):

	issue_duration = []
	pull_commits =[]
	for issue in issue_dic.keys():
		issue_created_time = issue_dic[issue][4]
		issue_closed_time = issue_dic[issue][6]
		if issue_closed_time:
			issue_duration_time = calculate_time_difference(issue_created_time, issue_closed_time)
			if issue in issue_to_pull_dic.keys():
				pull_set = issue_to_pull_dic[issue]
				# print(pull_set)
				for pull in pull_set:
					pull_commits.append(int(pull_dic[pull][-9]))
					issue_duration.append(issue_duration_time)
	return issue_duration, pull_commits 




def get_issue_duration_and_modi_file(issue_to_pull_dic, pull_to_commit_dic, commit_to_file_dic,commit_file_dic):
	issue_duration =[]
	file_to_issue = []
	for issue in issue_dic.keys():
		issue_created_time = issue_dic[issue][4]
		issue_closed_time = issue_dic[issue][6]
		if issue_closed_time:
			issue_duration_time = calculate_time_difference(issue_created_time, issue_closed_time)
			if issue in issue_to_pull_dic.keys():
			# for issue in issue_to_pull_dic.keys():
				pull_set = issue_to_pull_dic[issue]
				# print(pulls)
				for pull in pull_set:
					if pull in pull_to_commit_dic.keys():
						commit_set = pull_to_commit_dic[pull]
						for commit in commit_set:
							if commit in commit_file_dic.keys():
								# print()
								file_changes = commit_file_dic[str(commit)]
								print(file_changes)
								file_to_issue.append(file_changes)
								issue_duration.append(issue_duration_time)


	return issue_duration, file_to_issue



def get_pull_duration_and_pull_change_files(pull_dic):

	change_files = []
	pull_duration =[]
	num_of_comment = []
	for pull in pull_dic:
		pull_created_time = pull_dic[pull][3]
		pull_closed_time = pull_dic[pull][5]
		if pull_closed_time:
			# print(pull_closed_time)
			pull_duration_time = calculate_time_difference(pull_created_time, pull_closed_time)
			pull_duration.append(pull_duration_time)
			change_files.append(float(pull_dic[pull][-2]))

	return pull_duration, change_files



def get_pull_duration_and_pull_comments(pull_dic):

	change_files = []
	pull_duration =[]
	num_of_comment = []
	for pull in pull_dic:
		pull_created_time = pull_dic[pull][3]
		pull_closed_time = pull_dic[pull][5]
		if pull_closed_time:
			# print(pull_closed_time)
			pull_duration_time = calculate_time_difference(pull_created_time, pull_closed_time)
			pull_duration.append(pull_duration_time)
			num_of_comment.append(int(pull_dic[pull][7]))

	return pull_duration, num_of_comment


	



# print(pull_duration)




# get_issue_modi_file(issue_to_pull_dic, pull_to_commit_dic, commit_to_file_dic,commit_file_dic)
# get_top10_bug_module(issue_to_pull_dic, pull_to_commit_dic, commit_to_file_dic,commit_file_dic)

# 
# X,Y = get_pull_duration_and_pull_change_files(pull_dic)

# X,Y = get_pull_duration_and_pull_comments(pull_dic)



# X,Y = get_issue_duration_and_change_files(issue_dic,issue_to_pull_dic,pull_dic)
# X,Y = get_issue_duration_and_pull_comment(issue_dic,issue_to_pull_dic,pull_dic)
# X,Y = get_issue_duration_and_pull_duration(issue_dic,issue_to_pull_dic,pull_dic)
# X,Y = get_issue_duration_and_pull_review_comments(issue_dic,issue_to_pull_dic,pull_dic)

# X,Y = get_issue_duration_and_pull_commits(issue_dic,issue_to_pull_dic,pull_dic)

# X,Y = get_issue_duration_and_modi_file(issue_to_pull_dic, pull_to_commit_dic, commit_to_file_dic,commit_file_dic)


# X,Y =get_issue_duration_and_issue_comment(issue_dic)


# print(pearsonr(X,Y))
# print(spearmanr(X,Y,nan_policy = 'raise',alternative = 'two-sided'))
# print(kendalltau(X, Y))












# ================================================================================================

def count_rust_features(code):
    feature_counts = Counter()

    # with open(file_path, 'r') as file:
    #     code = file.read()

    # Count ownership and borrowing
    feature_counts['Ownership'] = len(re.findall(r'&[^\s]+', code))

    # Count pattern matching
    feature_counts['Pattern'] = len(re.findall(r'\bmatch\b', code))

    # Count error handling
    feature_counts['Error'] = len(re.findall(r'\bResult\b', code))

    # Count concurrency (threads)
    feature_counts['Concurrency'] = len(re.findall(r'\bthread::', code))

    # Count macros
    feature_counts['Macros'] = len(re.findall(r'\bmacro_rules!\b', code))

    feature_counts['trait'] = len(re.findall(r"\btrait\s*<[^>]+>\s*", code))

    feature_counts['unsafe'] = len(re.findall(r'\bunsafe\b', code))

    return feature_counts

# Usage example
# rust_file_path = 'path/to/your/rust_file.rs'





def cal_odds(group1_data,group2_data):
# Prepare your data
# group1_data = [1, 1, 0, 0, 1]
# group2_data = [0, 1, 1, 0, 0]

# Create the contingency table
	a = sum(group1_data)
	b = len(group1_data) - a
	c = sum(group2_data)
	d = len(group2_data) - c
	odds_ratio, p_value = fisher_exact([[a, b], [c, d]])
	return odds_ratio, p_value




def rel_rust_feature_commit_verfied(test_cases,issue_to_pull_dic,pull_to_commit_dic,commit_dic):

	Ownership= []
	Pattern = []
	Error = []
	Concurrency = []
	Macros = []
	Trait =[]
	Unsafe = []
	commit_accept = []

	for test_case in test_cases[1:]:
		issue_id = test_case[0]
		code = test_case[-1]

		feature_counts = count_rust_features(code)

		if issue_id in issue_to_pull_dic.keys():
			for pull_id in issue_to_pull_dic[issue_id]:
				# for commit_sha in pull_to_commits[pull_id]:
				# print(pull_dic[pull_id][-13])
				merged = pull_dic[pull_id][-13]
				if pull_id in pull_to_commit_dic.keys():
					for commit_sha in pull_to_commit_dic[pull_id]:
						if commit_sha in commit_dic.keys():
							commit_verify = commit_dic[commit_sha][3]
							# print(commit_verify)
							if commit_verify == "True":
								commit_accept.append(1)
							else:
								commit_accept.append(0)

							for feature, count in feature_counts.items():
								if feature == 'Ownership':
									Ownership.append(count)
									# if count == 0:
									# 	Ownership.append(0)
									# else:
									# 	Ownership.append(1)
								if feature == 'Pattern':
									Pattern.append(count)
									# if count == 0:
									# 	Pattern.append(0)
									# else:
									# 	Pattern.append(1)
								if feature == 'Error':
									Error.append(count)
									# if count == 0:
									# 	Error.append(0)
									# else:
									# 	Error.append(1)
								if feature == 'Concurrency':
									Concurrency.append(count)
									# if count == 0:
									# 	Concurrency.append(0)
									# else:
									# 	Concurrency.append(1)
								if feature == 'Macros':
									Macros.append(count)
									# if count == 0:
									# 	Macros.append(0)
									# else:
									# 	Macros.append(1)
																	# 	Concurrency.append(1)
								if feature == 'trait':
									Trait.append(count)

								if feature == 'unsafe':
									Unsafe.append(count)
									# if count == 0:
									# 	Macros.append(0)
									# else:
									# 	Macros.append(1)

	return commit_accept,Ownership,Pattern,Error,Concurrency,Macros,Trait,Unsafe 




def rel_rust_feature_and_issue_duration(test_cases,issue_dic):

	Ownership= []
	Pattern = []
	Error = []
	Concurrency = []
	Macros = []
	issue_duration = []
	Trait =[]
	Unsafe =[]

	for test_case in test_cases[1:]:
		issue_id = test_case[0]
		code = test_case[-1]

		feature_counts = count_rust_features(code)
		if issue_id in issue_dic.keys():
			issue_created_time = issue_dic[issue_id][4]
			issue_closed_time = issue_dic[issue_id][6]
			if issue_closed_time:
				issue_duration_time = calculate_time_difference(issue_created_time, issue_closed_time)
				issue_duration.append(issue_duration_time)
				for feature, count in feature_counts.items():
					if feature == 'Ownership':
						Ownership.append(count)
					if feature == 'Pattern':
						Pattern.append(count)
					if feature == 'Error':
						Error.append(count)
					if feature == 'Concurrency':
						Concurrency.append(count)
					if feature == 'Macros':
						Macros.append(count)
					if feature == 'trait':
						Trait.append(count)
					if feature == 'unsafe':
						Unsafe.append(count)


	return issue_duration,Ownership,Pattern,Error,Concurrency,Macros,Trait,Unsafe


# Y,X1,X2,X3,X4,X5 = rel_rust_feature_commit_verfied(test_cases,issue_to_pull_dic,pull_to_commit_dic,commit_dic)


Y,X1,X2,X3,X4,X5,X6,X7 = rel_rust_feature_and_issue_duration(test_cases,issue_dic)

# print(rel_rust_feature_and_issue_duration(test_cases,issue_dic))
# print(pearsonr(Y,X1))
# print(spearmanr(Y,X1))


# print(pearsonr(Y,X2))
# print(spearmanr(Y,X2))


# print(pearsonr(Y,X3))
# print(spearmanr(Y,X3))


# print(pearsonr(Y,X4))
# print(spearmanr(Y,X4))



# print(pearsonr(Y,X5))
# print(spearmanr(Y,X5))

# print(pearsonr(Y,X6))
# print(spearmanr(Y,X6))


# print(X7)
print(pearsonr(Y,X7))
print(spearmanr(Y,X7))

