import os

new_crawler_path = '/home/xxm/Desktop/empirical_bug_rust_compiler/crawler'
new_project = "empirical_bug_rust_compiler"



for root, dirs,files in os.walk(new_crawler_path):
	for file in files:
		if file.endswith(".py"):
			path = root + "/"+file
			print(path)
			f1 = open(path,'r')
			content =f1.read()
			f1.close()
			content.replace('empirical_bug',new_project)
			print(content)
			f2 = open(path,'w')
			f2.write(content)
			f2.close()
