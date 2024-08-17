use crafty_novels::stendhal;

fn main() {
    test_string_parsing();
}

fn test_string_parsing() {
    let parse = stendhal::Stendhal::parse_string;

    let input = r#"#- This is the start of the page
First line
#- New Page
Not a #- new page
 #- also not a new page



Lots of paragraph breaks
Some line breaks
Some line breaks (2)
Some line breaks (3)"#;

    parse(input);
}
