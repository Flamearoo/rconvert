use roped::*;

use num_bigint::BigUint;

const CHAR_INDEX: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z', '-', '_',
];

fn decode(input: &str, base: u8) -> Result<BigUint, String> {
    let arr: Vec<u8> = input
        .chars()
        .map(|char| {
            let val = CHAR_INDEX
                .iter()
                .position(|&x| x == char)
                .ok_or(format!("Could not convert from base {}", base))?;
            Ok(val as u8)
        })
        .collect::<Result<_, String>>()?;
    BigUint::from_radix_be(&arr, base.into()).ok_or(format!("Could not convert from base {}", base))
}

fn recode(num: BigUint, base: u8) -> Result<String, String> {
    let arr = num.to_radix_be(base.into());
    arr.iter().try_fold(String::new(), |mut w: String, num| {
        let char = CHAR_INDEX
            .get(num.to_owned() as usize)
            .ok_or(format!("Could not convert to base {}", base))?
            .to_owned();
        w.push(char);
        Ok(w)
    })
}

struct Converter {
    base_i: u8,
    base_f: u8,
}

impl Converter {
    fn convert(&self, input: &str) -> Result<String, String> {
        if self.base_i > 64 || self.base_i < 1 {
            return Ok("Initial base is too large (1-64)".into());
        };
        if self.base_f > 64 || self.base_f < 1 {
            return Ok("Final base is too large (1-64)".into());
        };
        recode(decode(input, self.base_i)?, self.base_f)
    }
}

struct BaseWrapper(u8);

impl std::str::FromStr for BaseWrapper {
    type Err = EmptyState;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "una" => Ok(BaseWrapper(1)),
            "bin" => Ok(BaseWrapper(2)),
            "ter" => Ok(BaseWrapper(3)),
            "qua" => Ok(BaseWrapper(4)),
            "qui" => Ok(BaseWrapper(5)),
            "sen" => Ok(BaseWrapper(6)),
            "sep" => Ok(BaseWrapper(7)),
            "oct" => Ok(BaseWrapper(8)),
            "non" => Ok(BaseWrapper(9)),
            "dec" => Ok(BaseWrapper(10)),
            "duo" => Ok(BaseWrapper(12)),
            "hex" => Ok(BaseWrapper(16)),
            "hxa" => Ok(BaseWrapper(32)),
            "sn2" => Ok(BaseWrapper(36)),
            "hxb" => Ok(BaseWrapper(64)),
            n => Ok(BaseWrapper(n.parse::<u8>().map_err(|_| EmptyState)?)),
        }
    }
}

#[allow(dead_code)]
#[derive(Strand)]
#[strand(state = "EmptyState", action = "action")]
pub struct Convert {
    input: String,
    base_i: BaseWrapper,
    base_f: BaseWrapper,
}

impl Convert {
    fn action(&self, _: &mut EmptyState) -> Result<(), String> {
        let converter = Converter {
            base_i: self.base_i.0,
            base_f: self.base_f.0,
        };

        println!("{}", converter.convert(&self.input)?);

        Ok(())
    }
}
