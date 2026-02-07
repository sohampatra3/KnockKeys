use crate::models::{CodeTopic, LanguageMode};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub fn generate_code_drill_text(
    language: LanguageMode,
    topic: CodeTopic,
    snippet_count: usize,
    seed: u64,
) -> String {
    let mut rng = StdRng::seed_from_u64(seed);
    let pool = snippet_pool(language, topic);

    let count = snippet_count.max(1);
    let mut out = String::new();
    for i in 0..count {
        if i > 0 {
            out.push(' ');
        }
        let idx = rng.gen_range(0..pool.len());
        out.push_str(pool[idx]);
    }

    out.trim().to_string()
}

fn snippet_pool(language: LanguageMode, topic: CodeTopic) -> &'static [&'static str] {
    match (language, topic) {
        (LanguageMode::Cpp, CodeTopic::Loops) => CPP_LOOPS,
        (LanguageMode::Cpp, CodeTopic::IfElse) => CPP_IF_ELSE,
        (LanguageMode::Cpp, CodeTopic::Sorting) => CPP_SORTING,
        (LanguageMode::Cpp, CodeTopic::Searching) => CPP_SEARCHING,
        (LanguageMode::Cpp, CodeTopic::Mixed) => CPP_MIXED,

        (LanguageMode::C, CodeTopic::Loops) => C_LOOPS,
        (LanguageMode::C, CodeTopic::IfElse) => C_IF_ELSE,
        (LanguageMode::C, CodeTopic::Sorting) => C_SORTING,
        (LanguageMode::C, CodeTopic::Searching) => C_SEARCHING,
        (LanguageMode::C, CodeTopic::Mixed) => C_MIXED,

        (LanguageMode::Rust, CodeTopic::Loops) => RUST_LOOPS,
        (LanguageMode::Rust, CodeTopic::IfElse) => RUST_IF_ELSE,
        (LanguageMode::Rust, CodeTopic::Sorting) => RUST_SORTING,
        (LanguageMode::Rust, CodeTopic::Searching) => RUST_SEARCHING,
        (LanguageMode::Rust, CodeTopic::Mixed) => RUST_MIXED,

        (LanguageMode::Python, CodeTopic::Loops) => PY_LOOPS,
        (LanguageMode::Python, CodeTopic::IfElse) => PY_IF_ELSE,
        (LanguageMode::Python, CodeTopic::Sorting) => PY_SORTING,
        (LanguageMode::Python, CodeTopic::Searching) => PY_SEARCHING,
        (LanguageMode::Python, CodeTopic::Mixed) => PY_MIXED,

        (LanguageMode::Go, CodeTopic::Loops) => GO_LOOPS,
        (LanguageMode::Go, CodeTopic::IfElse) => GO_IF_ELSE,
        (LanguageMode::Go, CodeTopic::Sorting) => GO_SORTING,
        (LanguageMode::Go, CodeTopic::Searching) => GO_SEARCHING,
        (LanguageMode::Go, CodeTopic::Mixed) => GO_MIXED,
    }
}

const CPP_LOOPS: &[&str] = &[
    "for (int i = 0; i < n; i++) { sum += i; }",
    "while (left < right) { left++; right--; }",
    "for (auto v : nums) { total += v; }",
    "int i = 0; do { i++; } while (i < limit);",
];

const CPP_IF_ELSE: &[&str] = &[
    "if (x % 2 == 0) { kind = \"even\"; } else { kind = \"odd\"; }",
    "if (score >= 90) { grade = 'a'; } else if (score >= 80) { grade = 'b'; }",
    "if (ok && ready) { run(); } else { wait(); }",
];

const CPP_SORTING: &[&str] = &[
    "std::sort(nums.begin(), nums.end());",
    "std::stable_sort(items.begin(), items.end(), by_key);",
    "for (size_t i = 0; i + 1 < n; i++) { if (a[i] > a[i + 1]) std::swap(a[i], a[i + 1]); }",
];

const CPP_SEARCHING: &[&str] = &[
    "auto it = std::find(nums.begin(), nums.end(), target);",
    "bool ok = std::binary_search(nums.begin(), nums.end(), key);",
    "int pos = -1; for (int i = 0; i < n; i++) { if (a[i] == key) { pos = i; break; } }",
];

const CPP_MIXED: &[&str] = &[
    "for (int i = 0; i < n; i++) { if (a[i] < minv) { minv = a[i]; } }",
    "std::sort(nums.begin(), nums.end()); bool hit = std::binary_search(nums.begin(), nums.end(), key);",
    "int pos = -1; for (int i = 0; i < n; i++) { if (a[i] == key) { pos = i; break; } }",
];

const C_LOOPS: &[&str] = &[
    "for (int i = 0; i < n; i++) { sum += i; }",
    "while (idx < len) { total += arr[idx]; idx++; }",
    "int i = 0; do { i++; } while (i < limit);",
    "for (int j = n - 1; j >= 0; j--) { out += j; }",
];

const C_IF_ELSE: &[&str] = &[
    "if (x > y) { max = x; } else { max = y; }",
    "if (flag) { state = 1; } else if (retry) { state = 2; } else { state = 0; }",
    "if (count == 0) { empty = 1; } else { empty = 0; }",
];

const C_SORTING: &[&str] = &[
    "for (int i = 0; i < n - 1; i++) { for (int j = 0; j < n - i - 1; j++) { if (a[j] > a[j + 1]) { int t = a[j]; a[j] = a[j + 1]; a[j + 1] = t; } } }",
    "qsort(a, n, sizeof(int), cmp_int);",
    "for (int i = 1; i < n; i++) { int k = a[i]; int j = i - 1; while (j >= 0 && a[j] > k) { a[j + 1] = a[j]; j--; } a[j + 1] = k; }",
];

const C_SEARCHING: &[&str] = &[
    "int pos = -1; for (int i = 0; i < n; i++) { if (a[i] == key) { pos = i; break; } }",
    "int lo = 0, hi = n - 1; while (lo <= hi) { int mid = (lo + hi) / 2; if (a[mid] == key) { return mid; } if (a[mid] < key) lo = mid + 1; else hi = mid - 1; }",
    "char *p = strstr(text, pat); if (p) { found = 1; }",
];

const C_MIXED: &[&str] = &[
    "for (int i = 0; i < n; i++) { if (a[i] < minv) { minv = a[i]; } }",
    "qsort(a, n, sizeof(int), cmp_int); int lo = 0, hi = n - 1;",
    "int pos = -1; for (int i = 0; i < n; i++) { if (a[i] == key) { pos = i; break; } }",
];

const RUST_LOOPS: &[&str] = &[
    "for i in 0..n { sum += i; }",
    "while idx < nums.len() { total += nums[idx]; idx += 1; }",
    "for v in nums.iter() { total += *v; }",
    "loop { i += 1; if i >= limit { break; } }",
];

const RUST_IF_ELSE: &[&str] = &[
    "if x > y { max = x; } else { max = y; }",
    "let kind = if n % 2 == 0 { \"even\" } else { \"odd\" };",
    "if ok && ready { run = true; } else if retry { run = maybe(); }",
];

const RUST_SORTING: &[&str] = &[
    "nums.sort();",
    "nums.sort_by(|a, b| a.cmp(b));",
    "items.sort_by_key(|it| it.key);",
    "for i in 1..a.len() { let mut j = i; while j > 0 && a[j - 1] > a[j] { a.swap(j - 1, j); j -= 1; } }",
];

const RUST_SEARCHING: &[&str] = &[
    "if let Some(pos) = nums.iter().position(|v| *v == key) { found = pos as i32; }",
    "match nums.binary_search(&key) { Ok(i) => hit = i as i32, Err(_) => hit = -1 }",
    "for (i, v) in nums.iter().enumerate() { if *v == key { pos = i as i32; break; } }",
];

const RUST_MIXED: &[&str] = &[
    "for i in 0..n { if a[i] < minv { minv = a[i]; } }",
    "nums.sort(); match nums.binary_search(&key) { Ok(i) => hit = i as i32, Err(_) => hit = -1 }",
    "if let Some(pos) = nums.iter().position(|v| *v == key) { found = pos as i32; }",
];

const PY_LOOPS: &[&str] = &[
    "for i in range(n): total += i",
    "while left < right: left += 1; right -= 1",
    "for v in nums: total += v",
    "for i, v in enumerate(nums): acc += i + v",
];

const PY_IF_ELSE: &[&str] = &[
    "if x > y: best = x else: best = y",
    "label = 'even' if n % 2 == 0 else 'odd'",
    "if ok and ready: run() elif retry: wait() else: stop()",
];

const PY_SORTING: &[&str] = &[
    "nums.sort()",
    "nums = sorted(nums)",
    "items.sort(key=lambda x: x[1])",
    "for i in range(len(a) - 1): if a[i] > a[i + 1]: a[i], a[i + 1] = a[i + 1], a[i]",
];

const PY_SEARCHING: &[&str] = &[
    "pos = -1; for i, v in enumerate(nums): if v == key: pos = i; break",
    "if key in nums: found = nums.index(key)",
    "lo, hi = 0, len(nums) - 1",
];

const PY_MIXED: &[&str] = &[
    "nums.sort(); pos = -1; for i, v in enumerate(nums): if v == key: pos = i; break",
    "for i in range(n): if arr[i] < minv: minv = arr[i]",
    "label = 'ok' if score >= 50 else 'retry'",
];

const GO_LOOPS: &[&str] = &[
    "for i := 0; i < n; i++ { sum += i }",
    "for idx < len(nums) { total += nums[idx]; idx++ }",
    "for _, v := range nums { total += v }",
    "for { i++; if i >= limit { break } }",
];

const GO_IF_ELSE: &[&str] = &[
    "if x > y { best = x } else { best = y }",
    "if n%2 == 0 { kind = \"even\" } else { kind = \"odd\" }",
    "if ok && ready { run() } else if retry { wait() } else { stop() }",
];

const GO_SORTING: &[&str] = &[
    "sort.Ints(nums)",
    "sort.Slice(items, func(i, j int) bool { return items[i].key < items[j].key })",
    "for i := 0; i < len(a)-1; i++ { if a[i] > a[i+1] { a[i], a[i+1] = a[i+1], a[i] } }",
];

const GO_SEARCHING: &[&str] = &[
    "pos := -1; for i, v := range nums { if v == key { pos = i; break } }",
    "idx := sort.SearchInts(nums, key)",
    "for lo <= hi { mid := (lo + hi) / 2; if nums[mid] == key { return mid } }",
];

const GO_MIXED: &[&str] = &[
    "sort.Ints(nums); pos := sort.SearchInts(nums, key)",
    "for i := 0; i < n; i++ { if arr[i] < minv { minv = arr[i] } }",
    "if ok && ready { run() } else { wait() }",
];
