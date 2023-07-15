import csv
import os
from collections import Counter
from collections import defaultdict
import re
import statistics
import ast
from datetime import datetime
import sys
import re

csv.field_size_limit(sys.maxsize)

def read_from_csv(path):
	csv_reader = csv.reader(open(path,'r'))

	data = []
	for row in csv_reader:
		data.append(row)
	return(data)


def get_top_15_commit_file(commit_files):
	# print(commit_files[0])
	print("The top 15 commit files:")
	f_names = []
	for fname in commit_files[1:]:
		f_names.append(fname[1])
	# print(f_names)

	counter = Counter(f_names)
	# 按照出现次数的降序排列
	summary = sum(counter.values())
	sorted_counter = sorted(counter.items(), key=lambda x: x[1], reverse=True)
	# 打印结果
	i = 1
	for item, count in sorted_counter[:15]:
	    # print('Top',f"{i}   {item}: {count}")
	    print(item)
	    # print(count)
	    # print(round(count/summary*100,4))
	    i = i + 1



def get_top_15_file_type(project_path):
	# file_counts = defaultdict(int)
	count = 0
	extensions = []    
	for root, dirs, files in os.walk(project_path):
		for file in files:
			count =count + 1
			_, file_extension = os.path.splitext(file)
			extensions.append(file_extension)
			# file_counts[file_extension] += 1

	print("The total number of files:",count)
	counter = Counter(extensions)
	# print(type(counter))
	# 按照出现次数的降序排列
	summary = sum(counter.values())
	sorted_counter = sorted(counter.items(), key=lambda x: x[1], reverse=True)
	# 打印结果

	# print(type(sorted_counter))
	i = 1
	for item, count in sorted_counter[:10]:
	    # print('label_list',f"{i}   {item}: {count} {count/summary}")
	    # print(item)
	    # print(round(count/summary*100,4))
	    print(count)
	    i = i + 1	



def count_lines_of_code(lines):
    code_lines = 0
    comment_lines = 0
    in_comment_block = False

    for line in lines:
        line = line.strip()

        # 忽略空行
        if not line:
            continue

        # 忽略单行注释
        if line.startswith("#"):
            comment_lines += 1
            continue

        # 处理多行注释
        if line.startswith("'''") or line.startswith('"""'):
            if in_comment_block:
                in_comment_block = False
            else:
                in_comment_block = True

            comment_lines += 1
            continue

        # 去除多行注释中的行
        if in_comment_block:
            comment_lines += 1
            continue

        # 计算代码行
        code_lines += 1

    return code_lines


def get_project_loc(project_path):

	loc_list = []
	file_num = 0
	for root, dirs, files in os.walk(project_path):
		for file in files:
			file_num = file_num + 1
			if file.endswith(".py"):
				file_path = os.path.join(root,file)
				# print(file_path)

				lines = []
				try:
					with open(file_path, 'r') as file:
						lines = file.readlines()
				except:
					print("Loc parse Error:str(e)")

				loc = count_lines_of_code(lines)

				loc_list.append(loc)
	print("File Number:",file_num)
	sumary = sum(loc_list)
	print("summary:",sumary)
	mean = statistics.mean(loc_list)
	print("平均数：", mean)
	# 中位数
	median = statistics.median(loc_list)
	print("中位数：", median)




def analyze_test_case(source_code):
    tree = ast.parse(source_code)

    # 初始化计数器
    variable_count = 0
    function_count = 0
    class_count = 0
    scope_count = 0

    # 遍历语法树
    for node in ast.walk(tree):
        if isinstance(node, ast.Name):
            # 统计变量数量
            variable_count += 1
        elif isinstance(node, ast.FunctionDef):
            # 统计函数数量
            function_count += 1
        elif isinstance(node, ast.ClassDef):
            # 统计类数量
            class_count += 1
        elif isinstance(node, (ast.FunctionDef, ast.ClassDef)):
            # 统计作用域数量（函数和类）
            scope_count += 1

    return variable_count, function_count,class_count,scope_count
    # # 打印统计结果
    # print("变量数量:", variable_count)
    # print("函数数量:", function_count)
    # print("类数量:", class_count)
    # print("作用域数量:", scope_count)


# file = '/home/xxm/Desktop/empirical_bug_rust_compiler/test_case/Rust-GCC_gccrs/182_767512872.rs'

def rust_file_statistic(file_content):

	def count_variables(file_content):
	    pattern = r'\b(let|mut)\s+(\w+)\b'
	    matches = re.findall(pattern, file_content)
	    return len(matches)

	def count_functions(file_content):
	    pattern = r'\bfn\s+(\w+)\b'
	    matches = re.findall(pattern, file_content)
	    return len(matches)

	def count_classes(file_content):
	    pattern = r'\bstruct\s+(\w+)\b'
	    matches = re.findall(pattern, file_content)
	    return len(matches)

	def count_scopes(file_content):
	    pattern = r'(\{|\})'
	    matches = re.findall(pattern, file_content)
	    return len(matches)

	def count_lines_without_comments(file_content):
	    lines = file_content.split('\n')
	    line_count = 0
	    comment_flag = False

	    for line in lines:
	        line = line.strip()

	        # 忽略空行
	        if not line:
	            continue

	        # 忽略单行注释
	        if line.startswith('//'):
	            continue

	        # 处理多行注释
	        if '/*' in line and '*/' not in line:
	            comment_flag = True
	            continue

	        if '*/' in line:
	            comment_flag = False
	            continue

	        # 忽略多行注释中的代码行
	        if comment_flag:
	            continue

	        line_count += 1

	    return line_count

	# # 读取Rust文件内容
	# with open(file, 'r') as file:
	#     file_content = file.read()


	variable_count = count_variables(file_content)
	function_count = count_functions(file_content)
	class_count = count_classes(file_content)
	scope_count = count_scopes(file_content)
	line_count = count_lines_without_comments(file_content)
	return variable_count,function_count,class_count,scope_count,line_count








def get_test_case_rust_feature(test_cases):
	ownership = []
	pattern =[]
	Error =[]
	concurrency = []
	macro =[]
	trait = []
	unsafe = []


	for testcase in test_cases[1:]:
		code = testcase[3]
		# Count ownership and borrowing
		ownership.append(len(re.findall(r'&[^\s]+', code)))
	    # Count pattern matching
		pattern.append(len(re.findall(r'\bmatch\b', code)))

	    # Count error handling
		Error.append(len(re.findall(r'\bResult\b', code)))

	    # Count concurrency (threads)
		concurrency.append(len(re.findall(r'\bthread::', code)))

	    # Count macros
		macro.append(len(re.findall(r'\bmacro_rules!\b', code)))

		trait.append(len(re.findall(r"\btrait\s*<[^>]+>\s*", code)))

		unsafe.append(len(re.findall(r'\bunsafe\b', code)))

	mean = statistics.mean(ownership)
	print("ownership平均数：", mean)
	# median = statistics.median(ownership)
	# print("ownership中位数：", median)
	summary = sum(ownership)
	print("ownership summary：", summary)

	mean = statistics.mean(pattern)
	print("pattern平均数：", mean)
	# median = statistics.median(pattern)
	# print("pattern中位数：", median)
	summary = sum(pattern)
	print("pattern summary：", summary)

	mean = statistics.mean(Error)
	print("Error平均数：", mean)
	# median = statistics.median(Error)
	# print("Error中位数：", median)
	summary = sum(Error)
	print("Errorsummary：", summary)

	mean = statistics.mean(concurrency)
	print("concurrency：", mean)
	# median = statistics.median(concurrency)
	# print("concurrency中位数：", median)
	summary = sum(concurrency)
	print("concurrency summary：", summary)

	mean = statistics.mean(macro)
	print("macro 平均数：", mean)
	# median = statistics.median(macro)
	# print("macro 中位数：", median)
	summary = sum(macro)
	print("macrosummary：", summary)

	mean = statistics.mean(trait)
	print("trait平均数：", mean)
	# median = statistics.median(trait)
	# print("trait中位数：", median)
	summary = sum(trait)
	print("trait summary：", summary)

	mean = statistics.mean(unsafe)
	print("unsafe平均数：", mean)
	# median = statistics.median(trait)
	# print("trait中位数：", median)
	summary = sum(unsafe)
	print("unsafe summary：", summary)



def get_test_case_stats(test_cases):

	loc_list = []
	var_list = []
	func_list = []
	class_list = []
	scope_list = []

	for testcase in test_cases[1:]:
		# print(code)
		code = testcase[3]
		# codelines = code.split("\n")
		# print(codelines[0])


	# 	# if 'py' in code[0]:
		# try:
		# 	ast.parse(code)
		# 	# print(code)
		# 	code_lines=code.split("\n")
		# 	loc_list.append(count_lines_of_code(code_lines))

		# 	var,func,clas,scope = analyze_test_case(code)
		# 	var_list.append(var)
		# 	func_list.append(func)
		# 	class_list.append(clas)
		# 	scope_list.append(scope)
		# 	# print(var,func,clas,scope )

		# except:
		# 	pass
		# 	# print("Test case Loc parse Error:str(e)")
		var,func,clas,scope,loc = 	rust_file_statistic(code)
		if var == 0 or loc > 60 or scope > 60: 
			pass
		else:
			loc_list.append(loc)
			var_list.append(var)
			func_list.append(func)
			class_list.append(clas)
			scope_list.append(scope)



	print("Total python file:",len(loc_list))

	mean = statistics.mean(loc_list)
	print("loc 平均数：", mean)
	# 中位数
	median = statistics.median(loc_list)
	print("loc 中位数：", median)

	counter = Counter(loc_list )
	# 按照出现次数的降序排列
	summary = sum(counter.values())
	sorted_counter = sorted(counter.items(), key=lambda x: x[1], reverse=True)
	# 打印结果
	i = 1
	for item, count in sorted_counter[:20]:
	    # print('loc Top',f"{i}   {item}: {count}")
	    # print(item)
	    print(count)
	    # print(round(count/summary*100,4))
	    i = i + 1


	print("\n")
	mean = statistics.mean(var_list)
	print("var 平均数：", mean)
	# 中位数
	median = statistics.median(var_list)
	print("var 中位数：", median)


	counter = Counter(var_list)
	# 按照出现次数的降序排列
	summary = sum(counter.values())
	sorted_counter = sorted(counter.items(), key=lambda x: x[1], reverse=True)
	# 打印结果
	i = 1
	for item, count in sorted_counter[:20]:
	    # print('var Top',f"{i}   {item}: {count}")
	    # print(item)
	    print(count)
	    # print(round(count/summary*100,4))
	    i = i + 1



	print("\n")
	mean = statistics.mean(func_list)
	print("func 平均数：", mean)
	# 中位数
	median = statistics.median(func_list)
	print("func 中位数：", median)

	counter = Counter(func_list)
	# 按照出现次数的降序排列
	summary = sum(counter.values())
	sorted_counter = sorted(counter.items(), key=lambda x: x[1], reverse=True)
	# 打印结果
	i = 1
	for item, count in sorted_counter[:20]:
	    # print('func Top',f"{i}   {item}: {count}")
	    # print(item)
	    print(count)
	    # print(round(count/summary*100,4))
	    i = i + 1


	print("\n")
	mean = statistics.mean(class_list)
	print("class 平均数：", mean)
	# 中位数
	median = statistics.median(class_list)
	print("class 中位数：", median)


	counter = Counter(class_list)
	summary = sum(counter.values())
	# 按照出现次数的降序排列
	sorted_counter = sorted(counter.items(), key=lambda x: x[1], reverse=True)
	# 打印结果
	i = 1
	for item, count in sorted_counter[:20]:
	    # print('class Top',f"{i}   {item}: {count}")
	    # print(item)
	    print(count)
	    # print(round(count/summary*100,4))
	    i = i + 1


	print("\n")
	mean = statistics.mean(scope_list)
	print("scope 平均数：", mean)
	# 中位数
	median = statistics.median(scope_list)
	print("scope 中位数：", median)


	counter = Counter(scope_list)
	# 按照出现次数的降序排列
	sorted_counter = sorted(counter.items(), key=lambda x: x[1], reverse=True)
	# 打印结果
	i = 1
	for item, count in sorted_counter[:20]:
	    # print('scope Top',f"{i}   {item}: {count}")
	    # print(item)
	    print(count)
	    # print(round(count/summary*100,4))
	    i = i + 1
	  




def get_issue_state(issues):
	# print(issues[0])
	open_state_list =[]
	closed_state_list =[]
	for issue in issues[1:]:
		if issue[2] == "open":
			open_state_list.append(issue[3])
		else:
			closed_state_list.append(issue[3])


	counter = Counter(open_state_list)
	# 按照出现次数的降序排列
	sorted_counter = sorted(counter.items(), key=lambda x: x[1], reverse=True)
	# 打印结果
	i = 1
	for item, count in sorted_counter[:15]:
	    print('open issue state reason',f"{i}   {item}: {count}")
	    i = i + 1	


	counter = Counter(closed_state_list)
	# 按照出现次数的降序排列
	sorted_counter = sorted(counter.items(), key=lambda x: x[1], reverse=True)
	# 打印结果
	i = 1
	for item, count in sorted_counter[:15]:
	    print('closed issue state reason',f"{i}   {item}: {count}")
	    i = i + 1	



def get_top_10_issue_label(issues):
	label_list = []
	for issue in issues:
		label_str = issue[-2]
		if label_str:
			# print(label_str.split("|||")[1:])
			for label in label_str.split("|||")[1:]:
				label_list.append(label)


	counter = Counter(label_list)
	# print(type(counter))
	# 按照出现次数的降序排列
	summary = sum(counter.values())
	sorted_counter = sorted(counter.items(), key=lambda x: x[1], reverse=True)
	# 打印结果

	# print(type(sorted_counter))
	i = 1
	for item, count in sorted_counter[:10]:
	    print('label_list',f"{i}   {item}: {count} {count/summary}")
	    # print(item)
	    # print(round(count/summary*100,2))
	    # print(count)
	    i = i + 1	



def get_pull_statistics(pulls):

	state_list = []
	merged_list =[]
	for pull in pulls[1:]:
		state = pull[2]
		state_list.append(state)
		# print(state)
		merged = pull[19]
		merged_list.append(merged)




	counter = Counter(state_list)
	# 按照出现次数的降序排列
	sorted_counter = sorted(counter.items(), key=lambda x: x[1], reverse=True)
	# 打印结果
	i = 1
	for item, count in sorted_counter[:15]:
	    print('',f"{i}   {item}: {count}")
	    i = i + 1	

	counter = Counter(merged_list)
	# 按照出现次数的降序排列
	sorted_counter = sorted(counter.items(), key=lambda x: x[1], reverse=True)
	# 打印结果
	i = 1
	for item, count in sorted_counter[:15]:
	    print('',f"{i}   {item}: {count}")
	    i = i + 1




def get_commit_statistics(commits):

	commit_verification_verified_list = []
	stats_additions_list = []
	stats_deletions_list = []
	for commit in commits[1:]:
		commit_verification_verified = commit[4]
		commit_verification_verified_list.append(commit_verification_verified)
		stats_additions= int(commit[-4])
		stats_additions_list.append(stats_additions)
		stats_deletions = int(commit[-5])
		stats_deletions_list.append(stats_deletions)



	counter = Counter(commit_verification_verified_list)
	# 按照出现次数的降序排列
	sorted_counter = sorted(counter.items(), key=lambda x: x[1], reverse=True)
	# 打印结果
	i = 1
	for item, count in sorted_counter[:15]:
	    print('',f"{i}   {item}: {count}")
	    i = i + 1

	print("summary of additions",sum(stats_additions_list))
	print("summary of deletions",sum(stats_deletions_list))



def partition_data(data, bins):
    """
    将数据分成不同的区间
    
    参数：
    data (list): 包含数据的列表
    bins (list): 包含区间的列表，按升序排列
    
    返回值：
    dict: 字典，键为区间范围，值为该区间范围内的数据
    """
    partitioned_data = {}
    for value in data:
        for i in range(len(bins) - 1):
            if bins[i] <= value < bins[i + 1]:
                if bins[i] not in partitioned_data:
                    partitioned_data[bins[i]] = []
                partitioned_data[bins[i]].append(value)
                break
    return partitioned_data




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


def get_issue_duration(issues):

	issue_duration_list = []
	for issue in issues[1:]:
		issue_created_at = issue[5]
		issue_closed_at = issue[7]
		if issue_closed_at:
			issue_duration = calculate_time_difference(issue_created_at,issue_closed_at)
			issue_duration_list.append(issue_duration)


	# mn = statistics.mean(issue_duration_list)
	# print("mean:",mn)
	# md = statistics.median(issue_duration_list)
	# print("median",mn)
	# print(issue_duration_list)
	# # 测试函数
	# # data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
	bins = [0, 24, 72, 240,720, 8760, 100000000]
	partitioned_data = partition_data(issue_duration_list, bins)
	for key, values in partitioned_data.items():
	    print(f"区间 {key} 的数据: {len(values)}",round(len(values)/len(issue_duration_list)*100,2))



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
    top_10_key_value_pairs = sorted_items[:10]  # 获取前10个键值对
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
							print(file_sha)
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
	print(find_top_10_key_value_pairs(file_to_num_issue))	






# reponame = "rust-lang/rust"
reponame = "Rust-GCC/gccrs"
# reponame = "pylint-dev/pylint"
# reponame ="microsoft/pyright"
# reponame = "google/pytype"





project = reponame.replace("/","_")


print(reponame)
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




# print("The number of commits:", len(commits)-1)

# print("The number of issues:", len(issues)-1)


# print("The number of pulls:", len(pulls)-1)
# 
# print("The number of modified files:", len(commit_files)-1)

# get_top_15_commit_file(commit_files)
# get_top_15_file_type(project_path)

# get_project_loc(project_path)
# get_test_case_stats(test_cases)

# get_top_10_issue_label(issues)

get_test_case_rust_feature(test_cases)
# get_issue_state(issues)
# get_pull_statistics(pulls)
# get_commit_statistics(commits)

# get_issue_duration(issues)



# get_issue_modi_file(issue_to_pull_dic, pull_to_commit_dic, commit_to_file_dic,commit_file_dic)

