use crate::config::CONFIG;
use git2::ObjectType;





pub fn files(repo: String, branch: String, path: String) -> Option<Vec<String>> {
    if branch.contains(":") {
        return None;
    }
    let mut repo_path = CONFIG.git_location.clone();
    repo_path.push(format!("{}.git", repo));
    let repo_clone = repo.clone();
    let repo = git2::Repository::open(repo_path).ok()?;
    let obj = repo.revparse_single(&format!("{}:{}", branch, path)).ok()?;
    let mut returns = Vec::new();
    match obj.kind() {
        Some(ObjectType::Tree) => {
            let tree = obj.as_tree()?;
            for entry in tree.iter() {
                if entry.kind() == Some(ObjectType::Blob) {
                    returns.push(format!("{}{}", path, entry.name()?));
                }
                else {
                    for i in files(repo_clone.clone(), branch.clone(), format!("{}{}/", path, entry.name()?))? {
                        returns.push(i);
                    }
                }
            }
        }
        _ => {
            return None;
        }
    }
    Some(returns)
}
