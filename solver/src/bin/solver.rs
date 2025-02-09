use core::panic;
use clap::Parser;
use num_bigint::BigInt;
use std::{ops::Sub, vec};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct BooleanToken {
    value: bool,
}

#[derive(Clone, Debug)]
struct StringToken {
    value: String,
}

#[derive(Clone, Debug)]
struct IntegerToken {
    value: BigInt,
}

#[derive(Clone, Debug)]
struct VariableToken {
    value: String,
}

#[derive(Clone, Debug)]
enum UnaryOperatorTypes {
    IntegerNegation,
    BooleanNot,
    StringToInt,
    IntToString,
}
#[derive(Clone, Debug)]
enum BinaryOperatorTypes {
    IntegerAddition,
    IntegerSubtraction,
    IntegerMultiplication,
    IntegerDivision,
    IntegerModulo,
    IntegerComparisonLess,
    IntegerComparisonMore,
    EqualityComparison,
    BooleanOr,
    BooleanAnd,
    StringConcatenation,
    TakeFirstXCharsOfStringY,
    DropFirstXCharsOfStringY,
    ApplyTermXToY,
}

#[derive(Clone, Debug)]
struct UnaryOperatorToken {
    value: UnaryOperatorTypes,
}

#[derive(Clone, Debug)]
struct BinaryOperatorToken {
    value: BinaryOperatorTypes,
}

#[derive(Clone, Debug)]
struct IfToken {}

#[derive(Clone, Debug)]
struct LambdaToken {
    value: String,
}

#[derive(Clone, Debug)]
enum Token {
    B(BooleanToken),
    I(IntegerToken),
    S(StringToken),
    U(UnaryOperatorToken),
    Bi(BinaryOperatorToken),
    If(IfToken),
    L(LambdaToken),
    V(VariableToken),
}

#[derive(Clone, Debug)]
enum EvalResulToken {
    B(BooleanToken),
    I(IntegerToken),
    S(StringToken),
}

#[derive(Clone, Debug)]
struct EvaluationResult {
    value: EvalResulToken,
}

#[derive(Clone, Debug)]
struct SubstituionContext {
    value: HashMap<String, EvalResulToken>,
    order: Vec<EvaluationResult>
}

fn minus_integer(x: IntegerToken) -> IntegerToken {
    return IntegerToken {
        value: BigInt::ZERO.sub(x.value),
    };
}

fn boolean_not(x: BooleanToken) -> BooleanToken {
    return BooleanToken { value: !x.value };
}

fn from_big_int_to_string(x: BigInt) -> String {
    let mut val = x.clone();
    let mut res = String::new();
    if val == BigInt::ZERO {
        return "a".to_string();
    }
    if val < BigInt::ZERO {
        panic!()
    }

    while val > BigInt::ZERO {
        let tmp = (&val % BigInt::from(94u32)).to_u32_digits().1.get(0).unwrap().clone();
        val = val / BigInt::from(94u32);
        res.push(((tmp + '!' as u32) as u8) as char);
    }

    return res.chars().rev().collect();
}

fn from_string_to_big_int(x: String) -> BigInt {
    let mut val = BigInt::from(0u32);
    for &ch in x.as_bytes() {
        if ch == b'-' {
            panic!()
        }
        val *= BigInt::from(94u32);
        val += BigInt::from(ch - b'!');
    }
    return val;
}

fn int_to_string(x: IntegerToken) -> StringToken {
    return StringToken {
        value: from_big_int_to_string(x.value),
    };
}

fn string_to_int(x: StringToken) -> IntegerToken {
    return IntegerToken {
        value: from_string_to_big_int(x.value),
    };
}

fn integer_addition(x: EvaluationResult, y: EvaluationResult) -> EvaluationResult {
    let (EvalResulToken::I(left), EvalResulToken::I(right)) = (x.value, y.value) else {
        panic!("Not numbers!");
    };
    return EvaluationResult {
        value: EvalResulToken::I(IntegerToken {
            value: left.value + right.value,
        }),
    };
}

fn integer_subtraction(x: EvaluationResult, y: EvaluationResult) -> EvaluationResult {
    let (EvalResulToken::I(left), EvalResulToken::I(right)) = (x.value, y.value) else {
        panic!("Not numbers!");
    };
    return EvaluationResult {
        value: EvalResulToken::I(IntegerToken {
            value: left.value - right.value,
        }),
    };
}

fn boolean_and(x: EvaluationResult, y: EvaluationResult) -> EvaluationResult {
    let (EvalResulToken::B(left), EvalResulToken::B(right)) = (x.value, y.value) else {
        panic!("Not numbers!");
    };
    return EvaluationResult {
        value: EvalResulToken::B(BooleanToken {
            value: left.value && right.value,
        }),
    };
}

fn boolean_or(x: EvaluationResult, y: EvaluationResult) -> EvaluationResult {
    let (EvalResulToken::B(left), EvalResulToken::B(right)) = (x.value, y.value) else {
        panic!("Not numbers!");
    };
    return EvaluationResult {
        value: EvalResulToken::B(BooleanToken {
            value: left.value || right.value,
        }),
    };
}

fn interger_multiplication(x: EvaluationResult, y: EvaluationResult) -> EvaluationResult {
    let (EvalResulToken::I(left), EvalResulToken::I(right)) = (x.value, y.value) else {
        panic!();
    };
    return EvaluationResult {
        value: EvalResulToken::I(IntegerToken {
            value: left.value * right.value,
        }),
    };
}

fn integer_division(x: EvaluationResult, y: EvaluationResult) -> EvaluationResult {
    let (EvalResulToken::I(left), EvalResulToken::I(right)) = (x.value, y.value) else {
        panic!();
    };
    return EvaluationResult {
        value: EvalResulToken::I(IntegerToken {
            value: left.value / right.value,
        }),
    };
}

fn integer_modulo(x: EvaluationResult, y: EvaluationResult) -> EvaluationResult {
    let (EvalResulToken::I(left), EvalResulToken::I(right)) = (x.value, y.value) else {
        panic!();
    };
    return EvaluationResult {
        value: EvalResulToken::I(IntegerToken {
            value: left.value % right.value,
        }),
    };
}

fn integer_comparison_less(x: EvaluationResult, y: EvaluationResult) -> EvaluationResult {
    let (EvalResulToken::I(left), EvalResulToken::I(right)) = (x.value, y.value) else {
        panic!();
    };
    return EvaluationResult {
        value: EvalResulToken::B(BooleanToken {
            value: left.value < right.value,
        }),
    };
}

fn equality_comparison(x: EvaluationResult, y: EvaluationResult) -> EvaluationResult {
    let (EvalResulToken::I(left1), EvalResulToken::I(right1)) = (&x.value, &y.value) else {
        let (EvalResulToken::B(left2), EvalResulToken::B(right2)) = (&x.value, &y.value) else {
            let (EvalResulToken::S(left3), EvalResulToken::S(right3)) = (&x.value, &y.value) else {
                panic!();
            };
            return EvaluationResult {
                value: EvalResulToken::B(BooleanToken {
                    value: left3.value == right3.value,
                }),
            };
        };
        return EvaluationResult {
            value: EvalResulToken::B(BooleanToken {
                value: left2.value == right2.value,
            }),
        };
    };

    return EvaluationResult {
        value: EvalResulToken::B(BooleanToken {
            value: left1.value == right1.value,
        }),
    };
}

fn integer_comparison_more(x: EvaluationResult, y: EvaluationResult) -> EvaluationResult {
    let (EvalResulToken::I(left), EvalResulToken::I(right)) = (x.value, y.value) else {
        panic!();
    };
    return EvaluationResult {
        value: EvalResulToken::B(BooleanToken {
            value: left.value > right.value,
        }),
    };
}

fn string_concatenation(x: EvaluationResult, y: EvaluationResult) -> EvaluationResult {
    let (EvalResulToken::S(left), EvalResulToken::S(right)) = (x.value, y.value) else {
        panic!();
    };
    return EvaluationResult {
        value: EvalResulToken::S(StringToken {
            value: left.value + right.value.as_str(),
        }),
    };
}

fn take_first_x_chars_of_string_y(x: EvaluationResult, y: EvaluationResult) -> EvaluationResult {
    let (EvalResulToken::I(left), EvalResulToken::S(right)) = (x.value, y.value) else {
        panic!();
    };
    let n = left.value.to_string().parse().unwrap();
    return EvaluationResult {
        value: EvalResulToken::S(StringToken {
            value: right.value.as_str()[..n].to_string(),
        }),
    };
}

fn drop_first_x_chars_of_string_y(x: EvaluationResult, y: EvaluationResult) -> EvaluationResult {
    let (EvalResulToken::I(left), EvalResulToken::S(right)) = (x.value, y.value) else {
        panic!();
    };
    let n = left.value.to_string().parse().unwrap();
    return EvaluationResult {
        value: EvalResulToken::S(StringToken {
            value: right.value.as_str()[n..].to_string(),
        }),
    };
}

fn define_term_end(tokens: &Vec<Token>, start: usize) -> usize {
    return match tokens.get(start).unwrap() {
        Token::B(_) => start + 1,
        Token::I(_) => start + 1,
        Token::S(_) => start + 1,
        Token::U(_) => define_term_end(tokens, start + 1),
        Token::Bi(_) => {
            let first_end = define_term_end(tokens, start + 1);
            let second_end = define_term_end(tokens, first_end);
            second_end
        }
        Token::If(_) => {
            let first_end = define_term_end(tokens, start + 1);
            let second_end = define_term_end(tokens, first_end);
            let third_end = define_term_end(tokens, second_end);
            third_end
        }
        Token::L(_) => define_term_end(tokens, start + 1),
        Token::V(_) => start + 1,
    };
}

fn parse(tokens: &Vec<Token>, start: usize, context: &mut SubstituionContext) -> EvaluationResult {
    match tokens.get(start).unwrap() {
        Token::B(value) => {
            return EvaluationResult {
                value: EvalResulToken::B(value.clone()),
            }
        }
        Token::I(value) => {
            return EvaluationResult {
                value: EvalResulToken::I(value.clone()),
            }
        }
        Token::S(value) => {
            return EvaluationResult {
                value: EvalResulToken::S(value.clone()),
            }
        }
        Token::U(value) => {
            let partial_result = parse(tokens, start + 1, context);
            return EvaluationResult {
                value: match value.value {
                    UnaryOperatorTypes::IntegerNegation => match partial_result.value {
                        EvalResulToken::I(x) => EvalResulToken::I(minus_integer(x)),
                        _ => panic!(""),
                    },
                    UnaryOperatorTypes::BooleanNot => match partial_result.value {
                        EvalResulToken::B(x) => EvalResulToken::B(boolean_not(x)),
                        _ => panic!(""),
                    },
                    UnaryOperatorTypes::IntToString => match partial_result.value {
                        EvalResulToken::I(x) => EvalResulToken::S(int_to_string(x)),
                        _ => panic!(""),
                    },
                    UnaryOperatorTypes::StringToInt => match partial_result.value {
                        EvalResulToken::S(x) => EvalResulToken::I(string_to_int(x)),
                        _ => panic!(""),
                    },
                },
            };
        }
        Token::Bi(value) => {
            return match value.value {
                BinaryOperatorTypes::IntegerAddition => {
                    let partial_result_left = parse(tokens, start + 1, context);
                    let partial_result_right =
                        parse(tokens, define_term_end(tokens, start + 1), context);
                    integer_addition(partial_result_left, partial_result_right)
                }
                BinaryOperatorTypes::IntegerSubtraction => {
                    let partial_result_left = parse(tokens, start + 1, context);
                    let partial_result_right =
                        parse(tokens, define_term_end(tokens, start + 1), context);
                    integer_subtraction(partial_result_left, partial_result_right)
                }
                BinaryOperatorTypes::IntegerMultiplication => {
                    let partial_result_left = parse(tokens, start + 1, context);
                    let partial_result_right =
                        parse(tokens, define_term_end(tokens, start + 1), context);
                    interger_multiplication(partial_result_left, partial_result_right)
                }
                BinaryOperatorTypes::IntegerDivision => {
                    let partial_result_left = parse(tokens, start + 1, context);
                    let partial_result_right =
                        parse(tokens, define_term_end(tokens, start + 1), context);
                    integer_division(partial_result_left, partial_result_right)
                }
                BinaryOperatorTypes::IntegerModulo => {
                    let partial_result_left = parse(tokens, start + 1, context);
                    let partial_result_right =
                        parse(tokens, define_term_end(tokens, start + 1), context);
                    integer_modulo(partial_result_left, partial_result_right)
                }
                BinaryOperatorTypes::IntegerComparisonLess => {
                    let partial_result_left = parse(tokens, start + 1, context);
                    let partial_result_right =
                        parse(tokens, define_term_end(tokens, start + 1), context);
                    integer_comparison_less(partial_result_left, partial_result_right)
                }
                BinaryOperatorTypes::IntegerComparisonMore => {
                    let partial_result_left = parse(tokens, start + 1, context);
                    let partial_result_right =
                        parse(tokens, define_term_end(tokens, start + 1), context);
                    integer_comparison_more(partial_result_left, partial_result_right)
                }
                BinaryOperatorTypes::EqualityComparison => {
                    let partial_result_left = parse(tokens, start + 1, context);
                    let partial_result_right =
                        parse(tokens, define_term_end(tokens, start + 1), context);
                    equality_comparison(partial_result_left, partial_result_right)
                }
                BinaryOperatorTypes::BooleanOr => {
                    // MAY BE LAZY?
                    let partial_result_left = parse(tokens, start + 1, context);
                    let partial_result_right =
                        parse(tokens, define_term_end(tokens, start + 1), context);
                    boolean_or(partial_result_left, partial_result_right)
                }
                BinaryOperatorTypes::BooleanAnd => {
                    // MAY BE LAZY?
                    let partial_result_left = parse(tokens, start + 1, context);
                    let partial_result_right =
                        parse(tokens, define_term_end(tokens, start + 1), context);

                    boolean_and(partial_result_left, partial_result_right)
                }
                BinaryOperatorTypes::StringConcatenation => {
                    let partial_result_left = parse(tokens, start + 1, context);
                    let partial_result_right =
                        parse(tokens, define_term_end(tokens, start + 1), context);
                    string_concatenation(partial_result_left, partial_result_right)
                }
                BinaryOperatorTypes::TakeFirstXCharsOfStringY => {
                    let partial_result_left = parse(tokens, start + 1, context);
                    let partial_result_right =
                        parse(tokens, define_term_end(tokens, start + 1), context);
                    take_first_x_chars_of_string_y(partial_result_left, partial_result_right)
                }
                BinaryOperatorTypes::DropFirstXCharsOfStringY => {
                    let partial_result_left = parse(tokens, start + 1, context);
                    let partial_result_right =
                        parse(tokens, define_term_end(tokens, start + 1), context);
                    drop_first_x_chars_of_string_y(partial_result_left, partial_result_right)
                }
                BinaryOperatorTypes::ApplyTermXToY => {
                    let partial_result_right =
                        parse(tokens, define_term_end(tokens, start + 1), context);

                    let mut new_context = context.clone();
                    new_context.order.push(partial_result_right);

                    let partial_result_left = parse(tokens, start + 1, &mut new_context);

                    return partial_result_left;
                }
            };
        }
        Token::If(_) => {
            let cond_start = start + 1;
            let left_start = define_term_end(tokens, cond_start);
            let right_start = define_term_end(tokens, left_start);

            let partial_result_cond = parse(tokens, cond_start, context);

            return match partial_result_cond.value {
                EvalResulToken::B(cond) => {
                    if cond.value {
                        parse(tokens, left_start, context)
                    } else {
                        parse(tokens, right_start, context)
                    }
                }
                _ => panic!("Not boolean"),
            };
        }
        Token::V(value) => {
            return EvaluationResult {
                value: context.value.get(&value.value).unwrap().clone(),
            };
        }
        Token::L(value) => {
            let f = &mut context.order;
            context.value.insert(value.value.clone(), f.pop().unwrap().value);
            return parse(tokens, start + 1, context);
        }
    }
}


#[derive(Parser, Debug)]
struct Cli {
    pub path: String,
}


fn main() {
    let cli = Cli::parse();

    let path = Path::new(&cli.path);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut input = String::new();
    match file.read_to_string(&mut input) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, input),
    }

    let mut tokens: Vec<Token> = vec![];

    for lexema in input.trim().split(' ') {
        match lexema.chars().nth(0).unwrap() {
            'T' => {
                tokens.push(Token::B(BooleanToken { value: true }));
            }
            'F' => {
                tokens.push(Token::B(BooleanToken { value: false }));
            }
            'I' => {
                tokens.push(Token::I(IntegerToken {
                    value: from_string_to_big_int(lexema.to_string()[1..].to_string()),
                }));
            }
            'S' => {
                tokens.push(Token::S(StringToken {
                    value: lexema.to_string()[1..].to_string(),
                }));
            }
            'U' => {
                tokens.push(Token::U(UnaryOperatorToken {
                    value: match lexema.chars().nth(1).unwrap() {
                        '-' => UnaryOperatorTypes::IntegerNegation,
                        '!' => UnaryOperatorTypes::BooleanNot,
                        '#' => UnaryOperatorTypes::StringToInt,
                        '$' => UnaryOperatorTypes::IntToString,
                        _ => panic!("Uknown unary operator"),
                    },
                }));
            }
            'B' => {
                tokens.push(Token::Bi(BinaryOperatorToken {
                    value: match lexema.chars().nth(1).unwrap() {
                        '+' => BinaryOperatorTypes::IntegerAddition,
                        '-' => BinaryOperatorTypes::IntegerSubtraction,
                        '*' => BinaryOperatorTypes::IntegerMultiplication,
                        '/' => BinaryOperatorTypes::IntegerDivision,
                        '%' => BinaryOperatorTypes::IntegerModulo,
                        '<' => BinaryOperatorTypes::IntegerComparisonLess,
                        '>' => BinaryOperatorTypes::IntegerComparisonMore,
                        '=' => BinaryOperatorTypes::EqualityComparison,
                        '|' => BinaryOperatorTypes::BooleanOr,
                        '&' => BinaryOperatorTypes::BooleanAnd,
                        '.' => BinaryOperatorTypes::StringConcatenation,
                        'T' => BinaryOperatorTypes::TakeFirstXCharsOfStringY,
                        'D' => BinaryOperatorTypes::DropFirstXCharsOfStringY,
                        '$' => BinaryOperatorTypes::ApplyTermXToY,
                        _ => panic!("Uknown binary operator"),
                    },
                }));
            }
            '?' => {
                tokens.push(Token::If(IfToken {}));
            }
            'L' => tokens.push(Token::L(LambdaToken {
                value: lexema.to_string()[1..].to_string(),
            })),
            'v' => tokens.push(Token::V(VariableToken {
                value: lexema.to_string()[1..].to_string(),
            })),
            _ => panic!("Unkown token"),
        }
    }

    let mut context = SubstituionContext {
        value: HashMap::new(),
        order: Vec::new()
    };

    println!("=======================================");

    match parse(&tokens, 0, &mut context).value {
        EvalResulToken::B(b) => {
            println!("B{}", b.value);
        }
        EvalResulToken::I(i) => {
            println!("I{}", from_big_int_to_string(i.value));
        }
        EvalResulToken::S(s) => {
            println!("S{}", s.value);
        }
    }
}
