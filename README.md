# Resume_TUI

A terminal-based resume/portfolio viewer, built with [Rust](https://www.rust-lang.org/) and [ratatui](https://ratatui.rs/). Your resume lives in a single `resume.toml` file and renders as a navigable, section-by-section terminal UI — no browser, no PDF, just a clean TUI.

```
┌ Name ──────────────────────────────────────────────────────────────────┐
│                          Your Name Here                                │
└──────────────────────────────────────────────────────────────────────┘
┌ Profile ─────────────────────┐┌ Contact ────────────────────────────┐
│ Short professional summary…  ││ Email: you@example.com               │
│                               ││ GitHub: github.com/you               │
│                               ││ LinkedIn: linkedin.com/in/you        │
└───────────────────────────────┘└───────────────────────────────────┘
[← →] Sections  [↑ ↓] Scroll  [q/Esc] Quit
```

## Features

- **Config-driven** — the entire resume is defined in `resume.toml`; no recompiling to update content.
- **Adaptive sections** — sections only appear if there's data for them. `Personal`, `Skills`, and `Projects` are always shown; `Education`, `Experience`, `Certifications`, `Competitions`, and `Activities` only render if the corresponding TOML data is present.
- **Adaptive Skills grid** — the Skills panel automatically switches between a 2×2 layout (4 cards) and a 2×3 layout (6 cards) depending on whether the optional `networking` / `security` categories are filled in.
- **Keyboard navigation** — arrow keys, WASD, or Vim-style `hjkl`, all supported.
- **Scrollable, paginated cards** — long lists (experience, projects, competitions, etc.) scroll and paginate to fit the terminal.
- **Panic-safe terminal restore** — a `Drop` guard restores the terminal (raw mode, alternate screen) even if the app panics mid-run.
- **Small, containerized footprint** — multi-stage Docker build on top of `debian:bookworm-slim`.

## Controls

| Key(s)              | Action              |
|----------------------|---------------------|
| `↑` / `w` / `k`       | Scroll up           |
| `↓` / `s` / `j`       | Scroll down         |
| `→` / `d` / `l`       | Next section        |
| `←` / `a` / `h`       | Previous section    |
| `q` / `Esc`           | Quit                |

## Getting started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)

### Run locally

```bash
git clone https://github.com/H5RV948/Resume_TUI.git
cd Resume_TUI
cp resume.example.toml resume.toml   # then edit with your own info
cargo run --release
```

### Run with Docker

```bash
docker build -t resume-tui .
docker run -it --rm resume-tui
```

> The Dockerfile expects a `resume.toml` in the build context — it's baked into the image at build time.

## Configuring your resume

All content is defined in `resume.toml`. Fields marked *optional* can be omitted entirely.

```toml
[personal]
name = "Your Name"
email = "you@example.com"
github = "https://github.com/you"
linkedin = "https://linkedin.com/in/you"
profile = "A short professional summary."   # optional
phone_number = "+52 000 000 0000"           # optional
location = "City, Country"                  # optional

[education]                                  # optional section
institution = "University Name"
degree = "B.S. in Computer Science"
gpa = "9.5/10"                               # optional
expected_graduation = "June 2029"            # optional
distinctions = ["Dean's list", "Academic scholarship"]
relevant_courses = ["Data Structures", "Networks"]  # optional

[skills]
tools = ["Splunk", "Metasploit", "Nmap", "Mimikatz", "cURL", "Cisco Packet Tracer"]
systems = ["Linux", "Windows", "Proxmox"]
languages = ["Python", "C++", "Go", "Rust", "SQL", "Bash"]
spoken = ["English (Fluent)", "Spanish (Native)", "German (Beginner)"]
networking = ["TCP/IP", "VPNs"]              # optional — enables the 6-panel Skills layout
security = ["AAA", "CIA Triad", "VPNs", "Footprinting"]  # optional — enables the 6-panel Skills layout

[[experience]]                                # optional section, repeatable
role = "Role title"
organization = "Company"
period = "Jun 2025 – Aug 2025"
bullets = ["Did a thing", "Did another thing"]

[[projects]]                                  # repeatable
name = "Project Name"
date = "2026"
tags = ["Rust", "Systems"]
description = "One-line description."        # optional
bullets = ["Built X", "Improved Y"]

[[certifications]]                            # optional section, repeatable
name = "Certification Name"
issuer = "Issuing Body"
date = "2026"

[[competitions]]                              # optional section, repeatable
name = "Competition Name"
team_name = "Team Name"
date = "2026"
tags = ["CTF", "Security"]                    # optional
bullets = ["Placed Nth", "Solved X challenges"]

[[activities]]                                # optional section, repeatable
name = "Activity Name"
tag = "Club / Organization"
date = "2026"
bullets = ["Contributed to X"]
```

## Project structure

```
.
├── src/
│   ├── main.rs      # entry point, terminal setup/teardown, main loop
│   ├── app.rs        # App state, active sections, input handling
│   ├── event.rs       # keyboard/resize event → Action mapping
│   ├── ui.rs           # all rendering logic (ratatui widgets/layouts)
│   └── resume.rs        # resume.toml schema (serde) + loader
├── resume.toml            # your resume data (not committed — see below)
├── Dockerfile
├── Cargo.toml
└── Cargo.lock
```

## Tech stack

- [ratatui](https://ratatui.rs/) — terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) — cross-platform terminal backend
- [serde](https://serde.rs/) + [toml](https://docs.rs/toml) — resume config parsing
- [anyhow](https://docs.rs/anyhow) — error handling

## Notes

- `resume.toml` contains personal information and is intended to be kept out of version control (or replaced with placeholder/example data) if you fork this for your own resume — see `resume.example.toml` for a template to copy.
- The 2×2 / 2×3 Skills layout switch is driven by whether `networking` and `security` are present (`Some`) in the parsed TOML, not just whether they're non-empty — see `ui.rs::render` for the `has_extra` check.
