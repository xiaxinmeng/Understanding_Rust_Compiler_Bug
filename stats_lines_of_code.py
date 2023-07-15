import os

def count_lines(filepath):
    with open(filepath, 'r', encoding='utf-8') as file:
        lines = file.readlines()
    return len(lines)



def count_code_lines(directory):
    c = 0
    total_lines = 0
    for root, dirs, files in os.walk(directory):
        for file in files:
            try: 
                if file.endswith('.py') or file.endswith('.pyi'):
                # if file.endswith('.py'):
                	# print(filepath)
                    filepath = os.path.join(root, file)
                    print(filepath)
                    lines = count_lines(filepath)
                    total_lines += lines
            except:
               c = c+1
    print("%s fails"%c)
    return total_lines

directory = '/Users/xiaxinmeng/Desktop/research/empirical_type_checking/empircal/dataset/project/mypy'
directory = '/Users/xiaxinmeng/Desktop/research/empirical_type_checking/empircal/dataset/project/pylint'
# directory = '/Users/xiaxinmeng/Desktop/research/empirical_type_checking/empircal/dataset/project/pyright'
# directory = '/Users/xiaxinmeng/Desktop/research/empirical_type_checking/empircal/dataset/project/pytype'
total_lines = count_code_lines(directory)
print(f'Total lines of code: {total_lines}')
