cached! {
    HOSTNAME;

    fn get_hostname() -> String = {
        let buffer_size = match nix::unistd::sysconf(nix::unistd::SysconfVar::HOST_NAME_MAX).unwrap() {
            Some(len) => len as usize,
            None => 128 as usize,
        };
        let mut buf: Vec<u8> = Vec::with_capacity(buffer_size);
        buf.resize(buffer_size, 0u8);
        match nix::unistd::gethostname(buf.as_mut()) {
            Err(_err) => "unknown".to_string(),
            Ok(hostname) => hostname.to_string_lossy().into_owned(),
        }
    }
}
