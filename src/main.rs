use bitvec::prelude::*;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};

const WORD_COUNT: usize = 5977;
const LETTER_COUNT: usize = 26;
const LETTER_OFFSET_BASE: usize = 'a' as usize;
const SEARCH_MAX_DEPTH: usize = 5;
const FEEDBACK_THRESHOLD: usize = 10;

static RAW_WORDS: &'static str = include_str!("../5words.txt");

type WordBitArr = BitArr!(for WORD_COUNT);
type LetterBitArr = BitArr!(for LETTER_COUNT);
struct Word {
    value: &'static str,
    letters: LetterBitArr,
}

struct Letter {
    words: WordBitArr,
}

struct PrettyDuration(Duration);

type SeenSetElementWidth = u16;
type SeenSetElement = [SeenSetElementWidth; SEARCH_MAX_DEPTH];
struct SeenSet {
    set: HashSet<SeenSetElement>,
}

struct SearchContext {
    words: Vec<Word>,
    letters: Vec<Letter>,
    seen: SeenSet,
    current: WordBitArr,
    result: File,
}

fn main() {
    let words = make_words();
    let mut context = SearchContext {
        letters: make_letters(&words),
        words,
        current: WordBitArr::ZERO,
        seen: SeenSet::new(),
        result: File::create("result.txt").unwrap(),
    };

    let start = Instant::now();
    let count = search(&mut context, &WordBitArr::ZERO, 0);
    let end = Instant::now();
    println!("Checked {} combinations", count);
    println!("Time taken: {}s", end.duration_since(start).as_secs_f64());
}

fn search(context: &mut SearchContext, blocklist: &WordBitArr, depth: usize) -> usize {
    if !context.seen.insert(&context.current) {
        return 0;
    }

    if depth == SEARCH_MAX_DEPTH {
        for word in context.current.iter_ones().map(|i| &context.words[i]) {
            print!("{} ", word.value);
            write!(context.result, "{} ", word.value).unwrap();
        }
        println!();
        writeln!(context.result).unwrap();
        return 1;
    }

    let start = Instant::now();
    let mut count = 0;
    for next_index in blocklist.iter_zeros() {
        if next_index >= WORD_COUNT {
            continue;
        }

        let next_word = &context.words[next_index];
        let mut next_blocklist = blocklist.clone();
        next_word
            .letters
            .iter_ones()
            .map(|i| &context.letters[i].words)
            .for_each(|x| next_blocklist |= x);

        context.current.set(next_index, true);
        count += search(context, &next_blocklist, depth + 1);
        context.current.set(next_index, false);

        if depth == 0 && (next_index + 1) % FEEDBACK_THRESHOLD == 0 {
            let delta = Instant::now().duration_since(start);
            let time_per_word = delta / (next_index as u32 + 1);
            let eta_left = time_per_word * (WORD_COUNT - next_index - 1) as u32;
            let eta_total = delta + eta_left;
            eprintln!(
                "{:4} of {:4} | Spent: {} | ETA Total: {} | ETA Left: {} | Checked {}",
                next_index + 1,
                WORD_COUNT,
                PrettyDuration(delta),
                PrettyDuration(eta_total),
                PrettyDuration(eta_left),
                count
            );
        }
    }

    if count == 0 {
        return 1;
    }

    count
}

fn make_words() -> Vec<Word> {
    RAW_WORDS.lines().map(parse_word).collect()
}

fn parse_word(data: &'static str) -> Word {
    let mut letters = LetterBitArr::ZERO;
    data.chars()
        .map(Into::<u64>::into)
        .map(|x| x as usize)
        .map(|x| x - LETTER_OFFSET_BASE)
        .for_each(|x| letters.set(x, true));

    Word {
        value: data,
        letters,
    }
}

fn make_letters(words: &[Word]) -> Vec<Letter> {
    let mut result: Vec<Letter> = (0..LETTER_COUNT)
        .map(|x| x + LETTER_OFFSET_BASE)
        .map(|x| unsafe { char::from_u32_unchecked(x as u32) })
        .map(|_| Letter {
            words: WordBitArr::ZERO,
        })
        .collect();

    words
        .iter()
        .enumerate()
        .flat_map(|(i, x)| x.letters.iter_ones().map(move |y| (i, y)))
        .for_each(|(i, l)| result.get_mut(l).unwrap().words.set(i, true));

    result
}

impl Display for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Display for PrettyDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        const SECOND: u128 = 1000;
        const MINUTE: u128 = 60 * SECOND;
        const HOUR: u128 = 60 * MINUTE;

        let mut millis = self.0.as_millis();

        let hours = millis / HOUR;
        millis -= hours * HOUR;
        write!(f, "{:2}h ", hours)?;

        let minutes = millis / MINUTE;
        millis -= minutes * MINUTE;
        write!(f, "{:2}min ", minutes)?;

        let seconds = millis / SECOND;
        millis -= seconds * SECOND;
        write!(f, "{:2}s ", seconds)?;

        Ok(())
    }
}

impl SeenSet {
    const INVALID_SENTINEL: u16 = u16::MAX;

    fn new() -> SeenSet {
        SeenSet {
            set: HashSet::new(),
        }
    }

    fn insert(&mut self, value: &WordBitArr) -> bool {
        let mut compact = [Self::INVALID_SENTINEL; SEARCH_MAX_DEPTH];
        value
            .iter_ones()
            .map(|x| x as SeenSetElementWidth)
            .enumerate()
            .for_each(|(i, x)| compact[i] = x);

        self.set.insert(compact)
    }
}
