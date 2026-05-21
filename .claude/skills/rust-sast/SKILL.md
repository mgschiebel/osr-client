---
name: rust-sast
description: Run Semgrep SAST scan on Rust code in the workspace. Creates tracked issues from findings with severity-based routing. Callable by /review-issue or standalone.
---

# /rust-sast

Runs Semgrep on Rust source code and creates GitHub issues from findings.

## When to use

- `/review-issue` calls this when a PR touches `.rs` files
- User runs `/rust-sast` standalone for ad-hoc scanning

## Input

- `--diff-base <branch>` — branch to compare against for skip check (default: `main`)
- `--parent-issue <number>` — optional. If provided, ERROR issues link back as blockers

## Workflow

### 1. Check dependencies

Run `which semgrep`. If missing, return error JSON and stop. Install: `paru -S semgrep` (AUR) or `pip install semgrep`

### 2. Run the scan script

```
bash .claude/skills/rust-sast/rust-sast.sh --diff-base=<base> [--parent-issue=<N>]
```

Script outputs JSON to stdout:
```json
{
  "status": "skipped | clean | findings",
  "summary": {"errors": N, "warnings": N, "infos": N},
  "results": { <semgrep --json output> }
}
```

Script always exits 0. Status meanings:
- `skipped` — no `.rs` files changed vs diff-base, nothing scanned
- `clean` — scanned, 0 findings
- `findings` — scanned, 1+ findings

### 3. Handle skipped / clean

If status is `skipped` or `clean`, report to user and stop. No issues to create.

### 4. Parse findings from results

Extract from `results.results[]` array. Each finding has:
- `extra.severity`: "ERROR" | "WARNING" | "INFO"
- `path`: file path
- `start.line`: line number
- `extra.message`: description
- `extra.rule_id`: rule identifier

### 5. Create issues by severity

#### ERROR findings — one issue per finding

For each ERROR:
- Label: `ready-for-agent`
- Title: `[SAST ERROR] <rule_id> in <file>`
- Body: include file path, line number, message, severity, full finding JSON
- If `--parent-issue` was provided: add parent issue number as a blocker link in the body, and use `gh issue edit` to add a `blockedBy` relationship (create the issue first, then link)

#### WARNING findings — one aggregated issue

If 1+ WARNINGs:
- Label: `ready-for-human` + `sast-finding`
- Title: `[SAST WARNINGS] Semgrep found N warnings`
- Body: table listing all WARNING findings (file, line, rule, message)
- If `--parent-issue` provided: mention parent in body (does NOT block)

#### INFO findings — one aggregated issue

If 1+ INFOs:
- Label: `ready-for-human` + `sast-finding`
- Title: `[SAST INFO] Semgrep found N info findings`
- Body: table listing all INFO findings (file, line, rule, message)
- If `--parent-issue` provided: mention parent in body (does NOT block)

### 6. Report to caller

Summarize: status, counts by severity, issue URLs created. If called by `/review-issue`, return this summary so it can decide whether to block the parent issue.

## Rules

- Official: `r2c/rust-security` (always runs)
- Custom: `.claude/skills/rust-sast/.semgrep.yml` (if present)
- Script auto-discovers its own rules path via `$0`
