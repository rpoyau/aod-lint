use aod_lint::models::diagnostics::{RuleAnchor, Severity};
use aod_lint::passes::typst_ast::TypstAstPass;

#[test]
fn reference_text_and_unicode_tail_do_not_use_a_fixed_byte_window() {
    // Byte 24 falls inside alpha. The former 24-byte lookahead panicked here.
    let source = format!("@paper {}α trailing", "a".repeat(16));
    let diagnostics = TypstAstPass::new("paper.typ", &source, false).run();
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].rule, RuleAnchor::Sec7PreciseAnchors);
    assert!(diagnostics[0].message.contains("'@paper'"));
    assert_eq!((diagnostics[0].line, diagnostics[0].column), (1, 1));
}

#[test]
fn references_report_line_starts_and_unicode_columns() {
    let diagnostics = TypstAstPass::new("paper.typ", "\n@first\nαβ @second", false).run();
    assert_eq!(diagnostics.len(), 2);
    assert_eq!((diagnostics[0].line, diagnostics[0].column), (2, 1));
    assert_eq!((diagnostics[1].line, diagnostics[1].column), (3, 4));
    assert!(diagnostics[1].message.contains("'@second'"));
}

#[test]
fn supplement_is_read_from_the_reference_ast() {
    let source = "@paper[Sec. 2, p. 4] and @other[Appendix α].";
    assert!(TypstAstPass::new("paper.typ", source, false)
        .run()
        .is_empty());
}

#[test]
fn forward_local_label_does_not_need_a_citation_supplement() {
    let source = "See @eq:balance.\n$ a = b $ <eq:balance>\n";
    assert!(TypstAstPass::new("paper.typ", source, false)
        .run()
        .is_empty());
}

#[test]
fn unknown_reference_is_not_assumed_to_be_a_bibliography_citation() {
    let diagnostics = TypstAstPass::new("paper.typ", "See @external.", true).run();
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].severity, Severity::Warning);
    assert!(diagnostics[0]
        .message
        .contains("imported label or a bibliography key"));
    assert!(!diagnostics[0].message.starts_with("Citation"));
}

#[test]
fn labels_in_comments_or_raw_examples_do_not_silence_reference_checks() {
    let source = "// <missing>\n`<missing>`\nSee @missing.\n";
    let diagnostics = TypstAstPass::new("paper.typ", source, false).run();
    assert_eq!(diagnostics.len(), 1);
    assert!(diagnostics[0].message.contains("'@missing'"));
    assert_eq!(diagnostics[0].line, 3);
}

#[test]
fn labels_inside_unexecuted_code_do_not_silence_live_reference_checks() {
    for inactive in [
        "#if false [<eq:ghost>]",
        "#let unused = [<eq:ghost>]",
        "#(() => [<eq:ghost>])",
        "#for i in () [<eq:ghost>]",
        "#while false [<eq:ghost>]",
        "#repr(<eq:ghost>)",
        "#repr([<eq:ghost>])",
    ] {
        let source = format!("{inactive}\nSee @eq:ghost.\n");
        let diagnostics = TypstAstPass::new("paper.typ", &source, false).run();
        assert_eq!(diagnostics.len(), 1, "source: {source}");
        assert!(diagnostics[0].message.contains("'@eq:ghost'"));
        assert_eq!(diagnostics[0].line, 2);
    }
}
