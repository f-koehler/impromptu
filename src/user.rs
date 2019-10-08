use std::ffi::CStr;
use std::path::PathBuf;

pub struct Passwd {
    pub username: String,
    pub home_directory: PathBuf,
}

pub fn get_passwd() -> Passwd {
    let mut result = Passwd {
        username: String::new(),
        home_directory: PathBuf::new(),
    };
    unsafe {
        let passwd = nix::libc::getpwuid(nix::libc::geteuid());
        result.username = CStr::from_ptr((*passwd).pw_name)
            .to_string_lossy()
            .into_owned();
        result.home_directory = PathBuf::from(
            CStr::from_ptr((*passwd).pw_dir)
                .to_string_lossy()
                .into_owned(),
        );
    }
    result
}

pub fn is_root() -> bool {
    nix::unistd::geteuid().is_root()
}
