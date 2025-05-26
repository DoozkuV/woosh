#[cfg(test)]
mod tests {
    use woosh::ast::{Command, Redirection};
    use woosh::parser::parse_line;

    #[test]
    fn test_empty_input() {
        let input = "";
        let ast = parse_line(input).unwrap();
        assert!(matches!(ast, Command::Empty));
    }

    #[test]
    fn test_whitespace_input() {
        let input = "    ";
        let ast = parse_line(input).unwrap();
        assert!(matches!(ast, Command::Empty));
    }

    #[test]
    fn test_simple_command() {
        let input = "ls -l";
        let ast = parse_line(input).unwrap();

        if let Command::Simple(simple) = ast {
            assert_eq!(simple.program, "ls");
            assert_eq!(simple.args, vec!["-l"]);
            assert!(simple.redirection.is_none());
        } else {
            panic!("Expected Simple command");
        }
    }

    #[test]
    fn test_trailing_whitespace() {
        let input = "ls -l    ";
        let ast = parse_line(input).unwrap();

        if let Command::Simple(simple) = ast {
            assert_eq!(simple.program, "ls");
            assert_eq!(simple.args, vec!["-l"]);
            assert!(simple.redirection.is_none());
        } else {
            panic!("Expected Simple command");
        }
    }

    #[test]
    fn test_single_quoted_argument() {
        let input = "echo 'Hello World'";
        let ast = parse_line(input).unwrap();

        if let Command::Simple(simple) = ast {
            assert_eq!(simple.program, "echo");
            assert_eq!(simple.args, vec!["Hello World"]); // Quotes stripped
        } else {
            panic!("Expected Simple command");
        }
    }

    #[test]
    fn test_single_quoted_with_special_chars() {
        let input = r#"echo '$HOME * `ls` "'weird'\\"string"#;
        let ast = parse_line(input).unwrap();

        if let Command::Simple(simple) = ast {
            assert_eq!(simple.program, "echo");
            // The single-quoted portion should preserve everything literally
            // Double quotes remain as regular characters
            assert_eq!(simple.args, vec![r#"$HOME * `ls` ""#, r#""weird'\\string"#]);
        } else {
            panic!("Expected Simple command");
        }
    }

    #[test]
    fn test_single_quoted_in_pipeline() {
        let input = "cat 'my file.txt' | grep 'important pattern' | wc -l";
        let ast = parse_line(input).unwrap();

        if let Command::Pipeline(commands) = ast {
            assert_eq!(commands.len(), 3);
            assert_eq!(commands[0].args, vec!["my file.txt"]);
            assert_eq!(commands[1].args, vec!["important pattern"]);
            assert_eq!(commands[2].args, vec!["-l"]);
        } else {
            panic!("Expected Pipeline");
        }
    }

    #[test]
    fn test_mixed_quoted_and_unquoted() {
        let input = "echo 'Hello' World 'from shell'";
        let ast = parse_line(input).unwrap();

        if let Command::Simple(simple) = ast {
            assert_eq!(simple.args, vec!["Hello", "World", "from shell"]);
        } else {
            panic!("Expected Simple command");
        }
    }

    #[test]
    fn test_empty_single_quotes() {
        let input = "echo ''";
        let ast = parse_line(input).unwrap();

        if let Command::Simple(simple) = ast {
            assert_eq!(simple.args, vec![""]);
        } else {
            panic!("Expected Simple command");
        }
    }

    #[test]
    fn test_pipeline() {
        let input = "ls -l | grep rs | wc -l";
        let ast = parse_line(input).unwrap();

        if let Command::Pipeline(commands) = ast {
            assert_eq!(commands.len(), 3);

            assert_eq!(commands[0].program, "ls");
            assert_eq!(commands[0].args, vec!["-l"]);
            assert!(commands[0].redirection.is_none());

            assert_eq!(commands[1].program, "grep");
            assert_eq!(commands[1].args, vec!["rs"]);
            assert!(commands[1].redirection.is_none());

            assert_eq!(commands[2].program, "wc");
            assert_eq!(commands[2].args, vec!["-l"]);
            assert!(commands[2].redirection.is_none());
        } else {
            panic!("Expected Pipeline");
        }
    }

    #[test]
    fn test_output_redirection() {
        let input = "ls > output.txt";
        let ast = parse_line(input).unwrap();

        if let Command::Simple(simple) = ast {
            assert_eq!(simple.program, "ls");
            assert!(simple.args.is_empty());
            if let Some(Redirection::Stdout(file)) = simple.redirection {
                assert_eq!(file, "output.txt");
            } else {
                panic!("Expected stdout redirection");
            }
        } else {
            panic!("Expected Simple command");
        }
    }

    #[test]
    fn test_pipeline_with_redirection() {
        let input = "ls | grep rs > output.txt";
        let ast = parse_line(input).unwrap();

        if let Command::Pipeline(commands) = ast {
            assert_eq!(commands.len(), 2);

            assert_eq!(commands[0].program, "ls");
            assert!(commands[0].args.is_empty());
            assert!(commands[0].redirection.is_none());

            assert_eq!(commands[1].program, "grep");
            assert_eq!(commands[1].args, vec!["rs"]);
            if let Some(Redirection::Stdout(file)) = &commands[1].redirection {
                assert_eq!(file, "output.txt");
            } else {
                panic!("Expected stdout redirection in second command");
            }
        } else {
            panic!("Expected Pipeline");
        }
    }

    #[test]
    fn test_invalid_pipe_ending() {
        let input = "ls |";
        assert!(parse_line(input).is_err());
    }

    #[test]
    fn test_invalid_redirection() {
        let input = "> output.txt";
        assert!(parse_line(input).is_err());
    }

    #[test]
    fn test_complex_command() {
        let input = r#"find . -name "*.rs" | xargs grep "pattern" > results.txt"#;
        let ast = parse_line(input).unwrap();

        if let Command::Pipeline(commands) = ast {
            assert_eq!(commands.len(), 2);

            assert_eq!(commands[0].program, "find");
            assert_eq!(commands[0].args, vec![".", "-name", r#""*.rs""#]);

            assert_eq!(commands[1].program, "xargs");
            assert_eq!(commands[1].args, vec!["grep", r#""pattern""#]);
            if let Some(Redirection::Stdout(file)) = &commands[1].redirection {
                assert_eq!(file, "results.txt");
            } else {
                panic!("Expected stdout redirection");
            }
        } else {
            panic!("Expected Pipeline");
        }
    }
}
