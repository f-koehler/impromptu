struct TerminalSize {
    ws_row: nix::libc::c_ushort,
    ws_col: nix::libc::c_ushort,
    ws_xpixel: nix::libc::c_ushort,
    ws_ypixel: nix::libc::c_ushort,
}

cached! {
    TERMINAL_SIZE;

    fn get_terminal_size() -> (u16, u16) = {
        unsafe {
            let mut winsize = TerminalSize {
                ws_row: 0,
                ws_col: 0,
                ws_xpixel: 0,
                ws_ypixel: 0,
            };

            match nix::libc::ioctl(
                nix::libc::STDOUT_FILENO,
                nix::libc::TIOCGWINSZ,
                &mut winsize,
            ) {
                -1 => return (0u16, 0u16),
                _ => {
                    return (
                        std::cmp::max(0, winsize.ws_col) as u16,
                        std::cmp::max(0, winsize.ws_row),
                    )
                }
            }
        }
    }
}
