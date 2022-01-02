use crate::*;

#[test]
fn italics() -> Result<()> {
    let asterisks = parse("hello *world*!")?;
    let underscores = parse("hello _world_!")?;
    println!("{:#?}", asterisks);
    println!("{:#?}", underscores);
    assert_eq!(
        asterisks,
        &[
            Span {
                kind: None,
                s: "hello ",
                range: 0..6
            },
            Span {
                kind: Some(MarkdownKind::Italics),
                s: "world",
                range: 7..12
            },
            Span {
                kind: None,
                s: "!",
                range: 13..14
            }
        ]
    );
    assert_eq!(asterisks, underscores);
    Ok(())
}

#[test]
fn bold() -> Result<()> {
    let parsed = parse("hello **world**!")?;
    println!("{:#?}", parsed);
    assert_eq!(
        parsed,
        &[
            Span {
                kind: None,
                s: "hello ",
                range: 0..6
            },
            Span {
                kind: Some(MarkdownKind::Bold),
                s: "world",
                range: 8..13
            },
            Span {
                kind: None,
                s: "!",
                range: 15..16
            }
        ]
    );

    Ok(())
}

#[test]
fn bold_italics() -> Result<()> {
    let three_asterisks = parse("hello ***world***!")?;
    let underscore_double_asterisk = parse("hello **_world_**!")?;
    let double_asterisk_underscore = parse("hello _**world**_!")?;
    dbg!(&three_asterisks);
    dbg!(&underscore_double_asterisk);
    dbg!(&double_asterisk_underscore);
    assert_eq!(three_asterisks, underscore_double_asterisk);
    assert_eq!(underscore_double_asterisk, double_asterisk_underscore);

    assert_eq!(
        three_asterisks,
        &[
            Span {
                kind: None,
                s: "hello ",
                range: 0..6
            },
            Span {
                kind: Some(MarkdownKind::BoldItalics),
                s: "world",
                range: 9..14
            },
            Span {
                kind: None,
                s: "!",
                range: 17..18
            }
        ]
    );

    Ok(())
}

#[test]
fn bold_underscore() -> Result<()> {
    let double_asterisk_double_underscore = parse("hello **__world__**!")?;
    let double_underscore_double_asterisk = parse("hello __**world**__!")?;

    dbg!(&double_asterisk_double_underscore);
    dbg!(&double_underscore_double_asterisk);

    assert_eq!(
        double_asterisk_double_underscore,
        double_underscore_double_asterisk
    );

    assert_eq!(
        double_asterisk_double_underscore,
        &[
            Span {
                kind: None,
                s: "hello ",
                range: 0..6
            },
            Span {
                kind: Some(MarkdownKind::BoldUnderscore),
                s: "world",
                range: 10..15
            },
            Span {
                kind: None,
                s: "!",
                range: 19..20
            }
        ]
    );

    Ok(())
}

#[test]
fn bold_italics_underscore() -> Result<()> {
    let triple_asterisk_double_underscore = parse("hello ***__world__***!")?;
    let double_asterisk_triple_underscore = parse("hello **___world___**!")?;
    let underscore_double_asterisk_double_underscore = parse("hello _**__world__**_!")?;
    let double_underscore_triple_asterisk = parse("hello __***world***__!")?;
    let double_underscore_double_asterisk_underscore = parse("hello __**_world_**__!")?;
    let triple_underscore_double_asterisk = parse("hello ___**world**___!")?;

    dbg!(&triple_asterisk_double_underscore);
    dbg!(&double_asterisk_triple_underscore);
    dbg!(&underscore_double_asterisk_double_underscore);
    dbg!(&double_underscore_triple_asterisk);
    dbg!(&double_underscore_double_asterisk_underscore);
    dbg!(&triple_underscore_double_asterisk);

    assert_eq!(
        triple_asterisk_double_underscore,
        double_asterisk_triple_underscore
    );
    assert_eq!(
        double_asterisk_triple_underscore,
        underscore_double_asterisk_double_underscore
    );
    assert_eq!(
        underscore_double_asterisk_double_underscore,
        double_underscore_triple_asterisk
    );
    assert_eq!(
        double_underscore_triple_asterisk,
        double_underscore_double_asterisk_underscore
    );
    assert_eq!(
        double_underscore_double_asterisk_underscore,
        triple_underscore_double_asterisk
    );

    assert_eq!(
        triple_asterisk_double_underscore,
        &[
            Span {
                kind: None,
                s: "hello ",
                range: 0..6
            },
            Span {
                kind: Some(MarkdownKind::BoldItalicsUnderscore),
                s: "world",
                range: 11..16
            },
            Span {
                kind: None,
                s: "!",
                range: 21..22
            }
        ]
    );

    Ok(())
}

#[test]
fn underscore() -> Result<()> {
    let parsed = parse("hello __world__!")?;
    println!("{:#?}", parsed);
    assert_eq!(
        parsed,
        &[
            Span {
                kind: None,
                s: "hello ",
                range: 0..6
            },
            Span {
                kind: Some(MarkdownKind::Underscore),
                s: "world",
                range: 8..13
            },
            Span {
                kind: None,
                s: "!",
                range: 15..16
            }
        ]
    );

    Ok(())
}

#[test]
fn inline_code() -> Result<()> {
    let parsed = parse("hello `world`!")?;
    println!("{:#?}", parsed);
    assert_eq!(
        parsed,
        &[
            Span {
                kind: None,
                s: "hello ",
                range: 0..6
            },
            Span {
                kind: Some(MarkdownKind::InlineCode),
                s: "world",
                range: 7..12
            },
            Span {
                kind: None,
                s: "!",
                range: 13..14
            }
        ]
    );
    Ok(())
}

#[test]
fn strikethrough() -> Result<()> {
    let parsed = parse("hello ~~world~~!")?;
    println!("{:#?}", parsed);
    assert_eq!(
        parsed,
        &[
            Span {
                kind: None,
                s: "hello ",
                range: 0..6
            },
            Span {
                kind: Some(MarkdownKind::Strikethrough),
                s: "world",
                range: 8..13
            },
            Span {
                kind: None,
                s: "!",
                range: 15..16
            }
        ]
    );
    Ok(())
}

#[test]
fn nothing() -> Result<()> {
    let parsed = parse("hello world")?;
    println!("{:#?}", parsed);
    assert_eq!(
        parsed[0],
        Span {
            kind: None,
            s: "hello world",
            range: 0..11
        }
    );

    Ok(())
}

#[test]
fn nothing_unicode() -> Result<()> {
    let parsed = parse("hello 𓀕 world")?;
    println!("{:#?}", parsed);
    assert_eq!(
        parsed[0],
        Span {
            kind: None,
            s: "hello 𓀕 world",
            range: 0..16
        }
    );

    Ok(())
}

#[test]
fn control_sequences_with_no_characters() -> Result<()> {
    let a_u = parse("*_")?;
    dbg!(&a_u);
    assert_eq!(
        a_u,
        &[Span {
            kind: None,
            s: "*_",
            range: 0..2,
        },]
    );

    let du = parse("__")?;
    dbg!(&du);
    assert_eq!(
        du,
        &[Span {
            kind: None,
            s: "__",
            range: 0..2,
        },]
    );

    let ta = parse("***")?;
    dbg!(&ta);
    assert_eq!(
        ta,
        &[Span {
            kind: None,
            s: "***",
            range: 0..3,
        },]
    );

    Ok(())
}

/*
    let a_du_da = parse("hello *__**world**_*_!")?;
    // a_du_da is wrong
    dbg!(&a_du_da);

    assert_eq!(
        a_du_da,
        &[Span {
            kind: None,
            s: "hello ",
            range: 0..6,
        },
            Span {
                kind: MarkdownKind::Italics,
                s: "_**world*",
                range: Default::default()
            }
        ]
    );
 */
