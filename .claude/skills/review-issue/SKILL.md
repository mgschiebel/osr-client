---
name: review-issue
description: Poll for ready-for-review issues and cross-verify implementation via code review, test validation, and security testing. Use when reviewing PRs/issues labeled ready-for-review, or when running the reviewer agent loop.
---

# Review Issue

## Quick start

```
/review-issue
```

Runs one iteration: poll → claim → review → test → label. Designed to be called via `/loop` for continuous operation.

## Workflow

### 1. Poll for issues

```
gh issue list --label "ready-for-review" --json number,title --jq '.[].number'
```

### 2. Claim atomically

For each issue, attempt claim. If it fails, another reviewer claimed it — try the next:

```
gh issue edit <NUMBER> --remove-label "ready-for-review" --add-label "in-progress-review"
```

Check exit code. Non-zero = claim failed, skip to next issue.

### 3. Read the issue and PR

```
gh issue view <NUMBER> --json body,title,comments
gh pr list --search "issue:<NUMBER>" --json number,url,headRefName,body
```

Checkout the PR branch:

```
gh pr checkout <PR_NUMBER>
```

### 4. Code review

Run `/code-review` on the PR diff to scan for defects and security issues.

### 5. Test coverage and quality

- Review TDD tests: verify red-green-refactor discipline was followed
- Review E2E tests: check coverage and quality
- Run the full test suite on the PR branch
- Report any missing or weak tests

### 6. Rust SAST scan

Run `/rust-sast --parent-issue=<NUMBER>` (skill handles skip/scan/issue creation).

If status is `findings` and ERRORs were found: record that security blocks. The skill creates `ready-for-agent` issues per ERROR (linked as blockers) and `ready-for-human` + `sast-finding` aggregated issues for WARNINGs/INFOs.

### 7. Verdict

**All clear (no defects, no security blocks):**
```
gh issue edit <NUMBER> --remove-label "in-progress-review" --add-label "ready-for-human"
gh pr comment <PR_NUMBER> --body "Reviewer approval: all checks passed."
```

**Defects found (code review or test failures):**

For each defect, create a separate issue:
```
gh issue create --title "Defect: <summary>" --body "## Parent\n#<NUMBER>\n\n## Defect\n<detailed description>" --label "ready-for-agent"
```

**Security blocks (rust-sast ERROR findings):**

The `/rust-sast` skill already created `ready-for-agent` issues linked as blockers. Just update the parent:
```
gh issue edit <NUMBER> --remove-label "in-progress-review" --add-label "needs-rework"
gh issue comment <NUMBER> --body "Security issues found — see blocking issues from /rust-sast scan."
```

**Defects + security blocks combined:**
```
gh issue edit <NUMBER> --remove-label "in-progress-review" --add-label "needs-rework"
gh issue comment <NUMBER> --body "Defects and security issues found — see blocking issues."
```
