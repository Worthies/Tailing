use std::env;
use std::io::{self, BufRead, Write};
// use std::os::unix::process::parent_id;
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    // let ppid = parent_id();
    // let tag = format!("tail-{}", ppid);
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

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        if let Some(ref mut jstdin) = journal_child.stdin {
            writeln!(jstdin, "{}", line)?;
        }
        if let Some(ref mut tstdin) = tail_child.stdin {
            writeln!(tstdin, "{}", line)?;
        }
    }

    drop(journal_child.stdin.take());
    journal_child.wait()?;

    drop(tail_child.stdin.take());
    tail_child.wait()?;

    Ok(())
}
