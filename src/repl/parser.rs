pub fn parse_args(input: String) -> Vec<String> {
    let mut args = Vec::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut backslash = false;
    let mut current_arg = String::new();
    for c in input.chars() {
        if backslash {
            current_arg.push(c);
            backslash = false;
        } else if in_single_quote {
            if c == '\'' {
                in_single_quote = false;
            } else {
                current_arg.push(c);
            }
        } else if in_double_quote {
            if c == '"' {
                in_double_quote = false;
            } else if c == '\\' {
                backslash = true;
            } else {
                current_arg.push(c);
            }
        } else {
            if c == '\\' {
                backslash = true;
            } else if c == '\'' {
                in_single_quote = true;
            } else if c == '"' {
                in_double_quote = true;
            } else if c == ' ' {
                if !current_arg.is_empty() {
                    args.push(current_arg);
                    current_arg = String::new();
                }
            } else {
                current_arg.push(c);
            }
        }
    }
    args.push(current_arg);
    args
}
