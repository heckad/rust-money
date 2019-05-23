use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use std::str::FromStr;


#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Money {
    amount: i64,
}

impl Money {
    pub fn new_from_int(amount: i64) -> Result<Money, String> {
        if let Some(amount) = amount.checked_mul(100) {
            Ok(Money { amount })
        } else {
            Err("amount very big for type money".to_string())
        }
    }
}

impl FromStr for Money {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Err("String is empty".to_string());
        }

        let mut amount: i64 = 0;

        let mut is_counter_activated = false;
        let mut counter: u8 = 0;

        let mut is_negative_value = false;

        let ns =
            if s.chars().nth(0).unwrap() == '-' {
                is_negative_value = true;
                &s[1..]
            } else if s.chars().nth(0).unwrap() == '+' {
                &s[1..]
            } else { s };

        for char in ns.chars() {
            if is_counter_activated {
                if counter <= 2 {
                    counter += 1
                } else {
                    return Err("Failing parsing string".to_string());
                }
            }
            if char == '.' {
                if !is_counter_activated {
                    is_counter_activated = true;
                    continue;
                } else {
                    return Err("Failing parsing string".to_string());
                }
            }

            amount = match amount.checked_mul(10) {
                Some(v) => v,
                None => return Err("Failing parsing string".to_string())
            };

            let num = match char.to_digit(10) {
                Some(v) => v as i64,
                None => return Err("Failing parsing string".to_string())
            };

            amount = match amount.checked_add(num) {
                Some(v) => v,
                None => return Err("Failing parsing string".to_string())
            };
        }

        while counter < 2 {
            counter += 1;
            amount *= 10;
        }
        if is_negative_value { amount = -amount }
        Ok(Money { amount })
    }
}

impl Add for &Money {
    type Output = Result<Money, String>;

    fn add(self, rhs: &Money) -> Self::Output {
        if let Some(amount) = self.amount.checked_add(rhs.amount) {
            Ok(Money { amount })
        } else {
            Err("Overflow".to_string())
        }
    }
}

impl Add<i64> for &Money {
    type Output = Result<Money, String>;

    fn add(self, rhs: i64) -> Self::Output {
        if let Some(amount) = self.amount.checked_add(rhs) {
            Ok(Money { amount })
        } else {
            Err("Overflow".to_string())
        }
    }
}

impl Sub for &Money {
    type Output = Result<Money, String>;

    fn sub(self, rhs: &Money) -> Self::Output {
        if let Some(amount) = self.amount.checked_sub(rhs.amount) {
            Ok(Money { amount })
        } else {
            Err("Overflow".to_string())
        }
    }
}

impl Mul for &Money {
    type Output = Result<Money, String>;

    fn mul(self, rhs: &Money) -> Self::Output {
        if let Some(amount) = self.amount.checked_mul(rhs.amount) {
            Ok(Money { amount })
        } else {
            Err("Overflow".to_string())
        }
    }
}

impl Div for &Money {
    type Output = Result<f64, String>;

    fn div(self, rhs: &Money) -> Self::Output {
        if rhs.amount == 0 {
            Err("Thero dividion".to_string())
        } else {
            Ok(self.amount as f64 / rhs.amount as f64)
        }
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("{:03}", self.amount);
        s.insert(s.len() - 2, '.');
        write!(f, "{}", s.to_string())
    }
}

mod test {
    use crate::Money;
    use std::str::FromStr;

    #[test]
    fn test_create_from_small_int() {
        let money = Money::new_from_int(65).unwrap();
        assert_eq!("65.00", money.to_string())
    }

    #[test]
    fn test_create_from_big_int() {
        let money = Money::new_from_int(9223372036854775800);
        assert_eq!(Err("amount very big for type money".to_string()), money)
    }

    #[test]
    fn test_create_from_string() {
        let money: Money = "123".parse().unwrap();
        assert_eq!("123.00", money.to_string());

        let money: Money = "123.0".parse().unwrap();
        assert_eq!("123.00", money.to_string());

        let money = Money::from_str("-123.0").unwrap();
        assert_eq!("-123.00", money.to_string());


        let money = Money::from_str("92233720368547758.07").unwrap();
        assert_eq!("92233720368547758.07", money.to_string());

        let money = Money::from_str("+92233720368547758.07").unwrap();
        assert_eq!("92233720368547758.07", money.to_string());

        let money = Money::from_str("-92233720368547758.07").unwrap();
        assert_eq!("-92233720368547758.07", money.to_string());
    }

    #[test]
    fn test_add_operation() {
        let money1 = &Money::from_str("55.32").unwrap();
        let money2 = &Money::from_str("12.").unwrap();
        let money3 = &Money::from_str("32.").unwrap();

        assert_eq!("67.32", (money1 + money2).unwrap().to_string());
        assert_eq!("87.32", (money1 + money3).unwrap().to_string());
    }

    #[test]
    fn test_div_operation() {
        let money1 = &"20".parse::<Money>().unwrap();
        let money2 = &"10".parse::<Money>().unwrap();

        assert_eq!(2 as f64, (money1 / money2).unwrap())
    }
}