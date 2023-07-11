## clexer

The clexer is a simple program that tokenizes a Hello world Program (written in c) into individual tokens. It recognizes keywords, identifiers, delimiters, literals, and other token types commonly found in the C programming language.

### Executing

```bash
cargo run -- tokenize hello.c
```

### Expected output

```bash
Token { kind: Hash, literal: "#" }
Token { kind: Identifier, literal: "include" }
Token { kind: LessThan, literal: "<" }
Token { kind: Identifier, literal: "stdio" }
Token { kind: Identifier, literal: "h" }
Token { kind: Int, literal: "int" }
Token { kind: Identifier, literal: "main" }
Token { kind: OpenParen, literal: "(" }
Token { kind: CloseParen, literal: ")" }
Token { kind: OpenCurly, literal: "{" }
Token { kind: Identifier, literal: "printf" }
Token { kind: OpenParen, literal: "(" }
Token { kind: StringLiteral, literal: "\"Hello, world!\"" }
Token { kind: CloseParen, literal: ")" }
Token { kind: Semicolon, literal: ";" }
Token { kind: Return, literal: "return" }
Token { kind: NumericLiteral, literal: "0" }
Token { kind: CloseCurly, literal: "}" }
Token { kind: Eof, literal: "" }
```
