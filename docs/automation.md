# Automation Pipeline

Two-agent autonomous development pipeline with human-in-the-loop sign-off.

## Architecture

```
Human (Terminal 3)                Issue Tracker (integration point)
├─ /grill-with-docs               ├─ ready-for-agent
├─ /to-prd                        ├─ in-progress-impl
├─ /to-issues                     ├─ ready-for-review
├─ reviews ready-for-human         ├─ in-progress-review
│                                  ├─ needs-rework
Terminal 1          Terminal 2    └─ ready-for-human
Implementer         Reviewer
/loop 10m           /loop 10m
/implement-issue    /review-issue
```

## State machine

```
ready-for-agent → in-progress-impl → ready-for-review → in-progress-review → ready-for-human
                                                  ↓ (defects found)
                                              needs-rework ──┐
                                                  (defect issues block parent, implementer fixes them first)
```

## Agent: Implementer (Terminal 1)

Skill: `/implement-issue`

Flow per loop iteration:
1. Poll `needs-rework` (priority), then `ready-for-agent`
2. Claim via `gh issue edit` (swap label to `in-progress-impl`) — atomic, prevents race conditions
3. Read issue body for acceptance criteria
4. Run `/tdd` — red-green-refactor on vertical slice
5. Push branch `issue-<NUMBER>`, create PR, comment PR link on issue
6. Set label to `ready-for-review`

## Agent: Reviewer (Terminal 2)

Skill: `/review-issue`

Flow per loop iteration:
1. Poll `ready-for-review`
2. Claim via `gh issue edit` (swap label to `in-progress-review`)
3. Read issue + checkout PR branch
4. `/code-review` for defects and security
5. Validate TDD/E2E tests, run full test suite
6. Security testing on build
7. **All clear** → label `ready-for-human`
8. **Defects** → create separate `ready-for-agent` defect issues, link as blocking parent, set parent to `needs-rework`

## Human (Terminal 3)

Flow:
1. `/grill-with-docs` — design session, sharpen domain language, update CONTEXT.md
2. Optional `/to-prd` — publish PRD to issue tracker
3. `/to-issues` — break plan into vertical slices, creates issues with `ready-for-agent`
4. Monitor `ready-for-human` — approve (merge), request changes (`needs-rework`), or reject (`wontfix`)

## Starting the pipeline

```bash
# Terminal 1
cd /home/mike/Documents/projects/osr-client && claude
> /loop 10m /implement-issue

# Terminal 2
cd /home/mike/Documents/projects/osr-client && claude
> /loop 10m /review-issue

# Terminal 3 (human)
cd /home/mike/Documents/projects/osr-client && claude
> /grill-with-docs   # then /to-prd, /to-issues as needed
> # manually review ready-for-human issues
```

## Scaling

Both agents use label-based claim locks (`in-progress-impl` / `in-progress-review`) that prevent race conditions. Multiple implementers and reviewers can run in parallel — the claim mechanism handles contention.

## Triage labels

See `docs/agents/triage-labels.md` for the full label vocabulary and state machine reference.
