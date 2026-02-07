use crate::models::{KeyStat, TypedEntry};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub struct MetricSnapshot {
    pub wpm: f64,
    pub cpm: f64,
    pub accuracy: f64,
    pub errors: u64,
    pub elapsed_secs: u64,
}

pub fn compute_live(entries: &[TypedEntry], elapsed: Duration, blocked_errors: u64) -> MetricSnapshot {
    let elapsed_secs = elapsed.as_secs().max(1);
    let total_typed = entries.len() as u64;
    let correct = entries.iter().filter(|e| e.correct).count() as u64;
    let errors = blocked_errors + entries.iter().filter(|e| !e.correct).count() as u64;

    let minutes = elapsed_secs as f64 / 60.0;
    let wpm = if minutes > 0.0 {
        (correct as f64 / 5.0) / minutes
    } else {
        0.0
    };
    let cpm = if minutes > 0.0 {
        correct as f64 / minutes
    } else {
        0.0
    };
    let attempts = total_typed + blocked_errors;
    let accuracy = if attempts == 0 {
        100.0
    } else {
        (correct as f64 / attempts as f64) * 100.0
    };

    MetricSnapshot {
        wpm,
        cpm,
        accuracy,
        errors,
        elapsed_secs,
    }
}

pub fn update_key_stat(stat: &mut KeyStat, correct: bool, delta_ms: u64) {
    stat.attempts += 1;
    if correct {
        stat.correct += 1;
    }
    stat.total_time_ms += delta_ms;
}

pub fn problematic_keys(entries: &[TypedEntry], misses_by_key: &HashMap<char, u64>) -> Vec<char> {
    let mut counts: HashMap<char, u64> = HashMap::new();
    for entry in entries.iter().filter(|e| !e.correct) {
        *counts.entry(entry.expected).or_insert(0) += 1;
    }
    for (k, v) in misses_by_key {
        *counts.entry(*k).or_insert(0) += *v;
    }

    let mut pairs: Vec<(char, u64)> = counts.into_iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    pairs.into_iter().take(5).map(|(c, _)| c).collect()
}

pub fn sparkline_points(samples: &[f64], width: usize) -> Vec<u64> {
    if samples.is_empty() || width == 0 {
        return vec![];
    }
    let mut reduced = Vec::with_capacity(width);
    if samples.len() <= width {
        reduced.extend(samples.iter().map(|v| *v as u64));
        while reduced.len() < width {
            reduced.push(0);
        }
        return reduced;
    }

    let chunk = samples.len() as f64 / width as f64;
    for i in 0..width {
        let start = (i as f64 * chunk).floor() as usize;
        let end = (((i + 1) as f64 * chunk).ceil() as usize).min(samples.len());
        if start >= end {
            reduced.push(0);
            continue;
        }
        let avg = samples[start..end].iter().sum::<f64>() / (end - start) as f64;
        reduced.push(avg as u64);
    }
    reduced
}
