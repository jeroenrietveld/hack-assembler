use regex::Regex;

use crate::helper::unwrap_optional_match;

pub enum InstructionType {
    C(CInstruction),
    A(AInstruction),
    L(LInstruction),
}

pub struct CInstruction {
    _raw: String,
    dest: Dest,
    comp: Comp,
    jump: Jump,
}

pub struct AInstruction {
    _raw: String,
    pub address: Option<i16>,
    pub symbol: Option<String>,
}

pub struct LInstruction {
    _raw: String,
    pub symbol: String,
}

impl CInstruction {
    pub fn new(raw: String) -> CInstruction {
        let (dest, comp, jump) = CInstruction::parse_raw(&raw);
        CInstruction {
            _raw: raw,
            dest,
            comp,
            jump,
        }
    }

    fn parse_raw(raw: &String) -> (Dest, Comp, Jump) {
        lazy_static! {
            static ref C_REGEX: Regex = Regex::new(r"((?P<dest>\w*)=)?(?P<comp>[\w|&!+-]*)(;?(?P<jump>\w*))").unwrap();
        }

        let captures = C_REGEX
            .captures(raw)
            .expect(&format!("Could not unwrap CInstruction captures for: {}", raw));
        let dest_capture = captures.name("dest");
        let comp_capture = captures.name("comp");
        let jump_capture = captures.name("jump");
        let dest = unwrap_optional_match(dest_capture);
        let comp = unwrap_optional_match(comp_capture);
        let jump = unwrap_optional_match(jump_capture);

        (
            Dest::new(dest.unwrap_or("")),
            Comp::new(comp.unwrap_or("")),
            Jump::new(jump.unwrap_or("")),
        )
    }

    pub fn to_binary(&self) -> String {
        let dest = self.dest.clone() as u8;
        let comp = self.comp.clone() as u8;
        let jump = self.jump.clone() as u8;
        format!("111{:07b}{:03b}{:03b}", comp, dest, jump)
    }
}

impl AInstruction {
    pub fn new(raw: String) -> AInstruction {
        let (address, symbol) = AInstruction::parse_raw(&raw);
        AInstruction {
            _raw: raw,
            address,
            symbol,
        }
    }

    fn parse_raw(raw: &String) -> (Option<i16>, Option<String>) {
        lazy_static! {
            static ref A_REGEX: Regex = Regex::new(r"@(?P<address>\d*)?(?P<symbol>[\w\.$_]*)?").unwrap();
        }

        let captures = A_REGEX
            .captures(raw)
            .expect("Could not unwrap AInstruction captures");

        let address_match = &captures["address"];
        let symbol_match = &captures["symbol"];

        if address_match.is_empty() {
            (None, Some(symbol_match.to_string()))
        } else {
            let address = address_match.parse::<i16>().unwrap();
            (Some(address), None)
        }
    }

    pub fn resolved_symbol_to_address(&self) -> bool {
        if self.address.is_some() {
            return true;
        };

        if self.symbol.is_none() {
            panic!("Unresolved AInstruction address doesn't have symbol")
        };

        false
    }

    pub fn to_binary(&self) -> String {
        format!(
            "0{:015b}",
            self.address.expect("Unresolved symbol for AInstruction")
        )
    }
}

impl LInstruction {
    pub fn new(raw: String) -> LInstruction {
        let symbol = LInstruction::parse_raw(&raw);
        LInstruction { _raw: raw, symbol }
    }

    fn parse_raw(raw: &String) -> String {
        lazy_static!{
            static ref L_REGEX: Regex = Regex::new(r"\((?P<symbol>[\w.$_]*)\)").unwrap();
        }

        let captures = L_REGEX
            .captures(raw)
            .expect(&format!("Could not unwrap LInstruction captures for: {}", raw));

        captures["symbol"].to_string()
    }
}

#[derive(Clone)]
enum Comp {
    ConstantZero = 42,
    ConstantOne = 63,
    ConstantNegativeOne = 58,
    DReg = 12,
    AReg = 48,
    NotDReg = 13,
    NotAReg = 49,
    NegateDReg = 15,
    NegateAReg = 51,
    DRegPlusOne = 31,
    ARegPlusOne = 55,
    DRegMinusOne = 14,
    ARegMinusOne = 50,
    DRegPlusAReg = 2,
    DRegMinusAReg = 19,
    ARegMinusDReg = 7,
    DRegAndAReg = 0,
    DRegOrAReg = 21,
    MIn = 112,
    NotMIn = 113,
    NegateMIn = 115,
    MInPlusOne = 119,
    MInMinusOne = 114,
    DRegPlusMIn = 66,
    DRegMinusMIn = 83,
    MInMinusDReg = 71,
    DRegAndMIn = 64,
    DRegOrMIn = 85,
}

impl Comp {
    fn new(comp: &str) -> Comp {
        match comp {
            "0" => Comp::ConstantZero,
            "1" => Comp::ConstantOne,
            "-1" => Comp::ConstantNegativeOne,
            "D" => Comp::DReg,
            "A" => Comp::AReg,
            "!D" => Comp::NotDReg,
            "!A" => Comp::NotAReg,
            "-D" => Comp::NegateDReg,
            "-A" => Comp::NegateAReg,
            "D+1" => Comp::DRegPlusOne,
            "A+1" => Comp::ARegPlusOne,
            "D-1" => Comp::DRegMinusOne,
            "A-1" => Comp::ARegMinusOne,
            "D+A" => Comp::DRegPlusAReg,
            "D-A" => Comp::DRegMinusAReg,
            "A-D" => Comp::ARegMinusDReg,
            "D&A" => Comp::DRegAndAReg,
            "D|A" => Comp::DRegOrAReg,
            "M" => Comp::MIn,
            "!M" => Comp::NotMIn,
            "-M" => Comp::NegateMIn,
            "M+1" => Comp::MInPlusOne,
            "M-1" => Comp::MInMinusOne,
            "D+M" => Comp::DRegPlusMIn,
            "D-M" => Comp::DRegMinusMIn,
            "M-D" => Comp::MInMinusDReg,
            "D&M" => Comp::DRegAndMIn,
            "D|M" => Comp::DRegOrMIn,
            _ => panic!("Invalid comp: {}", comp),
        }
    }
}

#[derive(Clone)]
enum Dest {
    Null = 0,
    MIn = 1,
    DReg = 2,
    MInDReg = 3,
    AReg = 4,
    ARegMIn = 5,
    ARegDReg = 6,
    ARegDRegMIn = 7,
}

impl Dest {
    fn new(dest: &str) -> Dest {
        match dest {
            "" => Dest::Null,
            "M" => Dest::MIn,
            "D" => Dest::DReg,
            "MD" => Dest::MInDReg,
            "A" => Dest::AReg,
            "AM" => Dest::ARegMIn,
            "AD" => Dest::ARegDReg,
            "AMD" => Dest::ARegDRegMIn,
            _ => panic!("Invalid destination"),
        }
    }
}

#[derive(Clone)]
enum Jump {
    NULL = 0,
    JGT = 1,
    JEQ = 2,
    JGE = 3,
    JLT = 4,
    JNE = 5,
    JLE = 6,
    JMP = 7,
}

impl Jump {
    fn new(jump: &str) -> Jump {
        match jump {
            "" => Jump::NULL,
            "JGT" => Jump::JGT,
            "JEQ" => Jump::JEQ,
            "JGE" => Jump::JGE,
            "JLT" => Jump::JLT,
            "JNE" => Jump::JNE,
            "JLE" => Jump::JLE,
            "JMP" => Jump::JMP,
            _ => panic!("Invalid jump"),
        }
    }
}
