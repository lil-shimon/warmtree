use anyhow::Result;
use std::process::Command;

#[derive(Debug, Clone, PartialEq)]
pub struct Worktree {
    pub path: String,
    pub branch: String,
    pub commit: String,
}

pub fn list_worktrees() -> Result<Vec<Worktree>> {
    let output = execute_git_command(&["worktree", "list", "--porcelain"])?;
    parse_worktree_output(&output)
}

pub fn create_worktree(path: &str, branch: &str) -> Result<()> {
    execute_git_command(&["worktree", "add", path, branch])?;
    Ok(())
}

fn execute_git_command(args: &[&str]) -> Result<String> {
    let output = Command::new("git").args(args).output()?;
    validate_command_success(&output)?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn validate_command_success(output: &std::process::Output) -> Result<()> {
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Git command failed: {}", error));
    }
    Ok(())
}

fn parse_worktree_output(output: &str) -> Result<Vec<Worktree>> {
    let mut worktrees = Vec::new();
    let lines: Vec<&str> = output.lines().collect();
    
    for chunk in lines.chunks(4) {
        if let Some(worktree) = parse_worktree_chunk(chunk) {
            worktrees.push(worktree);
        }
    }
    
    Ok(worktrees)
}

fn parse_worktree_chunk(chunk: &[&str]) -> Option<Worktree> {
    if chunk.is_empty() || !chunk[0].starts_with("worktree ") {
        return None;
    }
    
    let path = extract_worktree_path(chunk[0])?;
    let commit = extract_commit_hash(chunk)?;
    let branch = extract_branch_name(chunk)?;
    
    Some(Worktree { path, branch, commit })
}

fn extract_worktree_path(line: &str) -> Option<String> {
    if line.len() > 9 {
        Some(line[9..].to_string())
    } else {
        None
    }
}

fn extract_commit_hash(chunk: &[&str]) -> Option<String> {
    chunk.iter()
        .find(|line| line.starts_with("HEAD "))
        .and_then(|line| {
            if line.len() > 5 {
                Some(line[5..].to_string())
            } else {
                None
            }
        })
}

fn extract_branch_name(chunk: &[&str]) -> Option<String> {
    chunk.iter()
        .find(|line| line.starts_with("branch "))
        .and_then(|line| {
            if line.len() > 7 {
                Some(line[7..].to_string())
            } else {
                None
            }
        })
        .or_else(|| Some("detached".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_worktree_output_single() {
        let output = "worktree /path/to/repo\nHEAD abc123\nbranch refs/heads/main\n\n";
        let result = parse_worktree_output(output).unwrap();
        
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].path, "/path/to/repo");
        assert_eq!(result[0].branch, "refs/heads/main");
        assert_eq!(result[0].commit, "abc123");
    }

    #[test]
    fn test_parse_worktree_output_multiple() {
        let output = "worktree /repo\nHEAD abc123\nbranch refs/heads/main\n\nworktree /repo/feature\nHEAD def456\nbranch refs/heads/feature\n\n";
        let result = parse_worktree_output(output).unwrap();
        
        assert_eq!(result.len(), 2);
        assert_eq!(result[1].path, "/repo/feature");
        assert_eq!(result[1].branch, "refs/heads/feature");
    }

    #[test]
    fn test_extract_worktree_path() {
        let result = extract_worktree_path("worktree /test/path");
        assert_eq!(result, Some("/test/path".to_string()));
    }

    #[test]
    fn test_extract_commit_hash() {
        let chunk = &["worktree /path", "HEAD abc123", "branch main"];
        let result = extract_commit_hash(chunk);
        assert_eq!(result, Some("abc123".to_string()));
    }

    #[test]
    fn test_extract_branch_name() {
        let chunk = &["worktree /path", "HEAD abc123", "branch refs/heads/main"];
        let result = extract_branch_name(chunk);
        assert_eq!(result, Some("refs/heads/main".to_string()));
    }
}