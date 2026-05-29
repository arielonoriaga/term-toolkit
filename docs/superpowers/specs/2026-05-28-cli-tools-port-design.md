---
name: cli-tools-port
description: Port 5 bash tools from arielonoriaga/cli into ttk as Rust subcommands
metadata:
  type: project
---

# CLI Tools Port — arielonoriaga/cli → ttk

**Date:** 2026-05-28
**Source:** https://github.com/arielonoriaga/cli

## §1 Goal

Port 5 bash tools as native Rust subcommands of `ttk`.
No behavior regressions vs bash originals.
ffmpeg tools call ffmpeg via `std::process::Command` (no native encode).

## §2 Architecture

5 new crates added to Cargo workspace. `copy_clean_dir` helper extracted to `ttk-core` (shared by `copy-clean` and `build-and-sign`).

```
crates/
├── build-and-sign/   # ttk build-and-sign <source> <prefix>
├── copy-clean/       # ttk copy-clean <source> <dest>
├── mp3-compress/     # ttk mp3-compress [input] [output-dir]
├── mp4-optimize/     # ttk mp4-optimize [input] [output-dir] [--quality]
└── sign/             # ttk sign <folder> <prefix>
```

### §2.1 New deps

| crate | deps |
|---|---|
| `ttk-sign` | `md5 = "0.10"`, `sha1 = "0.10"`, `walkdir` (via ttk-core) |
| `ttk-build-and-sign` | same + `zip = "2"` |
| `ttk-copy-clean` | `ttk-core` |
| `ttk-mp3-compress` | stdlib only |
| `ttk-mp4-optimize` | stdlib only |

### §2.2 `ttk-core` addition

```
pub fn copy_clean_dir(src: &Path, dest: &Path, skip: &[&str]) -> Result<(), String>
```

Walks `src` with walkdir. Skips entries whose name matches any item in `skip` (exact name match for dirs; glob `*.ext` for files). Recreates directory structure under `dest`, copies all non-skipped files.

Default skip list used by both callers: `["node_modules", ".git", ".github", "dist"]` + file extensions `["md5", "sha1", "zip"]`.

## §3 Tool Specs

### §3.1 `ttk sign <folder> <prefix>`

**Interface:**
```
cmd: ttk sign <folder> <prefix>
```

**Behavior:**
```
V1: folder ⊥ exist → Err "folder not found: /foo"
walkdir(folder) → ∀ file → md5(bytes) + sha1(bytes)
output format: "{hash}  {path}\n"  (2 spaces — compatible w/ md5sum/sha1sum)
writes: {prefix}.md5, {prefix}.sha1
```

**Deps:** `md5 = "0.10"`, `sha1 = "0.10"`, `walkdir`

---

### §3.2 `ttk copy-clean <source> <dest>`

**Interface:**
```
cmd: ttk copy-clean <source> <dest>
```

**Behavior:**
```
V1: source ⊥ exist → Err "source not found: /foo"
V2: dest already exists → Err "destination already exists: /foo"
copy_clean_dir(source, dest, DEFAULT_SKIP)
```

DEFAULT_SKIP dirs: `node_modules`, `.git`, `.github`, `dist`
DEFAULT_SKIP file exts: `md5`, `sha1`, `zip`

---

### §3.3 `ttk build-and-sign <source> <prefix>`

**Interface:**
```
cmd: ttk build-and-sign <source> <prefix>
```

**Behavior:**
```
V1: source ⊥ exist → Err "source not found: /foo"
clean_dir = "{prefix}-clean"
V2: clean_dir already exists → Err "{prefix}-clean already exists"

Step 1: copy_clean_dir(source, clean_dir, DEFAULT_SKIP)
Step 2: walkdir(clean_dir) → ∀ file → md5+sha1 → "{prefix}.md5" / "{prefix}.sha1"
        format: "{hash}  {path}\n"
Step 3: zip clean_dir → "{prefix}.zip"  (preserves dir structure)
Step 4: rm -rf clean_dir
```

Output: `{prefix}.zip`, `{prefix}.md5`, `{prefix}.sha1`

---

### §3.4 `ttk mp3-compress [input] [output-dir]`

**Interface:**
```
cmd: ttk mp3-compress [input] [output-dir]
```

**Behavior:**
```
input default: "." (current dir)
output default: same dir as input file | input dir

V1: ffmpeg ⊥ in PATH → Err "ffmpeg not found — install it first"
V2: input ⊥ exist → Err "not found: /foo"
V3: 0 mp3 files found → Err "no MP3 files found in /foo"

file mode  (input = *.mp3):
  ffmpeg -i <f> -ac 1 -c:a aac -b:a 96k <out>/{stem}.m4a

dir mode   (input = dir):
  ∀ *.mp3 in dir (non-recursive) → same ffmpeg call
  counts converted files, reports total
```

**ffmpeg check:** `Command::new("ffmpeg").arg("-version").output().is_ok()`

---

### §3.5 `ttk mp4-optimize [input] [output-dir] [--quality high|medium|low|web]`

**Interface:**
```
cmd: ttk mp4-optimize [input] [output-dir] --quality high|medium|low|web
```

**quality default:** `web`

**Quality presets:**

| preset | CRF | max res | video bitrate | audio |
|---|---|---|---|---|
| `high` | 18 | 1920×1080 | 5000k | 192k |
| `medium` | 23 | 1280×720 | 2500k | 128k |
| `low` | 28 | 854×480 | 1000k | 96k |
| `web` | 25 | 1280×720 | 1500k | 128k |

**Behavior:**
```
V1: ffmpeg ⊥ in PATH → Err "ffmpeg not found"
V2: input ⊥ exist → Err "not found: /foo"
V3: 0 mp4 files found → Err "no MP4 files found in /foo"
V4: invalid quality value → Err "invalid quality 'x', use high|medium|low|web"

file mode (input = *.mp4):
  output: {out_dir}/{stem}_optimized.mp4
  ffmpeg -i <in> -c:v libx264 -profile:v baseline -level:v 3.0
         -crf <CRF> -maxrate <BITRATE> -bufsize <2×BITRATE>
         -vf "scale='min(W,iw)':'min(H,ih)':force_original_aspect_ratio=decrease,fps=30"
         -pix_fmt yuv420p -c:a aac -b:a <AUDIO> -ac 2 -ar 44100
         -movflags +faststart -preset slow -tune film -y <out>

dir mode (input = dir):
  ∀ *.mp4 in dir (non-recursive) → same ffmpeg call
  after each: show size reduction % (original → optimized)
  final summary: N/M files succeeded
```

---

## §4 Error Handling

```
∀ tool: missing required arg → clap error (auto)
∀ tool: V1 missing ffmpeg → clear message before any work
∀ tool: individual file failure → warn + continue (dir mode only)
∀ tool: all output written atomically where possible (sign/build-and-sign write to final path directly)
```

## §5 Invariants

```
I1: ∀ ffmpeg invocation → via std::process::Command, stderr forwarded
I2: checksum format compatible w/ md5sum -c / sha1sum -c
I3: copy_clean_dir ⊥ follow symlinks (avoid loops)
I4: mp3/mp4 dir mode → non-recursive (top-level files only, matches bash original)
I5: build-and-sign step 4 (rm clean_dir) runs even if zip fails → no temp leak
```

## §6 Out of Scope

- Native Rust audio/video encode (use ffmpeg)
- Recursive mp3/mp4 directory scanning
- Progress bars for ffmpeg (stderr forwarded as-is)
- SHA256/SHA512 checksums
