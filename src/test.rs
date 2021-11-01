use crate::*;

#[test]
fn bold() -> Result<()> {
    let bold = parse_md("**hello world**")?;
    println!("{:#?}", bold);
    assert_eq!(
        bold[0],
        Span {
            kind: MarkdownKind::Bold,
            s: "hello world",
            range: 2..13
        }
    );

    Ok(())
}

#[test]
fn italics() -> Result<()> {
    let asterisks = parse_md("*hello world*")?;
    println!("{:#?}", asterisks);
    let underscores = parse_md("_hello world_")?;
    println!("{:#?}", asterisks);
    assert!(asterisks.len() == 1);
    assert_eq!(asterisks, underscores);
    assert_eq!(
        underscores[0],
        Span {
            kind: MarkdownKind::Italic,
            s: "hello world",
            range: 1..12
        }
    );
    Ok(())
}

#[test]
fn bold_italics() -> Result<()> {
    let three_asterisks = parse_md("***hello world***")?;
    let underscore_double_asterisk = parse_md("**_hello world_**")?;
    let double_asterisk_underscore = parse_md("_**hello world**_")?;
    println!("{:#?}", three_asterisks);
    assert!(three_asterisks.len() == 1);
    assert_eq!(three_asterisks, underscore_double_asterisk);
    assert_eq!(underscore_double_asterisk, double_asterisk_underscore);
    assert_eq!(
        three_asterisks[0],
        Span {
            kind: MarkdownKind::BoldItalic,
            s: "hello world",
            range: 3..14
        }
    );

    Ok(())
}

#[test]
fn underline() -> Result<()> {
    let underline = parse_md("__hello world__")?;
    println!("{:#?}", underline);
    assert_eq!(
        underline[0],
        Span {
            kind: MarkdownKind::Underline,
            s: "hello world",
            range: 2..13
        }
    );

    Ok(())
}
