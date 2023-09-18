use git2::{Repository, RemoteCallbacks, Error, Cred};
use std::env;
use std::path::Path;

pub struct CloneCommand {
    pub url: String,
    pub path: Option<String>,
}

impl CloneCommand {
    pub fn new(url: String, path: Option<String>) -> Self {
        Self { url, path }
    }

    pub fn run(&self) {
        if self.url.starts_with("git@github.com:") {
            self.ssh_clone();
        } else if self.url.starts_with("https://") {
            self.https_clone();
        } else {
            panic!("Invalid URL");
        }
    }

    fn get_path(&self) -> &str {
        match &self.path {
            Some(path) => path.as_str(),
            None => ".",
        }
    }

    fn https_clone(&self) {
        let repo = match Repository::clone(self.url.as_str(), self.get_path()) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to clone: {}", e),
        };
        println!("Repo: {:?}", repo.path());
    }

    fn ssh_clone(&self) {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            Path::new(&format!("{}/.ssh/id_ed25519", env::var("HOME").unwrap())),
            None,
            )
        });

        // Prepare fetch options.
        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(callbacks);

        // Prepare builder.
        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fo);

        // Clone the project.
        let _ = builder.clone(
            &self.url.as_str(),
            Path::new(self.get_path()),
        );
    }
}
