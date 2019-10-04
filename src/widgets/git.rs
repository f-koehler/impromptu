use std::path::PathBuf;

pub fn get_git_repo(path: PathBuf) -> Option<git2::Repository> {
    let result = git2::Repository::discover(path);
    if result.is_ok() {
        result.ok()
    } else {
        None
    }
}

pub struct GitStatus {
    pub has_modifications: bool,
    pub is_detached: bool,
    pub is_clean: bool,
    pub state: git2::RepositoryState,
    pub description: String,
}

pub fn handle_git_repo(repo: &git2::Repository) -> GitStatus {
    let mut result = GitStatus {
        has_modifications: false,
        is_detached: false,
        is_clean: false,
        state: git2::RepositoryState::Clean,
        description: String::new(),
    };

    match repo.head_detached() {
        Err(_err) => (),
        Ok(detached) => result.is_detached = detached,
    }

    match repo.statuses(None) {
        Err(_err) => (),
        Ok(statuses) => {
            for i in 0..statuses.len() {
                match statuses.get(i) {
                    None => (),
                    Some(status) => result.has_modifications |= status.status().bits() != 0,
                }
            }
        }
    }

    let mut describe_options = git2::DescribeOptions::new();
    describe_options.describe_all();
    let mut describe_fmt_options = git2::DescribeFormatOptions::new();
    describe_fmt_options.dirty_suffix("⚡");
    match repo.describe(&describe_options) {
        Err(_err) => result.description = "Ø".to_string(),
        Ok(description) => match description.format(Some(&describe_fmt_options)) {
            Err(_err) => result.description = "unknw".to_string(),
            Ok(text) => result.description = text,
        },
    }

    result.is_clean = repo.state() == git2::RepositoryState::Clean;

    result
}
