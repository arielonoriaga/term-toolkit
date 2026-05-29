# Plan Review — TTK Rust Rewrite

**Date:** 2026-05-28
**Verdict:** NEEDS FIXES — 3 critical before implementation starts

---

## Critical fixes required

### C1: Git log delimiter → `%x00` (null byte)
**Where:** Task 2, `git_log` format string + `parse_commit_line`
**Problem:** `splitn(5, '|')` makes `test_parse_commit_line_subject_with_pipe` impossible to pass. Subject with `|` consumes a slot, pushing date to position 5 (out of bounds). `parse_commit_line` returns `None`, test panics.
**Fix:**
```rust
// git_log: change format
"--format=%H%x00%ae%x00%an%x00%s%x00%ad",

// parse_commit_line: change split
let parts: Vec<&str> = line.splitn(5, '\0').collect();
```

### C2: `parse_date` → return `Result`, not `panic!`
**Where:** Task 10, `main.rs`
**Problem:** `--since bad-input` panics instead of `eprintln! + exit 1`. Breaks spec §5 V1 + the whole plan's `Result<(), String>` error pattern.
**Fix:**
```rust
fn parse_date(s: &str) -> Result<DateTime<Local>, String> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| format!("invalid date '{}', use YYYY-MM-DD", s))?
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(Local)
        .single()
        .ok_or_else(|| format!("ambiguous local time for '{}'", s))
}
// usage in main: since.as_deref().map(parse_date).transpose()?
```

### C3: `--until DATE` excludes the full day
**Where:** Task 10, `parse_date`
**Problem:** `--until 2026-05-31` → `2026-05-31T00:00:00`. `git log --before=2026-05-31T00:00:00` excludes ALL commits on May 31. Wizard correctly uses `23:59:59` for `until`. Two code paths, two behaviors.
**Fix:** Separate helper for `until` that sets `and_hms_opt(23, 59, 59)`:
```rust
fn parse_since_date(s: &str) -> Result<DateTime<Local>, String> { parse_date_at(s, 0, 0, 0) }
fn parse_until_date(s: &str) -> Result<DateTime<Local>, String> { parse_date_at(s, 23, 59, 59) }

fn parse_date_at(s: &str, h: u32, m: u32, sec: u32) -> Result<DateTime<Local>, String> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| format!("invalid date '{}', use YYYY-MM-DD", s))?
        .and_hms_opt(h, m, sec)
        .unwrap()
        .and_local_timezone(Local)
        .single()
        .ok_or_else(|| format!("ambiguous local time for '{}'", s))
}
```

---

## High priority (fix before or during implementation)

### H1: `rename` data-loss — silent overwrite
**Where:** Task 6, `rename/lib.rs`
**Problem:** `fs::rename(old, new)` on Linux replaces `new` if it exists. Partial rename leaves mixed names → collision mid-run destroys files.
**Fix:** Before each rename:
```rust
if new_path.exists() {
    return Err(format!("collision: {} already exists", new_path.display()));
}
```

### H2: `optimize_file` JPEG — behavioral regression
**Where:** Task 7, `optimize_file` JPEG branch
**Problem:** TS: `output=Some && keep_original=false` → writes to output AND overwrites original. Rust: only writes to output.
**Fix:** After JPEG write to output, if `!keep_original { fs::write(input, &buf)?; }`

### H3: `clap` bool default `true` — user can never set to `false`
**Where:** Task 10, `--keep-original` / `--reset` flags
**Problem:** `#[arg(long, default_value_t = true)]` on `bool` — no negation flag defined. User can't opt out.
**Fix:** Add negation flag:
```rust
#[arg(long, default_value_t = true)]
keep_original: bool,
#[arg(long = "no-keep-original", overrides_with = "keep_original")]
_no_keep_original: bool,
```

### H4: No git timeout
**Where:** Task 2, `git_log`
**Problem:** One hung git process (NFS, corrupted pack, large repo) blocks entire scan. No timeout, no way to interrupt per-repo.
**Mitigation:** Add `wait-timeout` crate or document as known limitation. At minimum: emit "scanning /foo..." before each call so user can Ctrl-C.

---

## Medium (quality issues, fix when touching that task)

- **M1:** `AuthorGroups = HashMap` → random iteration order → non-deterministic markdown. Change to `IndexMap` (insertion order) or `BTreeMap` (sorted), or sort keys before rendering.
- **M2:** `test_optimize_overwrites_original_when_no_keep` — unused `original_size` var. Test doesn't verify its own name. Assert file content changed.
- **M3:** Non-TTY with no flags → `inquire` panics. Consider: if `!std::io::stdin().is_terminal()` and no date flags → return `Err("no TTY detected, use --last or --since/--until")`.
- **M4:** `clone-repo` reset: `git commit` without user identity fails in containers. Pass `-c user.email=noreply -c user.name=ttk` as git args for the reset commit.
