use git2::Repository;

pub struct CloneCommand<'a> {
    pub url: &'a str,
    pub path: Option<&'a str>,
}

impl<'a> CloneCommand<'a> {
    pub fn new(url: &'a str, path: Option<&'a str>) -> Self {
        Self { url, path }
    }

    pub fn run(&self) {
        let repo = match Repository::clone(self.url, self.get_path()) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to clone: {}", e),
        };
        println!("Repo: {:?}", repo.path());
    }

    fn get_path(&self) -> &str {
        match self.path {
            Some(path) => path,
            None => ".",
        }
    }
}