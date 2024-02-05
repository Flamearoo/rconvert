use std::str::FromStr;

use roped::*;

trait ToBytes {
    fn to_bytes(input: &str) -> Vec<u8>;
    fn from_bytes(input: Vec<u8>) -> String;
}

struct Convertor {
    a: TypeIdent,
    b: TypeIdent,
}

impl Convertor {
    fn convert(&self, input: &str) -> String {
        self.b.from_bytes(self.a.to_bytes(input))
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum TypeIdent {
    String(),
    Binary(),
    Hexadecimal(),
}

impl TypeIdent {
    fn to_bytes(&self, input: &str) -> Vec<u8> {
        match self {
            TypeIdent::String() => input.as_bytes().to_vec(),
            TypeIdent::Binary() => input
                .chars()
                .collect::<Vec<char>>()
                .chunks(8)
                .map(|chunk| {
                    u8::from_str_radix(chunk.iter().collect::<String>().as_str(), 2).unwrap()
                })
                .collect(),
            TypeIdent::Hexadecimal() => input
                .chars()
                .collect::<Vec<char>>()
                .chunks(2)
                .map(|chunk| {
                    u8::from_str_radix(chunk.iter().collect::<String>().as_str(), 16).unwrap()
                })
                .collect(),
        }
    }

    fn from_bytes(&self, input: Vec<u8>) -> String {
        match self {
            TypeIdent::String() => String::from_utf8_lossy(&input).to_string(),
            TypeIdent::Binary() => input
                .iter()
                .map(|&byte| format!("{:08b}", byte))
                .collect::<String>(),
            TypeIdent::Hexadecimal() => input
                .iter()
                .map(|&byte| format!("{:02X}", byte))
                .collect::<String>(),
        }
    }
}

impl FromStr for TypeIdent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "str" => Ok(TypeIdent::String()),
            "bin" => Ok(TypeIdent::Binary()),
            "hex" => Ok(TypeIdent::Hexadecimal()),
            _ => {
                Err(())
            },
        }
    }
}

fn decode(input: &str) -> (&str, TypeIdent) {
    if input.len() > 2 {
        match input.split_at(2) {
            ("0s", v) => return (v, TypeIdent::String()),
            ("0b", v) => return (v, TypeIdent::Binary()),
            ("0x", v) => return (v, TypeIdent::Hexadecimal()),
            _ => (),
        }
    }

    (input, TypeIdent::String())
}

fn recode(input: &str, ty: TypeIdent) -> String {
    match ty {
        TypeIdent::String() => input.into(),
        TypeIdent::Binary() => format!("0b{}", input),
        TypeIdent::Hexadecimal() => format!("0x{}", input),
    }
}

#[allow(dead_code)]
#[derive(Bundle)]
#[bundle(state = "EmptyState")]
pub enum Container {
    #[bundle(name = "quit")]
    Quit(Quit),
    #[bundle(name = "convert")]
    Convert(Convert),
}

#[allow(dead_code)]
#[derive(Strand)]
#[strand(state = "EmptyState", action = "action")]
pub struct Quit;

impl Quit {
    fn action(&self, _: &mut EmptyState) -> Result<(), String> {
        println!("Exiting program...");
        std::process::exit(0)
    }
}

#[allow(dead_code)]
#[derive(Strand)]
#[strand(state = "EmptyState", action = "action")]
pub struct Convert {
    input: String,
    ty: TypeIdent,
}

impl Convert {
    fn action(&self, _: &mut EmptyState) -> Result<(), String> {
        let (decoded, d_ty) = decode(&self.input);
        let convertor = Convertor {
            a: d_ty,
            b: self.ty,
        };

        let converted = if convertor.a == convertor.b {
            decoded.into()
        } else {
            convertor.convert(decoded)
        };

        println!("{}", recode(&converted, self.ty));

        Ok(())
    }
}
