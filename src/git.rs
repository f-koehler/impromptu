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
    pub is_detached: bool,
    pub is_clean: bool,
    pub state: git2::RepositoryState,
    pub description: String,
    pub number_changes: u64,
    pub number_modified: u64,
    pub number_new: u64,
    pub number_deleted: u64,
    pub number_conflicts: u64,
}

impl GitStatus {
    fn new() -> GitStatus {
        GitStatus {
            is_detached: false,
            is_clean: false,
            state: git2::RepositoryState::Clean,
            description: String::new(),
            number_changes: 0u64,
            number_modified: 0u64,
            number_new: 0u64,
            number_deleted: 0u64,
            number_conflicts: 0u64,
        }
    }
}

pub fn handle_git_repo(repo: &git2::Repository) -> GitStatus {
    let mut result = GitStatus::new();

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
                    Some(status) => {
                        if status.status().bits() != 0u32 {
                            result.number_changes += 1;
                        }
                        if status.status().is_index_modified()
                            || status.status().is_wt_modified()
                            || status.status().is_index_renamed()
                            || status.status().is_wt_renamed()
                            || status.status().is_index_typechange()
                            || status.status().is_wt_typechange()
                        {
                            result.number_modified += 1;
                        }
                        if status.status().is_index_new() || status.status().is_wt_new() {
                            result.number_new += 1;
                        }
                        if status.status().is_index_deleted() || status.status().is_wt_deleted() {
                            result.number_deleted += 1;
                        }
                        if status.status().is_conflicted() {
                            result.number_conflicts += 1;
                        }
                    }
                }
            }
        }
    }

    let mut describe_options = git2::DescribeOptions::new();
    describe_options.describe_all();
    let mut describe_fmt_options = git2::DescribeFormatOptions::new();
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
