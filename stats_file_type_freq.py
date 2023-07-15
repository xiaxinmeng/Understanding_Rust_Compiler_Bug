import csv

import os
from collections import defaultdict

def count_files_by_type(directory):
    file_counts = defaultdict(int)
    
    for root, dirs, files in os.walk(directory):
        for file in files:
            _, file_extension = os.path.splitext(file)
            file_counts[file_extension] += 1

    return dict(file_counts)





def print_file_counts(file_counts):
    sorted_counts = sorted(file_counts.items(), key=lambda x: x[1], reverse=True)

    for file_extension, count in sorted_counts:
        print(f"{file_extension}，{count}")

    for file_extension, count in sorted_counts:
        print(f"{count}")

    for file_extension, count in sorted_counts:
        print(f"{file_extension}")


# directory = '/Users/xiaxinmeng/Desktop/research/empirical_type_checking/empircal/dataset/project/mypy'
# directory = '/Users/xiaxinmeng/Desktop/research/empirical_type_checking/empircal/dataset/project/pyright'
# directory = '/Users/xiaxinmeng/Desktop/research/empirical_type_checking/empircal/dataset/project/pytype'
directory = '/Users/xiaxinmeng/Desktop/research/empirical_type_checking/empircal/dataset/project/pylint'
file_counts = count_files_by_type(directory)
print_file_counts(file_counts)