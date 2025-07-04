mod worktree;
mod ui;

use ui::run_interactive_worktree_menu;

fn main() {
    match run_interactive_worktree_menu() {
        Ok(_) => println!("Warmtree completed successfully!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
