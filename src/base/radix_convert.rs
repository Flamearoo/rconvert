use roped::*;

use num_bigint::BigUint;

fn decode(input: &str, base: u8) -> Result<BigUint, String> {
    //WRONG
    let arr: Vec<u8> = input.chars().map(|char| char as u8).collect();
    BigUint::from_radix_be(&arr, base.into()).ok_or(format!("Could not convert from base {}", base))
}

fn recode(num: BigUint, base: u8) -> Result<String, String> {
    //WRONG
    let arr = num.to_radix_be(base.into());
    String::from_utf8(arr).map_err(|_| format!("Could not convert into base {}", base))
}

struct Converter {
    base_i: u8,
    base_f: u8,
}

impl Converter {
    fn convert(&self, input: &str) -> Result<String, String> {
        recode(decode(input, self.base_i)?, self.base_f)
    }
}

#[allow(dead_code)]
#[derive(Strand)]
#[strand(state = "EmptyState", action = "action")]
pub struct Convert {
    input: String,
    base_i: u8,
    base_f: u8,
}

impl Convert {
    fn action(&self, _: &mut EmptyState) -> Result<(), String> {
        let converter = Converter {
            base_i: self.base_i,
            base_f: self.base_f,
        };

        println!("{}", converter.convert(&self.input)?);

        Ok(())
    }
}
