use crate::config::CONFIG;

pub fn get_tag(repo_name: String, amount: usize) -> Option<Vec<String>> {
    let mut repo_path = CONFIG.git_location.clone();
    repo_path.push(format!("{}.git", repo_name));
    let repo = git2::Repository::open(repo_path).ok()?;
    let mut tags = Vec::new();
    let mut i = 0;
    let repo_tags = repo.tag_names(None).ok()?;
    let mut tags_stringarray = repo_tags.iter().rev();
    let mut tag = tags_stringarray.next();
    while i < amount && tag.is_some() && tag.unwrap().is_some() {
        tags.push(tag.unwrap().unwrap().to_string());
        tag = tags_stringarray.next();
        i+=1;
    }
    Some(tags)
}
