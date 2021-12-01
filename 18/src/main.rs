use std::fs::read_to_string;
use nom::IResult;
use nom::sequence::delimited;
use nom::bytes::complete::{is_not, is_a, take_while};
use nom::lib::std::mem::take;
use nom::character::{is_digit, is_space};
use nom::multi::{many1, many0};
use nom::branch::alt;
use nom::character::complete::{one_of, char, multispace0, digit1};
use nom::combinator::opt;

fn main() {
    let input_string = read_to_string("input.txt").unwrap();
    let parsed_exprssions = parse_expressions(&input_string);
    println!("{:?}", parsed_exprssions);
    let result: u64 = parsed_exprssions.iter().map(|e| e.calculate()).sum();
    println!("{}", result);
}

#[derive(Debug)]
enum Expression {
    Plus(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Constant(u32)
}

impl Expression {
    fn calculate(&self) -> u64 {
        match self {
            Expression::Plus(left, right) =>
                left.calculate() + right.calculate(),
            Expression::Mul(left, right) =>
                left.calculate() * right.calculate(),
            Expression::Constant(number) => *number as u64
        }
    }
}

fn parse_expressions(input: &str) -> Vec<Expression> {
    input.split("\n")
        .filter(|e| !e.is_empty())
        .map(|e| parse_expression(e).unwrap().1)
        .collect()
}

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    let (input_rest, left) = parse_term(input)?;
    let (input_rest, sp) = multispace0(input_rest)?;
    parse_mul_and_right(left, input_rest)
}

fn parse_mul_and_right(left: Expression, input: &str) -> IResult<&str, Expression> {
    let (input_rest, operator) = opt(char('*'))(input)?;
    if operator.is_some() {
        let (input_rest, _) = multispace0(input_rest)?;
        let (input_rest, next) = parse_term(input_rest)?;
        let new_left = Expression::Mul(Box::new(left), Box::new(next));
        let (input_rest, _) = multispace0(input_rest)?;
        parse_mul_and_right(new_left, input_rest)
    } else {
        IResult::Ok((input_rest, left))
    }
}

fn parse_term(input: &str) -> IResult<&str, Expression> {
    let (input_rest, left) = parse_factor(input)?;
    let (input_rest, sp) = multispace0(input_rest)?;
    parse_plus_and_right(left, input_rest)
}

fn parse_plus_and_right(left: Expression, input: &str) -> IResult<&str, Expression> {
    let (input_rest, operator) = opt(char('+'))(input)?;
    if operator.is_some() {
        let (input_rest, _) = multispace0(input_rest)?;
        let (input_rest, next) = parse_factor(input_rest)?;
        let new_left = Expression::Plus(Box::new(left), Box::new(next));
        let (input_rest, _) = multispace0(input_rest)?;
        parse_plus_and_right(new_left, input_rest)
    } else {
        IResult::Ok((input_rest, left))
    }
}

fn parse_factor(input: &str) -> IResult<&str, Expression> {
    alt((parse_number, delimited(char('('), parse_expression, char(')'))))(input)
}

fn parse_number(input: &str) -> IResult<&str, Expression> {
    let (input_rest, left_str) = digit1(input)?;
    let number = Expression::Constant(left_str.parse::<u32>().unwrap());
    IResult::Ok((input_rest, number))
}