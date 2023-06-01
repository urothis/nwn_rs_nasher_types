use git2::Repository;

fn main() {
  let path = "assets/testing/modules/";

  let urls = vec![
    "https://github.com/b5635/the-frozen-north",
    "https://github.com/urothis/nwn-module-DungeonEternalX",
  ];

  // clone the repos
  for url in urls {
    // get the repo name
    let repo_name = match url.rsplit("/").next() {
      Some(name) => name,
      None => panic!("Failed to get repo name from url: {}", url),
    };
    // clone the repo
    match Repository::clone(url, path.to_owned() + repo_name) {
      Ok(repo) => repo,
      Err(_) => {
        continue;
      }
    };
  }
}
