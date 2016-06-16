import os
from pprint import pprint

def main():
    counter = {}
    file_path = os.path.join(os.curdir, 'text', 'alice.txt')
    with open(file_path) as f:
        for line in f:
            words = line.split()
            for word in words:
                word.lower()
                if word in counter.keys():
                    counter[word] += 1
                else:
                    counter[word] = 1

    pprint(counter)



if __name__=='__main__':
  main()
