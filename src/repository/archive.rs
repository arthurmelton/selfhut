use crate::config::CONFIG;
use crate::git::get_all::files;
use rocket::response::stream::ByteStream;
use tar::{Builder, Header};

#[get("/<repo>/archive/<oid>", rank = 2)]
pub fn archive(repo: String, oid: String) -> ByteStream![Vec<u8>] {
    ByteStream! {
        let oid = oid[..oid.len()-7].to_string();
        let files = files(repo.clone(), oid.clone(), "".to_string()).unwrap();
        for i in files {
            match get_tar(repo.clone(), oid.clone(), i) {
                Some(x) => yield x[..x.len()-1024].to_vec(),
                None => {},
            }
        }
        yield vec![0;1024]
    }
}

fn get_tar(repo: String, oid: String, path: String) -> Option<Vec<u8>> {
    let mut repo_path = CONFIG.git_location.clone();
    repo_path.push(format!("{}.git", repo));
    let repo_clone = repo.clone();
    let repo = git2::Repository::open(repo_path).unwrap();
    let obj = repo.revparse_single(&format!("{}:{}", oid, path)).ok()?;
    let blob = obj.as_blob()?;
    let mut header = Header::new_gnu();
    header.set_size(blob.size() as u64);
    header.set_cksum();
    let mut ar = Builder::new(Vec::new());
    ar.append_data(
        &mut header,
        format!("{}/{}", repo_clone, path),
        blob.content(),
    )
    .unwrap();
    Some(ar.into_inner().ok()?)
}
