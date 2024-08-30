#[derive(Debug)]
pub enum TokenKind {
    // GENERAL
    TkEof,

    // VALUES
    TkIdent,
    TkNumber,
}
