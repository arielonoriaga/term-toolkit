---
name: ttk-rust-rewrite
description: Rewrite term-toolkit in Rust (Cargo workspace) + add git-digest command
metadata:
  type: project
---

# TTK Rust Rewrite + git-digest

**Date:** 2026-05-28
**Repo:** https://github.com/arielonoriaga/term-toolkit

## §1 Goal

Rewrite term-toolkit (currently TypeScript/Bun) in Rust.
Port ∀ 4 existing cmds. Add `git-digest` as new cmd.
Single binary `ttk`. Publish to crates.io.

## §2 Architecture

Cargo workspace. `package.json` stays for npm legacy publish.

```
term-toolkit/
├── Cargo.toml            # [workspace] root
├── Cargo.lock
├── package.json          # legacy npm publish
├── crates/
│   ├── core/             # shared lib: git ops, prompts, output, markdown
│   ├── cli/              # binary entry — clap subcommand router
│   ├── deleter/          # port: delete files by index (even/odd)
│   ├── rename/           # port: rename files by sequence index
│   ├── optimize/         # port: image optimization
│   ├── clone-repo/       # port: git clone + optional history reset
│   └── git-digest/       # new: scan repos → log → author filter → MD
```

### §2.1 Core crate API

```
scan_repos(dir: &Path) → Vec<RepoPath>
git_log(repo: &Path, since: DateTime, until: DateTime) → Vec<Commit>

Commit { hash, email, name, subject, date, repo }

TermOutput::print_digest(groups: &AuthorGroups)
MarkdownWriter::write(groups: &AuthorGroups, path: &Path) → Result<()>
Prompt::date_range() → (DateTime, DateTime)
Prompt::select_authors(authors: &[AuthorSummary]) → Vec<String>
```

### §2.2 Deps

| crate | usage |
|---|---|
| `clap` (derive) | arg parsing |
| `inquire` | interactive wizard |
| `walkdir` | recursive dir scan |
| `chrono` | date range / parsing |
| `colored` | terminal color output |
| `std::process::Command` | git invocations (no git2) |

## §3 git-digest Command

### §3.1 Interface

```
api: ttk git-digest <dir> → scan + interactive wizard
api: ttk git-digest <dir> --last 24h|7d|30d → skip wizard date step
api: ttk git-digest <dir> --since DATE [--until DATE] → explicit range
api: ttk git-digest <dir> --output <path> → override MD output dir
```

`DATE` format: ISO 8601 (`2026-05-01`).
`--since` solo → `--until` defaults to now.
`--until` solo → `--since` defaults to 30d ago.

### §3.2 Execution Flow

```
Step 1 — SCAN
  walkdir(<dir>, follow_links=true) → dirs containing .git/
  ! skip .git/ internals (min_depth avoids recursing inside)
  log: "Found N repositories"

Step 2 — LOG
  ∀ repo → git log --format="%H|%ae|%an|%s|%ad" --date=iso
           --after=<since> --before=<until>
  accumulate: HashMap<AuthorEmail, Vec<Commit>>

Step 3 — DATE (wizard only, no flags)
  inquire::Select:
    > Last 24 hours
      Last 7 days
      Last 30 days
      Custom range → since + until prompts

Step 4 — AUTHORS
  inquire::MultiSelect:
    "arielonoriaga (14 commits, 3 repos)"
    ! ≥1 author required; re-prompt if 0 selected

Step 5 — RENDER
  terminal: colored, grouped by author → repo → commits
  file: git-digest-YYYY-MM-DD.md (in <dir> | --output)
```

### §3.3 Markdown Output

```markdown
# Git Digest — 2026-05-28

**Period:** 2026-05-27 → 2026-05-28
**Repos scanned:** 12 | **With activity:** 4

---

## arielonoriaga

### `~/projects/term-toolkit`
| hash | message | date |
|------|---------|------|
| `abc1234` | feat: scaffold rust workspace | 2026-05-28 10:32 |

### `~/projects/my-app`
| hash | message | date |
|------|---------|------|
| `aaa1111` | chore: update deps | 2026-05-27 18:44 |
```

## §4 Ported Commands

| cmd | TS source | Rust behavior |
|---|---|---|
| `ttk deleter <dir>` | `scripts/deleter.ts` | delete files by even/odd index |
| `ttk rename <newName>` | `scripts/rename-sequence.ts` | rename files sequentially |
| `ttk optimize <dir>` | `scripts/optimizer.ts` | compress images (quality, output) |
| `ttk clone-repo <url>` | `scripts/git.ts` | clone + optional history reset |

## §5 Error Handling

### Input errors → exit 1

```
V1: <dir> ⊥ exist → "error: directory not found: /foo"
V2: --last invalid value → "error: invalid duration '3x', use 24h|7d|30d"
V3: --since > --until → "error: --since must be before --until"
```

### Runtime warnings → skip + continue

```
V4: .git/ found but repo corrupt → ⚠ skipped /foo (not a valid git repo)
V5: git log fails (permissions) → ⚠ skipped /foo (git error: <msg>)
V6: MD file exists → ⚠ overwriting git-digest-2026-05-28.md
```

### Empty states → exit 0

```
V7: 0 repos found → "No git repositories found in /foo"
V8: repos found, 0 commits in range → "No commits found in the selected period"
V9: user deselects all authors → re-prompt "Select at least one author"
```

### Filesystem

```
V10: --output dir ⊥ exist → create automatically
V11: symlink loop → walkdir detects + skips
V12: >100 repos → show progress bar during scan
```

## §6 Invariants

```
I1: ∀ git invocation → via std::process::Command, not git2
I2: ∀ error in one repo → skip repo, never abort full scan
I3: MD output ⊥ written if 0 commits after author filter
I4: terminal output always before MD write
I5: --last / --since / --until present → wizard date step skipped
```

## §7 Out of Scope

- `--format` flag (JSON, CSV) — future
- GitHub/GitLab API integration — future
- Diff stats per commit (`--stat`) — future
- npm publish of Rust binary — future
