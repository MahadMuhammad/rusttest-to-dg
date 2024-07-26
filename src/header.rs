pub fn parse_edition_directive(line: &str, directive: &str) -> String {
    let colon = directive.len();
    let value = match line.starts_with(directive) && line.as_bytes().get(colon) == Some(&b':') {
        true => Some(line[(colon + 1)..].to_owned()),
        false => None,
    };

    format!(
        "// {{ dg-additional-options \"-frust-edition={}\" }}",
        value.unwrap()
    )
}
