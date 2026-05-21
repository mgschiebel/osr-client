## Local Files

The following files are gitignored and must not be pushed to remote main:
- `.claude/`, `.agents/` — Claude Code session data and agent state
- `docs/adr/`, `docs/agents/` — local documentation
- `CLAUDE.md` — this file (project-level override, local only)
- `CONTEXT.md` — project context (local only)
- `skills-lock.json` — skill dependencies lock file (local only)

## Agent skills

### Issue tracker

GitHub Issues via `gh` CLI. See `docs/agents/issue-tracker.md`.

### Triage labels

Default vocabulary: needs-triage, needs-info, ready-for-agent, ready-for-human, wontfix. See `docs/agents/triage-labels.md`.

### Domain docs

Single-context: one CONTEXT.md + docs/adr/ at repo root. See `docs/agents/domain.md`.
