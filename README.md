# A problem squared 38

On episode 38 of the A Problem Squared podcast, someone asked "Can you guess 5 unique words with unique letters in Wordle?".
One of the hosts, Matt Parker, created a script to find the solution in a month. 
This program can do it in just under an hour.

## How it works

`5words.txt` contains 5977 English words that are exactly 5 characters long and have no repeating letters, with anagrams removed.

The program chooses one then adds all other words that have the same letters to a blocklist.
For example, if [fldxt](https://www.collinsdictionary.com/dictionary/english/fldxt) was chosen, then any word that contains the letter f, l, d, x, or t will be added to the blocklist.
The program then chooses a non-blocked word and repeats the process until either 5 words have been found or all possible words are in the blocklist, in which case it goes back one step and tries another word.

The program also keeps track of word combinations it already tried to prevent duplicate effort.
The blocklist and other operations use bit-masking operations to make the calculations faster.

## Results

After checking 110014510 possible combinations in 57 minutes, the program found 538 combinations that worked, which is consistent with the results Matt got.

## How to run it

[Install Rust](https://www.rust-lang.org/tools/install) then just run `cargo run --release`.
The output will be written to `result.txt`.
