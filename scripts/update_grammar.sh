#!/usr/bin/env bash
set -euo pipefail

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TARGET_DIR="$REPO_DIR/languages/c3"
REPO_URL="https://github.com/c3lang/tree-sitter-c3.git"
RAW_URL="https://raw.githubusercontent.com/c3lang/tree-sitter-c3"

echo "Fetching latest tree-sitter-c3 commit SHA..."
LATEST_SHA="$(git ls-remote "$REPO_URL" refs/heads/main | cut -f1)"
echo "Pinned commit: $LATEST_SHA"

# 1. Update rev in extension.toml
sed -i -E "s/rev = \"[a-f0-9]+\"/rev = \"$LATEST_SHA\"/" "$REPO_DIR/extension.toml"

# 2. Download highlights.scm
echo "Downloading queries..."
mkdir -p "$TARGET_DIR"
curl -fsSL "$RAW_URL/$LATEST_SHA/queries/highlights.scm" -o "$TARGET_DIR/highlights.scm"

# 3. Optional injections.scm
curl -fsSL "$RAW_URL/$LATEST_SHA/queries/injections.scm" -o "$TARGET_DIR/injections.scm" 2>/dev/null || rm -f "$TARGET_DIR/injections.scm"

# 4. Download and patch indents.scm
curl -fsSL "$RAW_URL/$LATEST_SHA/queries/indents.scm" -o "$TARGET_DIR/indents.scm"

sed -i -E \
  -e 's/^\(\[/\[/g' \
  -e '/\(#set! indent\./d' \
  -e 's/@indent\.begin/@indent/g' \
  -e 's/@indent\.end/@end/g' \
  -e 's/@indent\.branch/@outdent/g' \
  -e 's/@indent\.align/@indent/g' \
  -e 's/@indent\.(auto|ignore)/@_indent.\1/g' \
  "$TARGET_DIR/indents.scm"

echo "Done."