use anyhow::Result;
use dialoguer::{Select, Input, Confirm};
use crate::worktree::{Worktree, list_worktrees, create_worktree};

pub fn run_interactive_worktree_menu() -> Result<()> {
    let worktrees = list_worktrees()?;
    display_worktree_menu(&worktrees)
}

fn display_worktree_menu(worktrees: &[Worktree]) -> Result<()> {
    let options = create_menu_options(worktrees);
    let selection = select_worktree_option(&options)?;
    
    match selection {
        0 => handle_create_new_worktree(),
        i if i <= worktrees.len() => handle_existing_worktree(&worktrees[i - 1]),
        _ => Ok(()),
    }
}

fn create_menu_options(worktrees: &[Worktree]) -> Vec<String> {
    let mut options = vec!["Create New Worktree".to_string()];
    
    for worktree in worktrees {
        let display_name = format_worktree_display(worktree);
        options.push(display_name);
    }
    
    options
}

fn format_worktree_display(worktree: &Worktree) -> String {
    let path = extract_directory_name(&worktree.path);
    let branch = format_branch_name(&worktree.branch);
    format!("{} ({})", path, branch)
}

fn extract_directory_name(path: &str) -> String {
    path.split('/')
        .last()
        .unwrap_or(path)
        .to_string()
}

fn format_branch_name(branch: &str) -> String {
    branch.strip_prefix("refs/heads/")
        .unwrap_or(branch)
        .to_string()
}

fn select_worktree_option(options: &[String]) -> Result<usize> {
    let selection = Select::new()
        .with_prompt("Select worktree option")
        .items(options)
        .interact()?;
    
    Ok(selection)
}

fn handle_create_new_worktree() -> Result<()> {
    let path = prompt_for_worktree_path()?;
    let branch = prompt_for_branch_name()?;
    
    confirm_and_create_worktree(&path, &branch)
}

fn prompt_for_worktree_path() -> Result<String> {
    let path: String = Input::new()
        .with_prompt("Enter worktree directory name")
        .interact_text()?;
    
    Ok(path)
}

fn prompt_for_branch_name() -> Result<String> {
    let branch: String = Input::new()
        .with_prompt("Enter branch name")
        .interact_text()?;
    
    Ok(branch)
}

fn confirm_and_create_worktree(path: &str, branch: &str) -> Result<()> {
    let confirmed = Confirm::new()
        .with_prompt(format!("Create worktree '{}' from branch '{}'?", path, branch))
        .interact()?;
    
    if confirmed {
        create_worktree(path, branch)?;
        println!("✅ Worktree created successfully!");
    }
    
    Ok(())
}

fn handle_existing_worktree(worktree: &Worktree) -> Result<()> {
    println!("Selected worktree: {}", worktree.path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_directory_name() {
        assert_eq!(extract_directory_name("/path/to/worktree"), "worktree");
        assert_eq!(extract_directory_name("simple-name"), "simple-name");
    }

    #[test]
    fn test_format_branch_name() {
        assert_eq!(format_branch_name("refs/heads/main"), "main");
        assert_eq!(format_branch_name("feature/test"), "feature/test");
    }

    #[test]
    fn test_format_worktree_display() {
        let worktree = Worktree {
            path: "/repo/feature-branch".to_string(),
            branch: "refs/heads/feature".to_string(),
            commit: "abc123".to_string(),
        };
        
        assert_eq!(format_worktree_display(&worktree), "feature-branch (feature)");
    }
}