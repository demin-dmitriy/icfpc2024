use core::panic;
use num_bigint::BigUint;
use std::{ops::Sub, vec};

use solver;
use std::collections::HashMap;

#[derive(Clone)]
struct BooleanToken {
    value: bool,
}

#[derive(Clone)]
struct StringToken {
    value: String,
}

#[derive(Clone)]
struct IntegerToken {
    value: BigUint,
}

#[derive(Clone)]
struct VariableToken {
    value: String,
}

#[derive(Clone)]
enum UnaryOperatorTypes {
    IntegerNegation,
    BooleanNot,
    StringToInt,
    IntToString,
}
#[derive(Clone)]
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

#[derive(Clone)]
struct UnaryOperatorToken {
    value: UnaryOperatorTypes,
}

#[derive(Clone)]
struct BinaryOperatorToken {
    value: BinaryOperatorTypes,
}

#[derive(Clone)]
struct IfToken {}

#[derive(Clone)]
struct LambdaToken {
    value: String,
}

#[derive(Clone)]
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

#[derive(Clone)]
enum EvalResulToken {
    B(BooleanToken),
    I(IntegerToken),
    S(StringToken),
}

#[derive(Clone)]
struct EvaluationResult {
    value: EvalResulToken,
}

#[derive(Clone)]
struct SubstituionContext {
    value: HashMap<String, EvalResulToken>,
}

fn minus_integer(x: IntegerToken) -> IntegerToken {
    return IntegerToken {
        value: BigUint::ZERO.sub(x.value),
    };
}

fn boolean_not(x: BooleanToken) -> BooleanToken {
    return BooleanToken { value: !x.value };
}

fn from_big_int_to_string(x: BigUint) -> String {
    return x.to_string();
}

fn from_string_to_big_int(x: String) -> BigUint {
    return BigUint::parse_bytes(x.as_bytes(), 10).unwrap();
    // let f = x.as_bytes().into_iter().map(|x| x - ('!' as u8)).collect::<Vec<u8>>();
    // return BigUint::parse_bytes(f.as_slice(), 94).unwrap();
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

fn parse(tokens: &Vec<Token>, start: usize, context: &SubstituionContext) -> EvaluationResult {
    // println!("{}", start);
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
                    let Token::L(next_token) = tokens.get(start + 1).unwrap() else {
                        panic!()
                    };

                    let partial_result_right = parse(tokens, define_term_end(tokens, start + 2), context);

                    let mut new_context = context.clone();
                    new_context
                        .value
                        .insert(next_token.value.clone(), partial_result_right.value);

                    let partial_result_left = parse(tokens, start + 2, &new_context);

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
        Token::L(_) => {
            panic!()
        }
    }
}

fn main() {
    let input = "B$ L# B$ L\" B+ v\" v\" B* I1 I2 v8";

    let mut tokens: Vec<Token> = vec![];

    for lexema in input.split(' ') {
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

    let context = SubstituionContext {
        value: HashMap::new(),
    };

    match parse(&tokens, 0, &context).value {
        EvalResulToken::B(b) => {
            println!("{}", b.value);
        },
        EvalResulToken::I(i) => {
            println!("{}", i.value.to_string());
        }
        EvalResulToken::S(s) => {
            println!("{}", s.value);
        }
    }
}
