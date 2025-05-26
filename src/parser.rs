use crate::ast::{Command, Redirection, SimpleCommand, OPERATORS};
use anyhow::{anyhow, Context};
use nom::bytes::complete::{is_not, tag, take_while1};
use nom::{
    character::complete::{multispace0, space1},
    combinator::{all_consuming, opt},
    error::{Error, ErrorKind},
    multi::separated_list0,
    sequence::{delimited, preceded},
    Err, IResult, Parser,
};

fn single_quoted_word(input: &str) -> IResult<&str, &str> {
    delimited(tag("'"), is_not("'"), tag("'")).parse(input)
}

/// A shell word is anything non-whitespace and not a redirection/operator
fn token(input: &str) -> IResult<&str, String> {
    separated_list0(
        single_quoted_word,
        take_while1(|c: char| !c.is_whitespace() && !OPERATORS.contains(&c)),
    )
    .parse(input)
    .map(|(l, r)| (l, r.concat()))
}

fn redirection(input: &str) -> IResult<&str, Redirection> {
    let (input, _) = preceded(multispace0, tag(">")).parse(input)?;
    let (input, _) = multispace0(input)?;
    let (input, file) = token(input)?;

    Ok((input, Redirection::Stdout(file.to_string())))
}

fn simple_command(input: &str) -> IResult<&str, SimpleCommand> {
    let (input, words) = separated_list0(space1, token).parse(input)?;
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
    let (input, _) = multispace0(input)?;

    if input.is_empty() {
        return Ok((input, Command::Empty));
    }

    let (input, cmds) = separated_list0(
        delimited(multispace0, tag("|"), multispace0),
        simple_command,
    )
    .parse(input)?;

    let (input, _) = multispace0(input)?; // Skip trailing whitespace

    Ok((
        input,
        match cmds.len() {
            1 => Command::Simple(cmds.into_iter().next().unwrap()),
            _ => Command::Pipeline(cmds),
        },
    ))
}

// TODO: Create a better error type
pub fn parse_line(input: &str) -> anyhow::Result<Command> {
    all_consuming(pipeline)
        .parse(input)
        .map(|(_, cmd)| cmd)
        .map_err(|e| anyhow!("parse error: {:?}", e))
        .context("while parsing shell command")
}
