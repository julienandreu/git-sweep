use std::{
    env,
    process::{exit, Command},
    time::Instant,
};

fn git_fetch() {
    Command::new("git")
        .current_dir(env::current_dir().unwrap())
        .args(["fetch"])
        .output()
        .expect("Failed to execute 'git fetch' command");
}

fn git_branch() -> Vec<String> {
    let output = Command::new("git")
        .current_dir(env::current_dir().unwrap())
        .args(["branch"])
        .output()
        .expect("Failed to execute 'git branch' command");

    if output.status.success() {
        return String::from_utf8_lossy(&output.stdout.to_owned())
            .lines()
            .into_iter()
            .filter_map(|branch| {
                if branch.starts_with('*') {
                    return None;
                }

                Some(branch.trim().to_owned())
            })
            .collect();
    }

    vec![]
}

fn git_branch_delete(branch: &str) -> String {
    Command::new("git")
        .current_dir(env::current_dir().unwrap())
        .args(["branch", "-D", &branch])
        .output()
        .expect("Failed to execute 'git branch -D' command");

    branch.to_owned()
}

fn format_branch_list(branches: &Vec<String>) -> String {
    branches.join("\n ")
}

fn main() {
    let start = Instant::now();

    git_fetch();
    let branches = git_branch();

    if branches.is_empty() {
        println!("No branch to sweep");
        exit(0);
    }

    let swept_branches = branches
        .into_iter()
        .map(|branch| git_branch_delete(&branch))
        .collect::<Vec<String>>();

    let swept_message = format!(
        "{} branch{} successfully swept in {:?}:\n {}",
        swept_branches.len(),
        if swept_branches.len() > 1 { "es" } else { "" },
        start.elapsed(),
        format_branch_list(&swept_branches)
    );

    println!("{}", swept_message);
}
