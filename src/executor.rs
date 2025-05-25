use crate::ast::{Command, Redirection, SimpleCommand};
use anyhow::{bail, Context, Result};
use nix::unistd::pipe;
use std::{
    fs::File,
    os::fd::OwnedFd,
    process::{Command as StdCommand, Stdio},
};

pub fn eval(cmd: Command) -> Result<()> {
    match cmd {
        Command::Simple(sc) => exec_simple(sc),
        Command::Pipeline(vec) => exec_pipeline(&vec),
        Command::Empty => Ok(()),
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
    let (mut reader_pipes, mut writer_pipes) = {
        let (reader_pipes_vec, writer_pipes_vec): (Vec<OwnedFd>, Vec<OwnedFd>) = (0..n - 1)
            .map(|_| pipe().context("pipe creation failed"))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .unzip();

        (reader_pipes_vec.into_iter(), writer_pipes_vec.into_iter())
    };

    let mut children = Vec::with_capacity(n);
    for (i, sc) in cmds.iter().enumerate() {
        let mut c = StdCommand::new(&sc.program);
        c.args(&sc.args);

        // stdin: for command 0 => inherit, else read end of pipe[i-1]
        if i > 0 {
            let r = reader_pipes.next().unwrap();
            c.stdin(Stdio::from(r));
        }

        // stdout: for last command => possible redirection or inherit
        //                             else write end of pipe[i]
        if i < n - 1 {
            let w = writer_pipes.next().unwrap();
            c.stdout(Stdio::from(w));
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
