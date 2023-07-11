use  std::{fs, env};

#[derive(Debug)]
enum TokenKind {
    // Keywords
    Int,
    Return,
    Identifier,

    // Delimeters
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    Semicolon,

    // Literals
    NumericLiteral,

    // Others
    Eof,
    Unknown,
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    literal: String
}

impl Token {
    fn new(kind:TokenKind, literal:String) -> Self {
        Self { kind, literal }
    }
}

#[derive(Debug)]
struct Lexer {
    source: Vec<char>,
    cursor: usize,
    char: char
}

impl Lexer {
    fn new(source: String) -> Self {
        let mut lex = Self {
            source : source.chars().collect(),
            cursor: 0 ,
            char: '0',
        };

        lex.read_char();

        return lex;
    } 

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.char {
            '(' => Token::new(TokenKind::OpenParen, self.char.to_string()),
            ')' => Token::new(TokenKind::CloseParen, self.char.to_string()),
            '{' => Token::new(TokenKind::OpenCurly, self.char.to_string()),
            '}' => Token::new(TokenKind::CloseCurly, self.char.to_string()),
            ';' => Token::new(TokenKind::Semicolon, self.char.to_string()),
            '\0' => Token::new(TokenKind::Eof, String::new()),
            '0'..='9' => self.read_numeric_literal(),
            _ => Token::new(TokenKind::Unknown, self.char.to_string())
        };

        self.read_char();

        token
    }

    fn read_char(&mut self) {
        if self.cursor >= self.source.len() {
            self.char = '\0';
        } else {
            self.char = self.source[self.cursor];
            self.cursor += 1;
        }
    }

    fn skip_whitespace(&mut self) {
        while self.char.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_numeric_literal(&mut self) -> Token {
        let mut literal = String::new();

        while self.char.is_digit(10) {
            literal.push(self.char);
            self.read_char();
        }

        Token::new(TokenKind::NumericLiteral, literal)

    }

}

fn main() {

    let args: Vec<String> = env::args().collect();

    let command = &args[1];
    let file_path = &args[2];

    let source = fs::read_to_string(file_path).expect("Can't read this file.");

    let mut lexer = Lexer::new(source);

    loop {
        let token = lexer.next_token();

        match token.kind {
            TokenKind::Eof => {
                println!("{:?}", token);
                break;
            }
            _ => println!("{:?}", token),
        }
    }
}
