---
description: Verify and update documentation across all EVO-OS submodule repos
---

# Update Docs

This workflow checks that documentation is up to date across all EVO-OS repos and helps regenerate stale docs.

## Repos to Check

| Repo | Key docs |
|------|---------|
| Root | README.md, ARCHITECTURE.md, CONTRIBUTING.md, docs/BOOT_CHAIN.md, docs/ROADMAP.md |
| frimware | README.md, ARCHITECTURE.md, docs/DESIGN.md |
| bootloader | README.md, docs/STAGES.md |
| bsp | README.md, docs/PLATFORMS.md |
| Kernel | README.md, ARCHITECTURE.md, docs/MEMORY.md, docs/SYSCALLS.md |
| drivers | README.md, docs/DRIVER_MODEL.md |
| libc | README.md, docs/ABI.md |
| filesystem | README.md, docs/VFS.md |
| GUI | README.md, docs/COMPOSITOR.md |
| apps | README.md, docs/APP_MODEL.md |

## Steps

// turbo
1. List all README files across all repos and check their last modified dates:
```bash
find . -name "README.md" -not -path "./.git/*" \
  -exec ls -la {} \; | sort
```

// turbo
2. Check for any README files that are still placeholder stubs (< 200 bytes):
```bash
find . -name "README.md" -not -path "./.git/*" \
  -size -200c -exec echo "STUB: {}" \;
```

// turbo
3. Check for any documented `docs/` directory paths that don't exist yet:
```bash
find . -name "*.md" -not -path "./.git/*" \
  | xargs grep -l "\[.*\](docs/" \
  | while read f; do
      dir=$(dirname "$f")
      grep -oP '\(docs/[^)]+\)' "$f" | tr -d '()' | while read link; do
        [ ! -f "$dir/$link" ] && echo "MISSING: $dir/$link (referenced in $f)"
      done
    done
```

4. Check that every submodule has a CONTRIBUTING or reference to root CONTRIBUTING:
```bash
for dir in frimware bootloader bsp Kernel drivers libc filesystem GUI apps; do
  if [ ! -f "$dir/CONTRIBUTING.md" ]; then
    echo "No CONTRIBUTING.md in $dir (should reference ../CONTRIBUTING.md)"
  fi
done
```

5. Verify all code examples in READMEs use correct filenames (spot check):
```bash
# Check that boot stage filenames match what's actually in boot/
grep -rn "stage1\|stage2" bootloader/README.md
ls boot/
```

6. After updating any doc, verify it renders correctly (lint markdown):
```bash
# Install markdownlint if needed
which markdownlint || npm install -g markdownlint-cli

# Lint all markdown files
markdownlint '**/*.md' --ignore node_modules --ignore .git
```

## Adding Missing Docs

For each `MISSING:` entry found in step 3, create the missing file:

```bash
# Example: create missing Kernel/docs/MEMORY.md
mkdir -p Kernel/docs
cat > Kernel/docs/MEMORY.md << 'EOF'
# EVO-OS Kernel — Memory Management

[Details to be filled in]
EOF
```

## Checking for Outdated Roadmap Entries

After each milestone is completed, update `docs/ROADMAP.md`:
- Change `📋 Planned` → `🔄 In Progress` → `✅ Complete`

```bash
# Show all roadmap items not yet started
grep "📋" docs/ROADMAP.md
```
