use std::{
    fs::{self, File, OpenOptions},
    io::Read,
    str::Chars,
};

#[derive(PartialEq, Eq)]
enum States {
    Out,
    Operator,
    Comma,
    Operand1,
    Operand2,
}

pub fn day3_1() -> u64 {
    let mut file: Vec<u8> = Vec::new();
    let mut state = States::Out;
    let _ = File::open("src/data/day3.txt")
        .unwrap()
        .read_to_end(&mut file)
        .unwrap_or_default();
    let n = file.len();
    let mut i = 0;
    let mut num1: u64 = 0;
    let mut num2: u64 = 0;
    let mut res: u64 = 0;

    while i < n {
        match state {
            States::Out => {
                if i + 3 < n
                    && file[i] as char == 'm'
                    && file[i + 1] as char == 'u'
                    && file[i + 2] as char == 'l'
                    && file[i + 3] as char == '('
                {
                    state = States::Operator;
                    i += 3;
                }
            }

            States::Operator => match file[i] as char {
                d @ '0'..='9' => {
                    num1 = d.to_digit(10).unwrap() as u64;
                    state = States::Operand1;
                }
                _ => state = States::Out,
            },
            States::Comma => match file[i] as char {
                d @ '0'..='9' => {
                    num2 = d.to_digit(10).unwrap() as u64;
                    state = States::Operand2
                }
                _ => state = States::Out,
            },
            States::Operand1 => match file[i] as char {
                d @ '0'..='9' => {
                    num1 = num1 * 10 + d.to_digit(10).unwrap() as u64;
                }
                ',' => state = States::Comma,
                _ => state = States::Out,
            },
            States::Operand2 => match file[i] as char {
                d @ '0'..='9' => num2 = num2 * 10 + d.to_digit(10).unwrap() as u64,
                ')' => {
                    res += num1 * num2;
                    state = States::Out
                }
                _ => state = States::Out,
            },
        }
        i += 1;
    }

    res
}

pub fn day3_2() -> u64 {
    let mut file: Vec<u8> = Vec::new();
    let mut state = States::Out;
    let _ = File::open("src/data/day3.txt")
        .unwrap()
        .read_to_end(&mut file)
        .unwrap_or_default();
    let n = file.len();
    let mut i = 0;
    let mut num1: u64 = 0;
    let mut num2: u64 = 0;
    let mut res: u64 = 0;
    let mut enabled = true;

    while i < n {
        match state {
            States::Out => {
                if i + 3 < n
                    && file[i] as char == 'm'
                    && file[i + 1] as char == 'u'
                    && file[i + 2] as char == 'l'
                    && file[i + 3] as char == '('
                {
                    state = States::Operator;
                    i += 3;
                } else if i + 6 < n
                    && file[i] as char == 'd'
                    && file[i + 1] as char == 'o'
                    && file[i + 2] as char == 'n'
                    && file[i + 3] as char == '\''
                    && file[i + 4] as char == 't'
                    && file[i + 5] as char == '('
                    && file[i + 6] as char == ')'
                {
                    enabled = false;
                    i += 6
                } else if i + 3 < n
                    && file[i] as char == 'd'
                    && file[i + 1] as char == 'o'
                    && file[i + 2] as char == '('
                    && file[i + 3] as char == ')'
                {
                    enabled = true;
                    i += 3
                }
            }

            States::Operator => match file[i] as char {
                d @ '0'..='9' => {
                    num1 = d.to_digit(10).unwrap() as u64;
                    state = States::Operand1;
                }
                _ => state = States::Out,
            },
            States::Comma => match file[i] as char {
                d @ '0'..='9' => {
                    num2 = d.to_digit(10).unwrap() as u64;
                    state = States::Operand2
                }
                _ => state = States::Out,
            },
            States::Operand1 => match file[i] as char {
                d @ '0'..='9' => {
                    num1 = num1 * 10 + d.to_digit(10).unwrap() as u64;
                }
                ',' => state = States::Comma,
                _ => state = States::Out,
            },
            States::Operand2 => match file[i] as char {
                d @ '0'..='9' => num2 = num2 * 10 + d.to_digit(10).unwrap() as u64,
                ')' => {
                    if enabled {
                        res += num1 * num2;
                    }
                    state = States::Out;
                }
                _ => state = States::Out,
            },
        }
        i += 1;
    }

    res
}
#[cfg(test)]
mod test {
    use crate::day3::{day3_1, day3_2};

    #[test]
    fn test_day_3_1() {
        assert_eq!(190604937, day3_1());
    }

    #[test]
    fn test_day_3_2() {
        assert_eq!(82857512, day3_2());
    }
}
