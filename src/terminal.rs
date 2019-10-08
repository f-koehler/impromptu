use std::os::unix::io::RawFd;

fn get_terminal_size_from_fd(fd: RawFd) -> Option<(u16, u16)> {
    unsafe {
        let mut win: nix::libc::winsize = std::mem::zeroed();
        match nix::libc::ioctl(fd, nix::libc::TIOCGWINSZ, &mut win) {
            -1 => return None,
            _ => Some((win.ws_col as u16, win.ws_row as u16)),
        }
    }
}

cached! {
    TERMINAL_SIZE;
    fn get_terminal_size() -> (u16, u16) = {
        match get_terminal_size_from_fd(nix::libc::STDOUT_FILENO) {
            Some(result) => result,
            None => match get_terminal_size_from_fd(nix::libc::STDERR_FILENO) {
                Some(result) => result,
                None => match get_terminal_size_from_fd(nix::libc::STDIN_FILENO) {
                    Some(result) => result,
                    None => (0u16, 0u16),
                },
            },
        }
    }
}
