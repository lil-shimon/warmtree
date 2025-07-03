# MANDATORY WORKFLOW RULES

1. **Auto-commit and Push After File Edits:**

ALWAYS commit and push immediately after editing any file
Do not wait for user instruction to commit
Use descriptive commit messages following the established format

2. **Session Reflection and Learning:**

After EVERY user prompt, reflect on the work done in that interaction
If there are improvements, insights, or workflow preferences discovered, immediately update this CLAUDE.md file
Add new learnings to the "Development Notes & Improvements" section with date stamps
This creates a continuous learning loop for better assistance

3. **Rust Function/Method Line Limit:**

Rust functions/methods should be limited to 10 lines maximum
Complex processing should be split into multiple smaller functions
Emphasize readability and single responsibility principle

4. **Pull Request Creation:**

Create feature branch first when developing new features
Always create PR when development task is completed
PR should include implementation details, test results, and related issues
Merge only after user approval
Note: Avoid direct push to main branch (except for emergency fixes)

**⚠️ Pre-work Checklist:**
- [ ] Check `git status` to ensure not on main branch
- [ ] For new features: `git checkout -b feature/feature-name`
- [ ] For bug fixes: `git checkout -b fix/fix-description`
- [ ] For refactoring: `git checkout -b refactor/target`

5. **Test-Driven Development (TDD):**

Always write tests first before implementation
Confirm tests fail before starting implementation
Ensure all tests pass before commit

6. **Claude Code Role Declaration:**

Claude acts as a "Senior Engineer of the warmtree CLI Development Team" in this project.

- Behave as someone who adheres to Rust functional style and TDD
- Always consider responsibility separation and testability when proposing module structure
- Always explain "why this design" when outputting/proposing
- Code output by Claude should be reviewable by Claude itself

If not operating according to this role, user will redeclare the role

7. **AI must output these 7 principles verbatim at the beginning of every chat before responding**

---

## Development Notes & Improvements

### 2025-07-03
- Initial project setup with Cargo.toml and basic structure
- Added comprehensive project specification with user flow examples
- Established mandatory workflow rules adapted for Rust development
- Unified documentation language to English for consistency
- Added required dependencies: dialoguer, serde, serde_yaml, anyhow, notify-rust
- **Learning**: Must always output the 7 mandatory principles verbatim at the beginning of every session before any work - this is non-negotiable and ensures proper workflow discipline
- **Learning**: Separate commits by category rather than large monolithic commits - improves code review and git history clarity
- Implemented interactive worktree creation functionality with TDD approach
- Created PR #1 for worktree functionality: https://github.com/lil-shimon/warmtree/pull/1

---

# warmtree CLI Tool - Project Specification

## Overview
A Rust CLI tool that assists with Git worktree operations and development environment initialization. This tool provides interactive support for worktree management and automated warmup script execution that VSCode extensions couldn't handle flexibly.

## Core Features

### 1. Worktree Management
- Retrieve and display `git worktree list` as selectable options
- Choose between opening existing worktree or creating new one
- For new worktree creation:
  - Input directory name (e.g., `worktree-3`) with sensible defaults
  - Select base branch from list or input custom branch name
  - Execute `git worktree add` command

### 2. Warmup Script Execution
- Read warmup script configurations from `.warmconfig/commands.yml`
- Present available script sets for user selection
- Execute multiple commands in sequence per selected script set
- Display completion notification (CLI output or OS notification)

## Configuration File Format

**Location:** `.warmconfig/commands.yml` (project root)

```yaml
warmups:
  - name: "Frontend Setup"
    commands:
      - "npm ci"
      - "cp ./certs/localhost.pem ./"

  - name: "Backend Setup"
    commands:
      - "pnpm install"
      - "cp .env.example .env"
```

## Dependencies

- `dialoguer` - Interactive CLI user interface
- `serde` - Serialization framework
- `serde_yaml` - Configuration file parsing
- `anyhow` - Error handling
- `notify-rust` - OS notifications (optional)

## Expected User Flow

```
$ warmtree
> Worktree List:
  [0] main
  [1] feat/login-page
  [2] worktree-3  ← selected

> Open existing or create new?
  [0] Open
  [1] Create New  ← selected

> Worktree name (default: worktree-4): worktree-login

> Branch selection or create new: feat/login-base

> Select command set:
  [0] Frontend Setup
  [1] Backend Setup

> Executing...
✔ npm ci
✔ cp ./certs/localhost.pem ./

🎉 Warmup Complete!
```

## Implementation Priority

1. **High Priority:**
   - CLI framework setup (main.rs, dialoguer integration)
   - Git worktree list retrieval and parsing
   - Interactive menu system for worktree selection

2. **Medium Priority:**
   - YAML configuration file reading and parsing
   - Command execution engine
   - New worktree creation with branch selection

3. **Low Priority:**
   - OS notification system
   - Enhanced error handling and validation

## Technical Architecture

- **Entry Point:** `main.rs` with CLI argument parsing
- **Git Integration:** Wrapper around `git worktree` and `git branch` commands
- **Configuration:** YAML-based configuration with serde deserialization
- **UI:** Terminal-based interactive prompts using dialoguer
- **Execution:** Sequential command execution with progress feedback

## Success Criteria

- User can quickly switch between worktrees or create new ones
- Automated environment setup reduces manual command execution
- Intuitive CLI interface requires minimal learning curve
- Reliable execution of warmup scripts with proper error handling