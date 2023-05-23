use std::process::Command;
use std::time::Duration;

use anyhow::Context;
use anyhow::Result;
use console::Key;
use console::Term;
use wait_timeout::ChildExt;

pub fn main() -> Result<()> {
    // create and run a command, wait for it using the `wait-timeout` crate
    run_and_wait().context("run and wait")?;

    println!("send me a SIGCHLD on {} plz", std::process::id());

    // do something else that uses syscalls (here: poll)
    println!("result is: {:?}", read_console());
    Ok(())
}

fn run_and_wait() -> Result<()> {
    let mut command = Command::new("sleep");
    let mut child = command
        .args(vec!["1"])
        .spawn()
        .context("child to be created")?;
    let timeout = Duration::from_secs(2);
    let code = match child.wait_timeout(timeout).context("waited for a sec")? {
        Some(status) => status.code(),
        None => {
            child.kill().context("send kill to child")?;
            child.wait().unwrap().code()
        }
    };
    println!("execution status code: {code:?}");
    Ok(())
}

fn read_console() -> Result<()> {
    let term = Term::stdout();
    loop {
        term.write_str("quit? (y/n) > ")
            .context("write to terminal")?;
        match term.read_key().context("read from terminal")? {
            Key::Char('y') => {
                term.write_line("YES").context("write YES");
                return Ok(());
            }
            _ => term.write_line("no").context("write no")?,
        };
    }
}
