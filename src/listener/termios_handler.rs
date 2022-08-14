use std::fs::OpenOptions;
use std::io::Result;
use std::os::unix::io::AsRawFd;
use termios::{Termios, ECHO, ICANON, tcsetattr, TCSADRAIN};

/* https://man7.org/linux/man-pages/man3/tcflow.3.html */
pub fn setup_fd() -> Result<()> {
    let tty = OpenOptions::new().write(true).read(true).open("/dev/tty")?;
    let fd = tty.as_raw_fd();
    let mut termios = Termios::from_fd(fd)?;

    /* !ECHO: Disable Echo input characters
    !ICANON Disable canonical mode */
    termios.c_lflag &= !(ECHO | ICANON);

    /* Applies the changes after all ouput written to fd
    has been transmitted */
    tcsetattr(fd, TCSADRAIN, &termios)?;
    Ok(())
}

/* TODO: Maybe implement a custom termios with libc since the
termion crate uses uninitialized memory.
https://github.com/dcuddeback/termios-rs/blob/master/src/lib.rs#L194 */
