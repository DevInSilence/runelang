#[derive(Debug, Clone)]
pub enum TokenKind {
    // GENERAL
    TkEof,

    // VALUES
    TkIdent,
    TkNumber,
}
