fn main() {
    println!("Hello, world!");
}

/// 位置情報
/// .0から.1までの区間を表す
/// たとえばLoc(4, 6)なら入力文字の5文字目から7文字目までの区間を表す（0始まり
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Loc(usize, usize);

// Locに便利メソッドを実装しておく
impl Loc {
    fn merge(&self, other: &Loc) -> Loc {
        use std::cmp::{max, min};
        Loc(min(self.0, other.0), max(self.1, other.1))
    }
}

// アノテーション。
// 値にさまざまなデータをもたせたもの。ここではLocをもたせている。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Annot<T> {
    value: T,
    loc: Loc,
}

impl<T> Annot<T> {
    fn new(value: T, loc: Loc) -> Self {
        Self { value, loc }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TokenKind {
    /// [0-9][0-9]*
    Number(u64),
    /// +
    Plus,
    /// -
    Minux,
    /// *
    Asterisk,
    /// /
    Slash,
    /// (
    LParen,
    /// )
    RParen,
}

// TokenKindにアノテーションをつけたものをTokenとして定義しておく
type Token = Annot<TokenKind>;
