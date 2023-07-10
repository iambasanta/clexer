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

}

fn main() {
    let content = "#include<stdio.h>
        int main(){
        printf('Hello world');
        return 0;
    } ".to_string();

    let mut lexer = Lexer::new(content);

    loop {
        let token = lexer.next_token();

        match token.kind {
            TokenKind::Eof => {
                println!("{:?}", token);
                break; // Terminate the loop when end-of-file token is encountered
            }
            _ => println!("{:?}", token),
        }
    }
}
