---
name: implement-issue
description: Poll for and implement GitHub issues using TDD, with claim-lock to prevent race conditions. Use when implementing issues labeled ready-for-agent or needs-rework, or when running the implementer agent loop.
---

# Implement Issue

## Quick start

```
/implement-issue
```

Runs one iteration: poll → claim → implement → PR → label. Designed to be called via `/loop` for continuous operation.

## Workflow

### 1. Poll for issues

Check `needs-rework` first (higher priority), then `ready-for-agent`:

```
gh issue list --label "needs-rework" --json number,title --jq '.[].number'
gh issue list --label "ready-for-agent" --json number,title --jq '.[].number'
```

### 2. Claim atomically

For each issue (in priority order), attempt claim. If it fails, another agent claimed it — try the next:

```
gh issue edit <NUMBER> --remove-label "needs-rework" --add-label "in-progress-impl"
```

or

```
gh issue edit <NUMBER> --remove-label "ready-for-agent" --add-label "in-progress-impl"
```

Check exit code. Non-zero = claim failed, skip to next issue.

### 3. Read the issue

```
gh issue view <NUMBER> --json body,title,labels
```

Extract acceptance criteria and context.

### 4. Implement with TDD

Run `/tdd` explicitly — red-green-refactor loop. Implement a vertical slice through all layers.

### 5. Push and create PR

Create branch named `issue-<NUMBER>`, push, and create PR:

```
gh pr create --title "<issue-title>" --body "Related to #<NUMBER>" --base main
```

Comment the PR link on the issue:

```
gh issue comment <NUMBER> --body "PR: <pr-url>"
```

### 6. Update label

```
gh issue edit <NUMBER> --remove-label "in-progress-impl" --add-label "ready-for-review"
```

### 7. On failure

If implementation fails, reset:

```
gh issue edit <NUMBER> --remove-label "in-progress-impl" --add-label "ready-for-agent"
gh issue comment <NUMBER> --body "Implementation failed: <reason>"
```
