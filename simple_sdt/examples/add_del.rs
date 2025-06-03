use std::str::Chars;

struct Parser<'a> {
    chars: Chars<'a>,
    lookahead: Option<char>,
}

impl <'a> Parser<'a> {
    fn new(input: &'static str) -> Self {
        let mut chars = input.chars();
        let lookahead = chars.next();

        let mut parser = Parser {chars, lookahead};
        parser.skip_white_space();
        parser
    }

    fn peek(&self) -> Option<char> {
        self.lookahead
    }

    fn consume(&mut self) {
        self.lookahead = self.chars.next();
    }

    fn skip_white_space(&mut self) {
        loop {
            if let Some(blank) = self.peek() {
                match blank {
                    ' ' => {
                        self.consume()
                    }
                    _ => break
                }
            } else {
                break;
            }
        }
    }

    // match long digit
    fn term(&mut self) -> Result<i32, String> {
        let mut long_digit = String::new();

        // collect digits
        loop {
            if let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    long_digit.push(c);

                    // change lookahead with the start char of chars
                    self.consume();
                } else {
                    // since there is no digit
                    break;
                }
            } else {
                // reach the end of chars
                break;
            }
        }

        // parse
        if long_digit.is_empty() {
            Err(format!("Syntax Error: expected a digit, found none"))
        } else {
            long_digit.parse::<i32>()
                .map_err(|e| format!("Failed to parse number: {}", e))
        }
    }

    // expr --- grasp the result
    fn expr(&mut self) -> Result<i32, String> {
        let mut res = self.term()?;

        loop {
            self.skip_white_space();
            if let Some(oper) = self.peek() {
                match oper {
                    '+' => {
                        self.consume();
                        self.skip_white_space();
                        let next_term = self.term()?;
                        res = res + next_term;
                    }
                    '-' => {
                        self.consume();
                        self.skip_white_space();
                        let next_term = self.term()?;
                        res = res - next_term;
                    }
                    _ => {return Err(format!("Syntex Error: Expected an operator ('+'m '-') found unexpected character"));}
                }
            } else {
                break;
            }
        }
        Ok(res)
    }
}

fn main() {}

#[cfg(test)]
mod test {
use super::*;

    #[test]
    fn unexpected_character() {
        let mut parser2 = Parser::new("abc");
        match parser2.term() {
            Ok(num) => println!("Parsed number: {}", num),
            Err(e) => println!("Error: {}", e), // Expected: Error: Expected a digit or a number, but found none or unexpected character
        }
    }

    #[test]
    fn lack_of_digit() {
        let mut parser = Parser::new("12+");
        match parser.expr() {
            Ok(num) => println!("Parsed number: {}", num),
            Err(e) => println!("Error: {}", e),
        }
    }

    #[test]
    fn empty() {
        let mut parser = Parser::new("");
        match parser.expr() {
            Ok(num) => println!("Parsed number: {}", num),
            Err(e) => println!("Error: {}", e),
        }
    }

    #[test]
    fn normal() {
        let mut parser = Parser::new("123+2345");
        match parser.expr() {
            Ok(num) => println!("Parsed number: {}", num),
            Err(e) => println!("Error: {}", e),
        }
    }

    #[test]
    fn with_blank() {
        let mut parser = Parser::new("  124 -     904    ");
        match parser.expr() {
            Ok(num) => println!("Parsed number: {}", num),
            Err(e) => println!("Error: {}", e),
        }
    }
}