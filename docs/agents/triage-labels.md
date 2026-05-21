# Triage Labels

The skills speak in terms of canonical triage roles. This file maps those roles to the actual label strings used in this repo's issue tracker.

## Label vocabulary

| Role                      | Label in our tracker   | Meaning                                                      |
| ------------------------- | ---------------------- | ------------------------------------------------------------ |
| `needs-triage`            | `needs-triage`         | Maintainer needs to evaluate this issue                      |
| `needs-info`              | `needs-info`           | Waiting on reporter for more information                     |
| `ready-for-agent`         | `ready-for-agent`      | Fully specified, ready for an implementer agent              |
| `in-progress-impl`        | `in-progress-impl`     | Implementer has claimed this issue (claim lock)              |
| `ready-for-review`        | `ready-for-review`     | Implementation done, ready for reviewer agent                |
| `in-progress-review`      | `in-progress-review`   | Reviewer has claimed this issue (claim lock)                 |
| `needs-rework`            | `needs-rework`         | Reviewer found defects; implementer should pick up (priority) |
| `ready-for-human`         | `ready-for-human`      | Reviewed and approved; human sign-off needed                 |
| `wontfix`                 | `wontfix`              | Will not be actioned                                         |

## State machine

```
needs-triage → ready-for-agent → in-progress-impl → ready-for-review → in-progress-review → ready-for-human
                                                                          ↓ (defects)
                                                                      needs-rework → in-progress-impl (loop back)
```

- Implementer polls: `needs-rework` first (priority), then `ready-for-agent`
- Reviewer polls: `ready-for-review`
- Claim mechanism: remove source label, add `in-progress-*` atomically via `gh issue edit`

When a skill mentions a triage role, use the corresponding label string from this table.
