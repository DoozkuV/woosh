/*
Copyright (C) 2025 George Nicholas Padron

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use crate::ast::{Command, Redirection, SimpleCommand, OPERATORS};
use anyhow::{anyhow, Context};
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{multispace0, space1},
    combinator::{all_consuming, opt},
    error::{Error, ErrorKind},
    multi::separated_list0,
    sequence::{delimited, preceded},
    Err, IResult, Parser,
};

/// A shell word is anything non-whitespace and not a redirection/operator
fn word(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| !c.is_whitespace() && !OPERATORS.contains(&c))(input)
}

fn redirection(input: &str) -> IResult<&str, Redirection> {
    let (input, _) = preceded(multispace0, tag(">")).parse(input)?;
    let (input, _) = multispace0(input)?;
    let (input, file) = word(input)?;

    Ok((input, Redirection::Stdout(file.to_string())))
}

fn simple_command(input: &str) -> IResult<&str, SimpleCommand> {
    let (input, words) = separated_list0(space1, word).parse(input)?;
    let (input, redirection) = opt(redirection).parse(input)?;

    if words.is_empty() {
        return Err(Err::Error(Error::new(input, ErrorKind::NonEmpty)));
    }

    let program = words[0].to_string();
    let args = words[1..].iter().map(|s| s.to_string()).collect();

    Ok((
        input,
        SimpleCommand {
            program: program.to_string(),
            args,
            redirection,
        },
    ))
}

fn pipeline(input: &str) -> IResult<&str, Command> {
    let (input, cmds) = separated_list0(
        delimited(multispace0, tag("|"), multispace0),
        simple_command,
    )
    .parse(input)?;

    if cmds.len() == 1 {
        Ok((input, Command::Simple(cmds.into_iter().next().unwrap())))
    } else {
        Ok((input, Command::Pipeline(cmds)))
    }
}

// TODO: Create a better error type
pub fn parse(input: &str) -> anyhow::Result<Command> {
    all_consuming(pipeline)
        .parse(input)
        .map(|(_, cmd)| cmd)
        .map_err(|e| anyhow!("parse error: {:?}", e))
        .context("while parsing shell command")
}
