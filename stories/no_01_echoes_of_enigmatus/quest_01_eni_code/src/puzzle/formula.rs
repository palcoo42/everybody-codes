use std::{error::Error, sync::LazyLock};

use crate::puzzle::eni_code::eni_code;
use regex::Regex;

static RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^A=(\d+) B=(\d+) C=(\d+) X=(\d+) Y=(\d+) Z=(\d+) M=(\d+)")
        .expect("Failed to create 'Formula' regex")
});

pub struct Formula {
    pub a: i64,
    pub b: i64,
    pub c: i64,
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub m: i64,
}

impl Formula {
    pub fn from_string(content: &str) -> Result<Formula, Box<dyn Error>> {
        // Try to extract values
        if let Some(captures) = RE.captures(content) {
            // Check number of captures
            if captures.len() != 8 {
                return Err(format!("Expected 7 captures, found {}", captures.len()).into());
            }

            let a: i64 = captures[1].parse().map_err(|_| -> Box<dyn Error> {
                format!("Failed to parse '{}' to i64", &captures[1]).into()
            })?;

            let b: i64 = captures[2].parse().map_err(|_| -> Box<dyn Error> {
                format!("Failed to parse '{}' to i64", &captures[2]).into()
            })?;

            let c: i64 = captures[3].parse().map_err(|_| -> Box<dyn Error> {
                format!("Failed to parse '{}' to i64", &captures[3]).into()
            })?;

            let x: i64 = captures[4].parse().map_err(|_| -> Box<dyn Error> {
                format!("Failed to parse '{}' to i64", &captures[4]).into()
            })?;

            let y: i64 = captures[5].parse().map_err(|_| -> Box<dyn Error> {
                format!("Failed to parse '{}' to i64", &captures[5]).into()
            })?;

            let z: i64 = captures[6].parse().map_err(|_| -> Box<dyn Error> {
                format!("Failed to parse '{}' to i64", &captures[6]).into()
            })?;

            let m: i64 = captures[7].parse().map_err(|_| -> Box<dyn Error> {
                format!("Failed to parse '{}' to i64", &captures[7]).into()
            })?;

            return Ok(Formula {
                a,
                b,
                c,
                x,
                y,
                z,
                m,
            });
        }

        Err(format!("Failed to parse Formula from '{content}'").into())
    }

    pub fn eni_code(&self) -> i64 {
        eni_code(self.a, self.x, self.m)
            + eni_code(self.b, self.y, self.m)
            + eni_code(self.c, self.z, self.m)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_eni_code() {
        assert_eq!(
            Formula {
                a: 4,
                b: 4,
                c: 6,
                x: 3,
                y: 4,
                z: 5,
                m: 11
            }
            .eni_code(),
            114644
        );

        assert_eq!(
            Formula {
                a: 8,
                b: 4,
                c: 7,
                x: 8,
                y: 4,
                z: 6,
                m: 12
            }
            .eni_code(),
            48661009
        );

        assert_eq!(
            Formula {
                a: 2,
                b: 8,
                c: 6,
                x: 2,
                y: 4,
                z: 5,
                m: 13
            }
            .eni_code(),
            313276
        );

        assert_eq!(
            Formula {
                a: 5,
                b: 9,
                c: 6,
                x: 8,
                y: 6,
                z: 8,
                m: 14
            }
            .eni_code(),
            11611972920
        );

        assert_eq!(
            Formula {
                a: 5,
                b: 9,
                c: 7,
                x: 6,
                y: 6,
                z: 8,
                m: 15
            }
            .eni_code(),
            1240513421
        );

        assert_eq!(
            Formula {
                a: 8,
                b: 8,
                c: 8,
                x: 6,
                y: 9,
                z: 6,
                m: 16
            }
            .eni_code(),
            24
        );
    }
}
