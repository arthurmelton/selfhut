pub fn diffs<'a>(commit: git2::Commit<'a>, repo: &'a git2::Repository) -> Option<git2::Diff<'a>> {
    match commit.tree() {
        Ok(tree) => match commit.parent(0) {
            Ok(parent) => match parent.tree() {
                Ok(parent_tree) => {
                    match repo.diff_tree_to_tree(Some(&tree), Some(&parent_tree), None) {
                        Ok(diff) => Some(diff),
                        Err(_) => None,
                    }
                }
                Err(_) => None,
            },
            Err(_) => None,
        },
        Err(_) => None,
    }
}
