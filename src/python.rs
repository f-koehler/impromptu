pub fn get_virtual_env_name() -> Option<String> {
    match std::env::var("VIRTUAL_ENV") {
        Err(_) => None,
        Ok(value) => {
            let path = std::path::PathBuf::from(value);
            match path.file_name() {
                None => match path.to_str() {
                    None => None,
                    Some(result) => Some(result.to_owned()),
                },
                Some(result) => match result.to_str() {
                    None => None,
                    Some(result) => Some(result.to_owned()),
                },
            }
        }
    }
}
