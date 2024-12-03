use std::fs::read_to_string;

fn load_input() -> Result<String, Box<dyn std::error::Error>> {
    Ok(read_to_string("./inputs/dec03.txt")?)
}

#[derive(Debug)]
enum Token {
    MUL,
    OPEN,
    COMMA,
    CLOSE,
    NUMBER(u16),
    JUNK,
}

// now, we need to tokenize.
// tokenization definition is:
// `mul` -> MUL
// `(` -> OPEN_BRACKET
// `,` -> COMMA
// `)` -> CLOSE_BRACKET
// `\d+` -> NUMBER
// else -> discarded.
fn tokenize(stream: &[u8]) -> Vec<Token> {
    let mut tokens_out = vec![];

    let mut i = 0_usize;
    while i < stream.len() {
        // try MUL token
        let x = stream.get(i..i + 3);
        if x.is_some() && x.unwrap() == b"mul" {
            tokens_out.push(Token::MUL);
            i += 3;
            continue;
        }
        match stream[i] {
            b'0'..=b'9' => {
                let mut j = i + 1;
                while j < stream.len() && (b'0'..=b'9').contains(&stream[j]) {
                    j += 1;
                }
                if let Ok(string) = std::str::from_utf8(&stream[i..j]) {
                    if let Ok(number) = string.parse() {
                        tokens_out.push(Token::NUMBER(number));
                        i = j;
                        continue;
                    }
                }
            }
            b'(' => tokens_out.push(Token::OPEN),
            b')' => tokens_out.push(Token::CLOSE),
            b',' => tokens_out.push(Token::COMMA),
            _ => tokens_out.push(Token::JUNK),
        }
        i += 1;
    }

    tokens_out
}

static DO_SYMBOL: &[u8] = "do()".as_bytes();
static DONT_SYMBOL: &[u8] = "don't()".as_bytes();
static MUL_SYMBOL: &[u8] = "mul".as_bytes();

static DO_LEN: usize = DO_SYMBOL.len();
static DONT_LEN: usize = DONT_SYMBOL.len();
static MUL_LEN: usize = MUL_SYMBOL.len();

fn do_dont_preproc_tokenize(stream: &[u8]) -> Vec<Token> {
    let mut accepting = true;
    let mut tokens_out = vec![];

    let mut i = 0_usize;
    while i < stream.len() {
        if !accepting {
            let x = stream.get(i..i + DO_LEN);
            if x.is_some() && x.unwrap() == DO_SYMBOL {
                accepting = true;
                i += DO_LEN;
                continue;
            }
        } else {
            // try don't() preproc
            let x = stream.get(i..i + DONT_LEN);
            if x.is_some() && x.unwrap() == DONT_SYMBOL {
                accepting = false;
                i += DONT_LEN;
                continue;
            }

            // try MUL token
            let x = stream.get(i..i + 3);
            if x.is_some() && x.unwrap() == MUL_SYMBOL {
                tokens_out.push(Token::MUL);
                i += MUL_LEN;
                continue;
            }
            match stream[i] {
                b'0'..=b'9' => {
                    let mut j = i + 1;
                    while j < stream.len() && (b'0'..=b'9').contains(&stream[j]) {
                        j += 1;
                    }
                    if let Ok(string) = std::str::from_utf8(&stream[i..j]) {
                        if let Ok(number) = string.parse() {
                            tokens_out.push(Token::NUMBER(number));
                            i = j;
                            continue;
                        }
                    }
                }
                b'(' => tokens_out.push(Token::OPEN),
                b')' => tokens_out.push(Token::CLOSE),
                b',' => tokens_out.push(Token::COMMA),
                _ => tokens_out.push(Token::JUNK),
            }
        }

        i += 1;
    }

    tokens_out
}

enum Expression {
    MUL(u16, u16),
}

fn parse(tokens: Vec<Token>) -> Vec<Expression> {
    let mut expr_out = vec![];

    let mut i = 0;
    while i < tokens.len() {
        let num_a;
        let num_b;
        while i < tokens.len() && !matches!(tokens[i], Token::MUL) {
            i += 1;
        }
        i += 1;
        if i >= tokens.len() || !matches!(tokens[i], Token::OPEN) {
            continue;
        }
        i += 1;
        if i >= tokens.len() {
            continue;
        }
        match tokens[i] {
            Token::NUMBER(a) => num_a = a,
            _ => continue,
        }
        i += 1;
        if i >= tokens.len() || !matches!(tokens[i], Token::COMMA) {
            continue;
        }
        i += 1;
        if i >= tokens.len() {
            continue;
        }
        match tokens[i] {
            Token::NUMBER(b) => num_b = b,
            _ => continue,
        }
        i += 1;
        if i >= tokens.len() || !matches!(tokens[i], Token::CLOSE) {
            continue;
        }

        expr_out.push(Expression::MUL(num_a, num_b))
    }

    expr_out
}

fn evaluate_sum(exprs: Vec<Expression>) -> u32 {
    let mut sum = 0;
    for expr in exprs {
        match expr {
            Expression::MUL(a, b) => sum += a as u32 * b as u32,
        }
    }

    sum
}

pub fn puzzle1() -> Result<u32, Box<dyn std::error::Error>> {
    let input_stream = load_input()?;
    let tokens = tokenize(input_stream.as_bytes());
    let expressions = parse(tokens);
    Ok(evaluate_sum(expressions))
}
pub fn puzzle2() -> Result<u32, Box<dyn std::error::Error>> {
    let input_stream = load_input()?;
    let tokens = do_dont_preproc_tokenize(input_stream.as_bytes());
    let expressions = parse(tokens);
    Ok(evaluate_sum(expressions))
}

#[test]
fn test_parser() {
    let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let tokens = tokenize(test_input.as_bytes());
    let expressions = parse(tokens);
    assert_eq!(evaluate_sum(expressions), 161);
}

#[test]
fn test_parse_do_dont() {
    let test_input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let tokens = do_dont_preproc_tokenize(test_input.as_bytes());
    let expressions = parse(tokens);
    assert_eq!(evaluate_sum(expressions), 48);
}
