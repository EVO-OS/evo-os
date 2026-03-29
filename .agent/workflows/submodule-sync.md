---
description: Sync and update all git submodules to their latest commits
---

# Submodule Sync

This workflow synchronizes all EVO-OS git submodules.

## Steps

// turbo
1. Display current submodule status (shows which are ahead/behind):
```bash
git submodule status
```

// turbo
2. Update all submodules to their pinned commits (from `.gitmodules`):
```bash
git submodule update --init --recursive
```

3. Pull latest changes from each submodule's remote `main` branch:
```bash
git submodule foreach 'git fetch origin && git checkout main && git pull origin main'
```

4. Review what changed in each submodule:
```bash
git submodule foreach 'echo "=== $name ===" && git log --oneline -5'
```

5. If all looks good, stage the updated submodule references in the root repo:
```bash
git add .gitmodules
git submodule foreach 'cd .. && git add $name'
git status
```

6. Commit the updated submodule pins:
```bash
git commit -m "chore: update submodule references to latest main"
```

## Updating a Single Submodule

To update only one submodule (e.g., `Kernel`):

```bash
cd Kernel
git fetch origin
git checkout main
git pull origin main
cd ..
git add Kernel
git commit -m "chore(kernel): update to latest"
```

## Pinning a Submodule to a Specific Commit

```bash
cd Kernel
git checkout <commit-hash>
cd ..
git add Kernel
git commit -m "chore(kernel): pin to <commit-hash> for release v0.2.0"
```

## Troubleshooting

- **"HEAD detached"** → normal for submodules; run `git checkout main` inside the submodule
- **Conflicts in submodule** → resolve inside the submodule, then `cd ..` and `git add <submodule_dir>`
- **"fatal: repository not found"** → the GitHub remote URL may have changed; update `.gitmodules`
