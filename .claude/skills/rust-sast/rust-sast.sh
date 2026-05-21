#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
DIFF_BASE="main"
PARENT_ISSUE=""

for arg in "$@"; do
    case $arg in
        --diff-base=*) DIFF_BASE="${arg#*=}" ;;
        --parent-issue=*) PARENT_ISSUE="${arg#*=}" ;;
        *) echo "Unknown arg: $arg" >&2; exit 1 ;;
    esac
done

RULES="$SCRIPT_DIR/.semgrep.yml"
SEMGREP_ARGS="--config=p/rust"
if [ -f "$RULES" ]; then
    SEMGREP_ARGS="$SEMGREP_ARGS --config=$RULES"
fi

if ! command -v semgrep &>/dev/null; then
    jq -n --arg msg "semgrep not found. Install: paru -S semgrep (or pip install semgrep)" \
        '{status:"error",summary:{errors:1,warnings:0,infos:0},results:{},message:$msg}'
    exit 0
fi

CHANGED_RS=$({
    git diff --name-only "$DIFF_BASE...HEAD" 2>/dev/null
    git diff --name-only "$DIFF_BASE" 2>/dev/null
    git diff --cached --name-only "$DIFF_BASE" 2>/dev/null
    git ls-files --others --exclude-standard 2>/dev/null
} | grep '\.rs$' | head -1)

if [ -z "$CHANGED_RS" ]; then
    echo '{"status":"skipped","summary":{"errors":0,"warnings":0,"infos":0},"results":{}}'
    exit 0
fi

SEMGREP_JSON=$(semgrep --json --quiet $SEMGREP_ARGS . 2>/dev/null || true)

ERRORS=$(echo "$SEMGREP_JSON" | jq -r '.results[]? | select(.extra.severity == "ERROR") | .extra.severity' 2>/dev/null | wc -l)
WARNINGS=$(echo "$SEMGREP_JSON" | jq -r '.results[]? | select(.extra.severity == "WARNING") | .extra.severity' 2>/dev/null | wc -l)
INFOS=$(echo "$SEMGREP_JSON" | jq -r '.results[]? | select(.extra.severity == "INFO") | .extra.severity' 2>/dev/null | wc -l)

if [ "$ERRORS" -gt 0 ] || [ "$WARNINGS" -gt 0 ] || [ "$INFOS" -gt 0 ]; then
    STATUS="findings"
else
    STATUS="clean"
fi

echo "$SEMGREP_JSON" | jq --arg status "$STATUS" \
    --argjson summary "{\"errors\":$ERRORS,\"warnings\":$WARNINGS,\"infos\":$INFOS}" \
    '{status:$status,summary:$summary,results:.}'
