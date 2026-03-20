use std::env;
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

const PIDFD_OPEN: libc::c_long = 434; // sys_pidfd_open

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let tag = "tailing";

    eprintln!("logger tag: {} (example: journalctl -t {} -f)", tag, tag);

    let mut journal_child = Command::new("/usr/bin/logger")
        .arg("-t")
        .arg(&tag)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    let mut tail_child = Command::new("/usr/bin/tail")
        .args(&args[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    let tail_pid = tail_child.id();
    let pidfd = unsafe { libc::syscall(PIDFD_OPEN, tail_pid as libc::pid_t, 0) as libc::c_int };
    if pidfd < 0 {
        return Err(io::Error::last_os_error());
    }

    let stdin_fd = 0;

    let mut tail_stdin = tail_child.stdin.take().unwrap();
    let mut journal_stdin = journal_child.stdin.take().unwrap();
    let mut stdin = io::stdin();

    loop {
        let mut pollfds = [
            libc::pollfd {
                fd: stdin_fd,
                events: libc::POLLIN,
                revents: 0,
            },
            libc::pollfd {
                fd: pidfd,
                events: libc::POLLIN,
                revents: 0,
            },
        ];

        unsafe {
            if libc::poll(pollfds.as_mut_ptr(), 2, -1) < 0 {
                break;
            }
        }

        // Check if tail process exited (pidfd is readable)
        if pollfds[1].revents & libc::POLLIN != 0 {
            break;
        }

        // Check if stdin has data
        if pollfds[0].revents & libc::POLLIN != 0 {
            let mut buf = [0u8; 4096];
            match stdin.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    if tail_stdin.write_all(&buf[..n]).is_err() {
                        break;
                    }
                    let _ = journal_stdin.write_all(&buf[..n]);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(_) => break,
            }
        }

        // Check if stdin is closed (EOF)
        if pollfds[0].revents & (libc::POLLHUP | libc::POLLERR) != 0 {
            break;
        }
    }

    drop(tail_stdin);
    drop(journal_stdin);
    drop(stdin);
    unsafe { libc::close(pidfd) };

    let _ = journal_child.wait();
    let tail_status = tail_child.wait()?;

    std::process::exit(tail_status.code().unwrap_or(0))
}
