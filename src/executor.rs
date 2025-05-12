use crate::ast::{Command, Redirection, SimpleCommand};
use anyhow::{bail, Context, Result};
use nix::unistd::{close, pipe};
use std::{
    fs::File,
    os::fd::FromRawFd,
    process::{Command as StdCommand, Stdio},
};

pub fn eval(cmd: Command) -> Result<()> {
    match cmd {
        Command::Simple(sc) => exec_simple(sc),
        Command::Pipeline(vec) => exec_pipeline(&vec),
    }
}

fn exec_simple(sc: SimpleCommand) -> Result<()> {
    let mut c = StdCommand::new(&sc.program);
    c.args(&sc.args);

    if let Some(Redirection::Stdout(path)) = sc.redirection {
        let f = File::create(&path)
            .with_context(|| format!("failed to open redirect file `{}`", path))?;
        c.stdout(Stdio::from(f));
    }

    let status = c
        .status()
        .with_context(|| format!("failed to spawn `{}`", sc.program))?;

    if !status.success() {
        bail!("`{}` exited with {}", sc.program, status);
    }
    Ok(())
}

fn exec_pipeline(cmds: &[SimpleCommand]) -> Result<()> {
    let n = cmds.len();
    // For N commands we need N-1 pipes
    let mut pipes = Vec::with_capacity(n - 1);
    for _ in 0..n - 1 {
        let (r, w) = pipe().context("pipe creation failed")?;
        pipes.push((r, w));
    }

    let mut children = Vec::with_capacity(n);
    for (i, sc) in cmds.iter().enumerate() {
        let mut c = StdCommand::new(&sc.program);
        c.args(&sc.args);

        // stdin: for command 0 => inherit, else read end of pipe[i-1]
        if i > 0 {
            let (r, _) = &pipes[i - 1];
            c.stdin(Stdio::from(r.try_clone()?));
        }

        // stdout: for last command => possible redirection or inherit
        //                             else write end of pipe[i]
        if i < n - 1 {
            let (_, w) = &pipes[i];
            c.stdout(Stdio::from(w.try_clone()?));
        } else if let Some(Redirection::Stdout(path)) = &sc.redirection {
            let f = File::create(path)
                .with_context(|| format!("failed to open redirect file `{}`", path))?;
            c.stdout(Stdio::from(f));
        }

        let child = c
            .spawn()
            .with_context(|| format!("failed to spawn `{}`", sc.program))?;
        children.push(child);
    }

    // Wait for all children
    for mut child in children {
        let status = child.wait()?;
        if !status.success() {
            bail!("pipeline element exited with {}", status);
        }
    }

    Ok(())
}
