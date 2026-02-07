use crate::models::{KeyStat, ProgressData, Settings, TestTextType};
use rand::distributions::{Distribution, WeightedIndex};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;

pub const HOME_ROW_START: &[char] = &['f', 'j', 'd', 'k', 's', 'l', 'a', ';'];
pub const UNLOCK_SEQUENCE: &[char] = &[
    'g', 'h', 'r', 'u', 'e', 'i', 'w', 'o', 'q', 'p', 't', 'y', 'v', 'm', 'c', 'n', 'x', 'b', 'z',
    ',', '.', '/', '[', ']', '\\', '-', '=', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
];

const WORDS: &[&str] = &[
    "alpha", "binary", "cable", "drift", "ember", "frame", "globe", "honey", "input", "jolly",
    "knock", "laser", "macro", "noble", "orbit", "pixel", "quiet", "river", "scale", "tango",
    "union", "vivid", "whale", "xenon", "young", "zesty", "adapt", "brisk", "clock", "delta",
    "event", "frost", "grain", "hover", "ideal", "jewel", "karma", "linen", "meter", "north",
    "ocean", "prism", "query", "raven", "solid", "trace", "urban", "verge", "woven", "yield",
    "zonal",
];

const QUOTES: &[&str] = &[
    "practice does not make perfect, only perfect practice makes progress.",
    "small daily improvements are the key to long term mastery.",
    "speed is a byproduct of accuracy and relaxed focus.",
    "consistency compounds faster than occasional intensity.",
    "measure your work, improve your weakest link, repeat.",
    "discipline is choosing what you want most over what you want now.",
    "clear feedback turns repetition into deliberate practice.",
];

pub fn coach_hint(target: char) -> (String, String) {
    let finger = finger_for_char(target).to_string();
    let base = primary_key_for_char(target);
    let mut guidance = if "asdfjkl;".contains(base) {
        "Home row anchor".to_string()
    } else if "qwertyuiop[]\\1234567890-=`".contains(base) {
        "Top row reach".to_string()
    } else if "zxcvbnm,./".contains(base) {
        "Bottom row reach".to_string()
    } else if base == ' ' {
        "Tap space with thumb".to_string()
    } else {
        "Keep wrists neutral and return to home row".to_string()
    };

    if requires_shift(target) {
        let shift = match key_hand(base) {
            Some(Hand::Left) => "Use Right Shift (right pinky)",
            Some(Hand::Right) => "Use Left Shift (left pinky)",
            None => "Use opposite-hand Shift",
        };
        guidance = format!("{guidance}; {shift}");
    }

    (finger, guidance)
}

pub fn finger_for_char(c: char) -> &'static str {
    match primary_key_for_char(c) {
        'q' | 'a' | 'z' | '`' | '1' => "Left pinky",
        'w' | 's' | 'x' | '2' => "Left ring",
        'e' | 'd' | 'c' | '3' => "Left middle",
        'r' | 'f' | 'v' | 't' | 'g' | 'b' | '4' | '5' => "Left index",
        'y' | 'h' | 'n' | 'u' | 'j' | 'm' | '6' | '7' => "Right index",
        'i' | 'k' | ',' | '8' => "Right middle",
        'o' | 'l' | '.' | '9' => "Right ring",
        'p' | ';' | '\'' | '/' | '[' | ']' | '\\' | '-' | '=' | '0' => "Right pinky",
        ' ' => "Thumb",
        _ => "Unknown",
    }
}

pub fn primary_key_for_char(c: char) -> char {
    match c {
        '~' => '`',
        '!' => '1',
        '@' => '2',
        '#' => '3',
        '$' => '4',
        '%' => '5',
        '^' => '6',
        '&' => '7',
        '*' => '8',
        '(' => '9',
        ')' => '0',
        '_' => '-',
        '+' => '=',
        '{' => '[',
        '}' => ']',
        '|' => '\\',
        ':' => ';',
        '"' => '\'',
        '<' => ',',
        '>' => '.',
        '?' => '/',
        _ => c.to_ascii_lowercase(),
    }
}

fn requires_shift(c: char) -> bool {
    c.is_ascii_uppercase()
        || matches!(
            c,
            '~' | '!'
                | '@'
                | '#'
                | '$'
                | '%'
                | '^'
                | '&'
                | '*'
                | '('
                | ')'
                | '_'
                | '+'
                | '{'
                | '}'
                | '|'
                | ':'
                | '"'
                | '<'
                | '>'
                | '?'
        )
}

#[derive(Clone, Copy)]
enum Hand {
    Left,
    Right,
}

fn key_hand(base: char) -> Option<Hand> {
    match base {
        'q' | 'a' | 'z' | '`' | '1' | 'w' | 's' | 'x' | '2' | 'e' | 'd' | 'c' | '3' | 'r' | 'f'
        | 'v' | 't' | 'g' | 'b' | '4' | '5' => Some(Hand::Left),
        'y' | 'h' | 'n' | 'u' | 'j' | 'm' | '6' | '7' | 'i' | 'k' | ',' | '8' | 'o' | 'l' | '.'
        | '9' | 'p' | ';' | '\'' | '/' | '[' | ']' | '\\' | '-' | '=' | '0' => Some(Hand::Right),
        _ => None,
    }
}

pub fn lesson_top_row() -> Vec<char> {
    "qwertyuiop[]\\".chars().collect()
}

pub fn lesson_bottom_row() -> Vec<char> {
    "zxcvbnm,./".chars().collect()
}

pub fn weakest_keys(progress: &ProgressData, limit: usize) -> Vec<char> {
    let mut scored: Vec<(char, f64)> = progress
        .key_stats
        .iter()
        .filter(|(_, s)| s.attempts >= 3)
        .map(|(c, s)| {
            let score = (1.0 - s.accuracy()) * 3.0 + (s.avg_time_ms() / 350.0).min(2.0);
            (*c, score)
        })
        .collect();

    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let mut keys: Vec<char> = scored.into_iter().take(limit).map(|(c, _)| c).collect();

    if keys.is_empty() {
        keys = progress
            .unlocked_chars
            .iter()
            .copied()
            .take(limit)
            .collect();
    }
    if keys.is_empty() {
        keys = HOME_ROW_START.iter().copied().take(limit).collect();
    }
    keys
}

pub fn maybe_unlock_next(
    progress: &mut ProgressData,
    settings: &Settings,
    accuracy: f64,
    wpm: f64,
) -> Option<char> {
    let can_unlock = accuracy >= settings.unlock_accuracy_threshold
        && wpm >= settings.unlock_wpm_threshold
        && progress.unlocked_chars.len() < HOME_ROW_START.len() + UNLOCK_SEQUENCE.len();

    if !can_unlock {
        return None;
    }

    for next in UNLOCK_SEQUENCE {
        if !progress.unlocked_chars.contains(next) {
            progress.unlocked_chars.push(*next);
            return Some(*next);
        }
    }
    None
}

pub fn generate_training_text(
    allowed_chars: &[char],
    key_stats: &HashMap<char, KeyStat>,
    length: usize,
    seed: u64,
) -> String {
    let mut rng = StdRng::seed_from_u64(seed);
    let allowed = sanitize_allowed(allowed_chars);
    if allowed.is_empty() {
        return "asdf jkl;".to_string();
    }

    let weights = weights_for_chars(&allowed, key_stats);
    let dist = WeightedIndex::new(weights).unwrap();

    let mut out = String::with_capacity(length + 16);
    while out.len() < length {
        let word_len = rng.gen_range(3..=8);
        for _ in 0..word_len {
            let idx = dist.sample(&mut rng);
            out.push(allowed[idx]);
        }
        out.push(' ');
    }
    out.trim().to_string()
}

pub fn generate_test_text(
    text_type: TestTextType,
    approx_chars: usize,
    seed: u64,
    allowed_chars: &[char],
) -> String {
    match text_type {
        TestTextType::RandomWords => generate_random_words(approx_chars, seed),
        TestTextType::PseudoWords => {
            generate_training_text(allowed_chars, &HashMap::new(), approx_chars, seed)
        }
        TestTextType::Quotes => generate_quotes(approx_chars, seed),
    }
}

fn sanitize_allowed(chars: &[char]) -> Vec<char> {
    let mut out = Vec::new();
    for c in chars {
        let lower = c.to_ascii_lowercase();
        if !out.contains(&lower) && !lower.is_ascii_whitespace() {
            out.push(lower);
        }
    }
    out
}

fn weights_for_chars(chars: &[char], stats: &HashMap<char, KeyStat>) -> Vec<f64> {
    chars
        .iter()
        .map(|c| {
            let stat = stats.get(c).cloned().unwrap_or_default();
            if stat.attempts < 5 {
                3.2
            } else {
                1.0 + (stat.error_rate() * 4.0) + (stat.avg_time_ms() / 300.0).min(2.5)
            }
        })
        .collect()
}

fn generate_random_words(approx_chars: usize, seed: u64) -> String {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut out = String::with_capacity(approx_chars + 16);
    while out.len() < approx_chars {
        let w = WORDS[rng.gen_range(0..WORDS.len())];
        out.push_str(w);
        out.push(' ');
    }
    out.trim().to_string()
}

fn generate_quotes(approx_chars: usize, seed: u64) -> String {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut out = String::with_capacity(approx_chars + 16);
    while out.len() < approx_chars {
        let q = QUOTES[rng.gen_range(0..QUOTES.len())];
        out.push_str(q);
        out.push(' ');
    }
    out.trim().to_ascii_lowercase()
}
