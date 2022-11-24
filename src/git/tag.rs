use crate::config::CONFIG;
use crate::git::commits::{get_commits, Commits};
use serde_derive::Serialize;

pub fn get_tag(
    repo_name: String,
    amount: usize,
    after: usize,
    name: Option<String>,
) -> Option<(Vec<Tags>, usize)> {
    let mut repo_path = CONFIG.git_location.clone();
    repo_path.push(format!("{}.git", repo_name));
    let repo = git2::Repository::open(repo_path).ok()?;
    let mut tags = Vec::new();
    let total = repo.tag_names(None).ok()?.len();
    let mut i = total;
    let _ = repo.tag_foreach(|x, y| {
        if (name.is_some() && name.as_ref().unwrap().as_bytes() == &y[10..]) || name.is_none() {
            if i >= after && (total < amount + after || i < amount - after) {
                match get_commits(repo_name.clone(), 1, Some(x.to_string()), None) {
                    Some(z) => match z.first() {
                        Some(z) => tags.push(Tags {
                            commit: z.clone(),
                            body: match repo.find_tag(x) {
                                Ok(x) => x.message().unwrap_or("").to_string(),
                                Err(_) => "".to_string(),
                            },
                            name: std::str::from_utf8(&y[10..]).unwrap_or("").to_string(),
                        }),
                        None => {}
                    },
                    None => {}
                }
            }
            if i < after + 1 {
                return false;
            }
            i -= 1;
        }
        true
    });
    tags.reverse();
    Some((tags, total))
}

#[derive(Serialize, Clone)]
pub struct Tags {
    commit: Commits,
    name: String,
    body: String,
}
