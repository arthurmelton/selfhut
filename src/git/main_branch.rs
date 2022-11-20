use crate::config::CONFIG;

pub fn main_branch(repo_name: String) -> Option<String> {
    let mut repo_path = CONFIG.git_location.clone();
    repo_path.push(format!("{}.git", repo_name));
    let repo = git2::Repository::open(repo_path).ok()?;
    let x = Some(
        repo.branches(Some(git2::BranchType::Local))
            .ok()?
            .filter(|branch| branch.is_ok() && branch.as_ref().unwrap().0.is_head())
            .map(|branch| branch.unwrap().0)
            .collect::<Vec<git2::Branch>>()
            .get(0)?
            .name()
            .ok()??
            .to_string(),
    );
    x
}
