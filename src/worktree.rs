use anyhow::Result;
use std::process::Command;

#[derive(Debug, Clone, PartialEq)]
pub struct Worktree {
    pub path: String,
    pub branch: String,
    pub commit: String,
}

pub fn list_worktrees() -> Result<Vec<Worktree>> {
    let output = Command::new("git")
        .args(["worktree", "list", "--porcelain"])
        .output()?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Git command failed: {}", error));
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_worktree_output(&stdout)
}

pub fn create_worktree(path: &str, branch: &str) -> Result<()> {
    let output = Command::new("git")
        .args(["worktree", "add", path, branch])
        .output()?;
    
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
        if chunk.is_empty() || !chunk[0].starts_with("worktree ") {
            continue;
        }
        
        let path = chunk[0][9..].to_string();
        let commit = chunk.iter()
            .find(|line| line.starts_with("HEAD "))
            .map(|line| line[5..].to_string())
            .unwrap_or_default();
        let branch = chunk.iter()
            .find(|line| line.starts_with("branch "))
            .map(|line| line[7..].to_string())
            .unwrap_or_else(|| "detached".to_string());
        
        worktrees.push(Worktree { path, branch, commit });
    }
    
    Ok(worktrees)
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
    fn test_parse_worktree_output_detached() {
        let output = "worktree /detached/path\nHEAD abc123\ndetached\n\n";
        let result = parse_worktree_output(output).unwrap();
        
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].path, "/detached/path");
        assert_eq!(result[0].branch, "detached");
        assert_eq!(result[0].commit, "abc123");
    }
}