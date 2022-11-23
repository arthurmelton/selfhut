pub fn diffs<'a>(commit: git2::Commit<'a>, repo: &'a git2::Repository) -> Option<git2::Diff<'a>> {
    let tree = commit.tree().ok()?;
    let parent_tree = match commit.parent(0) {
        Ok(parent) => Some(parent.tree().ok()?),
        Err(_) => Some(
            repo.find_tree(git2::Oid::from_str("4b825dc642cb6eb9a060e54bf8d69288fbee4904").ok()?)
                .ok()?,
        ),
    }?;
    match repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None) {
        Ok(diff) => Some(diff),
        Err(_) => None,
    }
}
