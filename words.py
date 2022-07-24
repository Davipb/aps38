import re


with open('words.txt', 'r') as file:
    words = file.readlines()

with open('5words.txt', 'w') as file:
    seen = set()
    for word in words:        
        word = word.lower().replace('\n', '').replace('\r', '')
        if not re.match('^[a-z]{5}$', word):
            continue

        letters = ''.join(sorted(set(word)))
        if len(letters) != 5:
            print('IGNORED', word, '- Duplicate letters')
            continue
        
        if letters in seen:
            print('IGNORED', word, '- Seen')
            continue

        seen.add(letters)
        file.write(word + '\n')
