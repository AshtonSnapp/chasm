use logos::{Logos, Lexer};

#[derive(Debug)]
pub enum IdentifierType {
    Label,
    Symbol,
}

#[derive(Debug)]
pub struct IdentifierInfo {
    itype: IdentifierType,
    val: String
}

#[derive(Debug)]
pub enum InstructionType {
    NoOperation,
    MoveData,
    HaltAndCatchFire,
    MoveDataXIndexed,
    MoveDataYIndexed,
    SetDirectPage,
    SetStackBank,
    GetStackBank,
    PushRegister,
    PushIntegerFlags,
    PullRegister,
    PullIntegerFlags,
    AddIntegers,
    AddIntegersCarry,
    SubtractIntegers,
    SubtractIntegersCarry,
    BitwiseNot,
    BitwiseAnd,
    BitwiseOr,
    BitwiseExclusiveOr,
    BitwiseShiftLeft,
    BitwiseShiftRight,
    BitwiseRotateLeft,
    BitwiseRotateRight,
    BitwiseSetRegisterBit,
    BitwiseSetIntegerFlag,
    BitwiseClearRegisterBit,
    BitwiseClearIntegerFlag,
    BitwisePopulationCount,
    BitwiseVacancyCount,
    Jump,
    JumpXIndexed,
    JumpYIndexed,
    CallSubroutine,
    CompareIntegers,
    TestRegisterBit,
    TestIntegerFlag,
    BranchEqual,
    BranchNotEqual,
    BranchLessThanUnsigned,
    BranchLessThanSigned,
    BranchGreaterThanUnsigned,
    BranchGreaterThanSigned,
    ReturnFromSubroutine,
    ReturnFromInterrupt,
    Break,
    WaitForInterrupt,
    BranchLessEqualUnsigned,
    BranchLessEqualSigned,
    BranchGreaterEqualUnsigned,
    BranchGreaterEqualSigned
}

#[derive(Debug)]
pub enum DirectiveType {
    SetOrigin,                  // .org
    MakeSymbol,                 // .sym
    PlaceByte,                  // .byte
    PlaceWord,                  // .word
    PlaceVector,                // .vec
    PlaceString,                // .str
    PlaceStringNullTerminated,  // .strz
    ImportAssembly,             // .asm
    ImportBinary,               // .bin
}

#[derive(Debug)]
pub enum AddressType {
    Absolute,                   // 24-bits                  Example: $FFFFFF
    Port,                       // 16-bits                  Example: $FFFFp
    ZeroBank,                   // 16-bits                  Example: $FFFF
    DirectPage,                 // 8-bits                   Example: $FF
    InstructionPtrRelative,     // 16-bits two's complement Example: IP+$7FFF or IP-$8000
    StackPtrRelative,           // 16-bits two's complement Example: SP+$7FFF or SP-$8000
    Indirect,                   // 24-bits                  Example: ($FFFFFF)
}

#[derive(Debug)]
pub enum NumBase {
    Hexadecimal,                // 0 1 2 3 4 5 6 7 8 9 A B C D E F 10
    Decimal,                    // 0 1 2 3 4 5 6 7 8 9 10
    Binary                      // 0 1 10
}

#[derive(Debug)]
pub struct ImmediateInfo {
    base: NumBase,
    val: u32
}

#[derive(Debug)]
pub struct AddressInfo {
    addr_type: AddressType,
    base: NumBase,
    val: u32
}

fn label(lex: &mut Lexer<Token>) -> Result<IdentifierInfo, ()> {
    let slice: &str = lex.slice();

    todo!("label callback unimplemented at the moment, sorry!");
    /* How do I do this?
     * I'll need to go backwards from the end of slice until I've seen a : and a space
     * That's the best way I can think of to go about this, but how would I do that?
     */
}

fn symbol(lex: &mut Lexer<Token>) -> Result<IdentifierInfo, ()> {
    let slice: &str = lex.slice();

    todo!("symbol callback unimplemented at the moment, sorry!");
    /* How do I do this?
     * I'll need to go backwards from the end of slice until I've seen a space
     * That's the best way I can think of to go about this, but how would I do that?
     */
}

fn instruction_small(lex: &mut Lexer<Token>) -> Result<InstructionType, ()> {
    let slice: &str = lex.slice();

    let poss_inst: &str = &slice[slice.len() - 3..slice.len() - 1].to_lowercase().as_str();

    // baby function lol

    match poss_inst {
        "or" => Ok(InstructionType::BitwiseOr),
        _ => Err(())
    }
}

fn instruction(lex: &mut Lexer<Token>) -> Result<InstructionType, ()> {
    let slice: &str = lex.slice();

    let poss_inst: &str = &slice[slice.len() - 4..slice.len() - 1].to_lowercase().as_str();

    match poss_inst {
        "mov" => Ok(InstructionType::MoveData),
        "not" => Ok(InstructionType::BitwiseNot),
        "and" => Ok(InstructionType::BitwiseAnd),
        "xor" => Ok(InstructionType::BitwiseExclusiveOr),
        "shl" => Ok(InstructionType::BitwiseShiftLeft),
        "shr" => Ok(InstructionType::BitwiseShiftRight),
        "rol" => Ok(InstructionType::BitwiseRotateLeft),
        "ror" => Ok(InstructionType::BitwiseRotateRight),
        "set" => Ok(InstructionType::BitwiseSetRegisterBit),
        "clr" => Ok(InstructionType::BitwiseClearRegisterBit),
        "brk" => Ok(InstructionType::Break),
        _ => Err(())
    }
}

fn instruction_large(lex: &mut Lexer<Token>) -> Result<InstructionType, ()> {
    let slice: &str = lex.slice();

    let poss_inst: &str = &slice[slice.len() - 5..slice.len() - 1].to_lowercase().as_str();

    match poss_inst {
        "noop" => Ok(InstructionType::NoOperation),
        "hacf" => Ok(InstructionType::HaltAndCatchFire),
        "movx" => Ok(InstructionType::MoveDataXIndexed),
        "movy" => Ok(InstructionType::MoveDataYIndexed),
        "push" => Ok(InstructionType::PushRegister),
        "pull" => Ok(InstructionType::PullRegister),
        "addi" => Ok(InstructionType::AddIntegers),
        "adci" => Ok(InstructionType::AddIntegersCarry),
        "subi" => Ok(InstructionType::SubtractIntegers),
        "sbci" => Ok(InstructionType::SubtractIntegersCarry),
        "pcnt" => Ok(InstructionType::BitwisePopulationCount),
        "vcnt" => Ok(InstructionType::BitwiseVacancyCount),
        "jump" => Ok(InstructionType::Jump),
        "call" => Ok(InstructionType::CallSubroutine),
        "cmpi" => Ok(InstructionType::CompareIntegers),
        "test" => Ok(InstructionType::TestRegisterBit),
        "bequ" => Ok(InstructionType::BranchEqual),
        "bneq" => Ok(InstructionType::BranchNotEqual),
        "bltu" => Ok(InstructionType::BranchLessThanUnsigned),
        "blts" => Ok(InstructionType::BranchLessThanSigned),
        "bgtu" => Ok(InstructionType::BranchGreaterThanUnsigned),
        "bgts" => Ok(InstructionType::BranchGreaterThanSigned),
        "rets" => Ok(InstructionType::ReturnFromSubroutine),
        "reti" => Ok(InstructionType::ReturnFromInterrupt),
        "wait" => Ok(InstructionType::WaitForInterrupt),
        "bleu" => Ok(InstructionType::BranchLessEqualUnsigned),
        "bles" => Ok(InstructionType::BranchLessEqualSigned),
        "bgeu" => Ok(InstructionType::BranchGreaterEqualUnsigned),
        "bges" => Ok(InstructionType::BranchGreaterEqualSigned),
        _ => Err(())
    }
}

fn instruction_xlarge(lex: &mut Lexer<Token>) -> Result<InstructionType, ()> {
    let slice: &str = lex.slice();

    let poss_inst: &str = &slice[slice.len() - 6..slice.len() - 1].to_lowercase().as_str();

    match poss_inst {
        "setdp" => Ok(InstructionType::SetDirectPage),
        "setsb" => Ok(InstructionType::SetStackBank),
        "getsb" => Ok(InstructionType::GetStackBank),
        "setif" => Ok(InstructionType::BitwiseSetIntegerFlag),
        "clrif" => Ok(InstructionType::BitwiseClearIntegerFlag),
        "jumpx" => Ok(InstructionType::JumpXIndexed),
        "jumpy" => Ok(InstructionType::JumpYIndexed),
        _ => Err(())
    }
}

fn instruction_xxlarge(lex: &mut Lexer<Token>) -> Result<InstructionType, ()> {
    let slice: &str = lex.slice();

    let poss_inst: &str = &slice[slice.len() - 7..slice.len() - 1].to_lowercase().as_str();

    match poss_inst {
        "pushif" => Ok(InstructionType::PushIntegerFlags),
        "pullif" => Ok(InstructionType::PullIntegerFlags),
        "testif" => Ok(InstructionType::TestIntegerFlag),
        _ => Err(())
    }
}

fn directive(lex: &mut Lexer<Token>) -> Result<DirectiveType, ()> {
    let slice: &str = lex.slice();

    let poss_dir: &str = &slice[slice.len() - 5..slice.len() - 1].to_lowercase().as_str;

    if poss_dir == ".org" {
        Ok(DirectiveType::SetOrigin)
    } else if poss_dir == ".sym" {
        Ok(DirectiveType::MakeSymbol)
    } else if poss_dir == ".vec" {
        Ok(DirectiveType::PlaceVector)
    } else if poss_dir == ".str" {
        Ok(DirectiveType::PlaceString)
    } else if poss_dir == ".asm" {
        Ok(DirectiveType::ImportAssembly)
    } else if poss_dir == ".bin" {
        Ok(DirectiveType::ImportBinary)
    } else {
        Err(())
    }
}

fn directive_large(lex: &mut Lexer<Token>) -> Result<DirectiveType, ()> {
    let slice: &str = lex.slice();

    let poss_dir: &str = &slice[slice.len() - 6..slice.len() - 1].to_lowercase().as_str;

    if poss_dir == ".byte" {
        Ok(DirectiveType::PlaceByte)
    } else if poss_dir == ".word" {
        Ok(DirectiveType::PlaceByte)
    } else if poss_dir == ".strz" {
        Ok(DirectiveType::PlaceStringNullTerminated)
    } else {
        Err(())
    }
}

// TODO: address and immediate functions go here

fn string(lex: &mut Lexer<Token>) -> Result<[u8], ()> {
    let slice: &str = lex.slice();

    todo!("string callback unimplemented at the moment, sorry!");
    /* How do I do this?
     * I'll need to go backwards from the end of slice until I've seen two "s
     * That's the best way I can think of to go about this, but how would I do that?
     */
}

fn char(lex: &mut Lexer<Token>) -> Result<u8, ()> {
    let slice: &str = lex.slice();

    let poss_char: &str = &slice[slice.len() - 2];

    // Welcome to hell.

    match poss_char {
        "\x00" => Ok(0),
        "\x01" => Ok(1),
        "\x02" => Ok(2),
        "\x03" => Ok(3),
        "\x04" => Ok(4),
        "\x05" => Ok(5),
        "\x06" => Ok(6),
        "\x07" => Ok(7),
        "\x08" => Ok(8),
        "\x09" => Ok(9),
        "\x0A" => Ok(10),
        "\x0B" => Ok(11),
        "\x0C" => Ok(12),
        "\x0D" => Ok(13),
        "\x0E" => Ok(14),
        "\x0F" => Ok(15),
        "\x10" => Ok(16),
        "\x11" => Ok(17),
        "\x12" => Ok(18),
        "\x13" => Ok(19),
        "\x14" => Ok(20),
        "\x15" => Ok(21),
        "\x16" => Ok(22),
        "\x17" => Ok(23),
        "\x18" => Ok(24),
        "\x19" => Ok(25),
        "\x1A" => Ok(26),
        "\x1B" => Ok(27),
        "\x1C" => Ok(28),
        "\x1D" => Ok(29),
        "\x1E" => Ok(30),
        "\x1F" => Ok(31),
        "\x20" => Ok(32),
        "\x21" => Ok(33),
        "\x22" => Ok(34),
        "\x23" => Ok(35),
        "\x24" => Ok(36),
        "\x25" => Ok(37),
        "\x26" => Ok(38),
        "\x27" => Ok(39),
        "\x28" => Ok(40),
        "\x29" => Ok(41),
        "\x2A" => Ok(42),
        "\x2B" => Ok(43),
        "\x2C" => Ok(44),
        "\x2D" => Ok(45),
        "\x2E" => Ok(46),
        "\x2F" => Ok(47),
        "\x30" => Ok(48),
        "\x31" => Ok(49),
        "\x32" => Ok(50),
        "\x33" => Ok(51),
        "\x34" => Ok(52),
        "\x35" => Ok(53),
        "\x36" => Ok(54),
        "\x37" => Ok(55),
        "\x38" => Ok(56),
        "\x39" => Ok(57),
        "\x3A" => Ok(58),
        "\x3B" => Ok(59),
        "\x3C" => Ok(60),
        "\x3D" => Ok(61),
        "\x3E" => Ok(62),
        "\x3F" => Ok(63),
        "\x40" => Ok(64),
        "\x41" => Ok(65),
        "\x42" => Ok(66),
        "\x43" => Ok(67),
        "\x44" => Ok(68),
        "\x45" => Ok(69),
        "\x46" => Ok(70),
        "\x47" => Ok(71),
        "\x48" => Ok(72),
        "\x49" => Ok(73),
        "\x4A" => Ok(74),
        "\x4B" => Ok(75),
        "\x4C" => Ok(76),
        "\x4D" => Ok(77),
        "\x4E" => Ok(78),
        "\x4F" => Ok(79),
        "\x50" => Ok(80),
        "\x51" => Ok(81),
        "\x52" => Ok(82),
        "\x53" => Ok(83),
        "\x54" => Ok(84),
        "\x55" => Ok(85),
        "\x56" => Ok(86),
        "\x57" => Ok(87),
        "\x58" => Ok(88),
        "\x59" => Ok(89),
        "\x5A" => Ok(90),
        "\x5B" => Ok(91),
        "\x5C" => Ok(92),
        "\x5D" => Ok(93),
        "\x5E" => Ok(94),
        "\x5F" => Ok(95),
        "\x60" => Ok(96),
        "\x61" => Ok(97),
        "\x62" => Ok(98),
        "\x63" => Ok(99),
        "\x64" => Ok(100),
        "\x65" => Ok(101),
        "\x66" => Ok(102),
        "\x67" => Ok(103),
        "\x68" => Ok(104),
        "\x69" => Ok(105),
        "\x6A" => Ok(106),
        "\x6B" => Ok(107),
        "\x6C" => Ok(108),
        "\x6D" => Ok(109),
        "\x6E" => Ok(110),
        "\x6F" => Ok(111),
        "\x70" => Ok(112),
        "\x71" => Ok(113),
        "\x72" => Ok(114),
        "\x73" => Ok(115),
        "\x74" => Ok(116),
        "\x75" => Ok(117),
        "\x76" => Ok(118),
        "\x77" => Ok(119),
        "\x78" => Ok(120),
        "\x79" => Ok(121),
        "\x7A" => Ok(122),
        "\x7B" => Ok(123),
        "\x7C" => Ok(124),
        "\x7D" => Ok(125),
        "\x7E" => Ok(126),
        "\x7F" => Ok(127),
        _ => Err(())
    }
}

fn char_esc(lex: &mut Lexer<Token>) -> Result<u8, ()> {
    let slice: &str = lex.slice();

    let poss_char: &str = &slice[slice.len() - 3..slice.len() - 2];

    match poss_char {
        "\\\\" => Ok(92),
        "\\a" => Ok(7),
        "\\b" => Ok(8),
        "\\f" => Ok(12),
        "\\n" => Ok(10),
        "\\r" => Ok(13),
        "\\t" => Ok(9),
        "\\v" => Ok(11),
        _ => Err(())
    }
}

fn char_esc_hex(lex: &mut Lexer<Token>) -> Result<u8, ()> {
    let slice: &str = lex.slice();

    let poss_char: &str = &slice[slice.len() - 5..slice.len() - 2];

    // Welcome to hell 2: electric boogaloo

    match poss_char {
        "\\x00" => Ok(0),
        "\\x01" => Ok(1),
        "\\x02" => Ok(2),
        "\\x03" => Ok(3),
        "\\x04" => Ok(4),
        "\\x05" => Ok(5),
        "\\x06" => Ok(6),
        "\\x07" => Ok(7),
        "\\x08" => Ok(8),
        "\\x09" => Ok(9),
        "\\x0A" => Ok(10),
        "\\x0B" => Ok(11),
        "\\x0C" => Ok(12),
        "\\x0D" => Ok(13),
        "\\x0E" => Ok(14),
        "\\x0F" => Ok(15),
        "\\x10" => Ok(16),
        "\\x11" => Ok(17),
        "\\x12" => Ok(18),
        "\\x13" => Ok(19),
        "\\x14" => Ok(20),
        "\\x15" => Ok(21),
        "\\x16" => Ok(22),
        "\\x17" => Ok(23),
        "\\x18" => Ok(24),
        "\\x19" => Ok(25),
        "\\x1A" => Ok(26),
        "\\x1B" => Ok(27),
        "\\x1C" => Ok(28),
        "\\x1D" => Ok(29),
        "\\x1E" => Ok(30),
        "\\x1F" => Ok(31),
        "\\x20" => Ok(32),
        "\\x21" => Ok(33),
        "\\x22" => Ok(34),
        "\\x23" => Ok(35),
        "\\x24" => Ok(36),
        "\\x25" => Ok(37),
        "\\x26" => Ok(38),
        "\\x27" => Ok(39),
        "\\x28" => Ok(40),
        "\\x29" => Ok(41),
        "\\x2A" => Ok(42),
        "\\x2B" => Ok(43),
        "\\x2C" => Ok(44),
        "\\x2D" => Ok(45),
        "\\x2E" => Ok(46),
        "\\x2F" => Ok(47),
        "\\x30" => Ok(48),
        "\\x31" => Ok(49),
        "\\x32" => Ok(50),
        "\\x33" => Ok(51),
        "\\x34" => Ok(52),
        "\\x35" => Ok(53),
        "\\x36" => Ok(54),
        "\\x37" => Ok(55),
        "\\x38" => Ok(56),
        "\\x39" => Ok(57),
        "\\x3A" => Ok(58),
        "\\x3B" => Ok(59),
        "\\x3C" => Ok(60),
        "\\x3D" => Ok(61),
        "\\x3E" => Ok(62),
        "\\x3F" => Ok(63),
        "\\x40" => Ok(64),
        "\\x41" => Ok(65),
        "\\x42" => Ok(66),
        "\\x43" => Ok(67),
        "\\x44" => Ok(68),
        "\\x45" => Ok(69),
        "\\x46" => Ok(70),
        "\\x47" => Ok(71),
        "\\x48" => Ok(72),
        "\\x49" => Ok(73),
        "\\x4A" => Ok(74),
        "\\x4B" => Ok(75),
        "\\x4C" => Ok(76),
        "\\x4D" => Ok(77),
        "\\x4E" => Ok(78),
        "\\x4F" => Ok(79),
        "\\x50" => Ok(80),
        "\\x51" => Ok(81),
        "\\x52" => Ok(82),
        "\\x53" => Ok(83),
        "\\x54" => Ok(84),
        "\\x55" => Ok(85),
        "\\x56" => Ok(86),
        "\\x57" => Ok(87),
        "\\x58" => Ok(88),
        "\\x59" => Ok(89),
        "\\x5A" => Ok(90),
        "\\x5B" => Ok(91),
        "\\x5C" => Ok(92),
        "\\x5D" => Ok(93),
        "\\x5E" => Ok(94),
        "\\x5F" => Ok(95),
        "\\x60" => Ok(96),
        "\\x61" => Ok(97),
        "\\x62" => Ok(98),
        "\\x63" => Ok(99),
        "\\x64" => Ok(100),
        "\\x65" => Ok(101),
        "\\x66" => Ok(102),
        "\\x67" => Ok(103),
        "\\x68" => Ok(104),
        "\\x69" => Ok(105),
        "\\x6A" => Ok(106),
        "\\x6B" => Ok(107),
        "\\x6C" => Ok(108),
        "\\x6D" => Ok(109),
        "\\x6E" => Ok(110),
        "\\x6F" => Ok(111),
        "\\x70" => Ok(112),
        "\\x71" => Ok(113),
        "\\x72" => Ok(114),
        "\\x73" => Ok(115),
        "\\x74" => Ok(116),
        "\\x75" => Ok(117),
        "\\x76" => Ok(118),
        "\\x77" => Ok(119),
        "\\x78" => Ok(120),
        "\\x79" => Ok(121),
        "\\x7A" => Ok(122),
        "\\x7B" => Ok(123),
        "\\x7C" => Ok(124),
        "\\x7D" => Ok(125),
        "\\x7E" => Ok(126),
        "\\x7F" => Ok(127),
        "\\x80" => Ok(128),
        "\\x81" => Ok(129),
        "\\x82" => Ok(130),
        "\\x83" => Ok(131),
        "\\x84" => Ok(132),
        "\\x85" => Ok(133),
        "\\x86" => Ok(134),
        "\\x87" => Ok(135),
        "\\x88" => Ok(136),
        "\\x89" => Ok(137),
        "\\x8A" => Ok(138),
        "\\x8B" => Ok(139),
        "\\x8C" => Ok(140),
        "\\x8D" => Ok(141),
        "\\x8E" => Ok(142),
        "\\x8F" => Ok(143),
        "\\x90" => Ok(144),
        "\\x91" => Ok(145),
        "\\x92" => Ok(146),
        "\\x93" => Ok(147),
        "\\x94" => Ok(148),
        "\\x95" => Ok(149),
        "\\x96" => Ok(150),
        "\\x97" => Ok(151),
        "\\x98" => Ok(152),
        "\\x99" => Ok(153),
        "\\x9A" => Ok(154),
        "\\x9B" => Ok(155),
        "\\x9C" => Ok(156),
        "\\x9D" => Ok(157),
        "\\x9E" => Ok(158),
        "\\x9F" => Ok(159),
        "\\xA0" => Ok(160),
        "\\xA1" => Ok(161),
        "\\xA2" => Ok(162),
        "\\xA3" => Ok(163),
        "\\xA4" => Ok(164),
        "\\xA5" => Ok(165),
        "\\xA6" => Ok(166),
        "\\xA7" => Ok(167),
        "\\xA8" => Ok(168),
        "\\xA9" => Ok(169),
        "\\xAA" => Ok(170),
        "\\xAB" => Ok(171),
        "\\xAC" => Ok(172),
        "\\xAD" => Ok(173),
        "\\xAE" => Ok(174),
        "\\xAF" => Ok(175),
        "\\xB0" => Ok(176),
        "\\xB1" => Ok(177),
        "\\xB2" => Ok(178),
        "\\xB3" => Ok(179),
        "\\xB4" => Ok(180),
        "\\xB5" => Ok(181),
        "\\xB6" => Ok(182),
        "\\xB7" => Ok(183),
        "\\xB8" => Ok(184),
        "\\xB9" => Ok(185),
        "\\xBA" => Ok(186),
        "\\xBB" => Ok(187),
        "\\xBC" => Ok(188),
        "\\xBD" => Ok(189),
        "\\xBE" => Ok(190),
        "\\xBF" => Ok(191),
        "\\xC0" => Ok(192),
        "\\xC1" => Ok(193),
        "\\xC2" => Ok(194),
        "\\xC3" => Ok(195),
        "\\xC4" => Ok(196),
        "\\xC5" => Ok(197),
        "\\xC6" => Ok(198),
        "\\xC7" => Ok(199),
        "\\xC8" => Ok(200),
        "\\xC9" => Ok(201),
        "\\xCA" => Ok(202),
        "\\xCB" => Ok(203),
        "\\xCC" => Ok(204),
        "\\xCD" => Ok(205),
        "\\xCE" => Ok(206),
        "\\xCF" => Ok(207),
        "\\xD0" => Ok(208),
        "\\xD1" => Ok(209),
        "\\xD2" => Ok(210),
        "\\xD3" => Ok(211),
        "\\xD4" => Ok(212),
        "\\xD5" => Ok(213),
        "\\xD6" => Ok(214),
        "\\xD7" => Ok(215),
        "\\xD8" => Ok(216),
        "\\xD9" => Ok(217),
        "\\xDA" => Ok(218),
        "\\xDB" => Ok(219),
        "\\xDC" => Ok(220),
        "\\xDD" => Ok(221),
        "\\xDE" => Ok(222),
        "\\xDF" => Ok(223),
        "\\xE0" => Ok(224),
        "\\xE1" => Ok(225),
        "\\xE2" => Ok(226),
        "\\xE3" => Ok(227),
        "\\xE4" => Ok(228),
        "\\xE5" => Ok(229),
        "\\xE6" => Ok(230),
        "\\xE7" => Ok(231),
        "\\xE8" => Ok(232),
        "\\xE9" => Ok(233),
        "\\xEA" => Ok(234),
        "\\xEB" => Ok(235),
        "\\xEC" => Ok(236),
        "\\xED" => Ok(237),
        "\\xEE" => Ok(238),
        "\\xEF" => Ok(239),
        "\\xF0" => Ok(240),
        "\\xF1" => Ok(241),
        "\\xF2" => Ok(242),
        "\\xF3" => Ok(243),
        "\\xF4" => Ok(244),
        "\\xF5" => Ok(245),
        "\\xF6" => Ok(246),
        "\\xF7" => Ok(247),
        "\\xF8" => Ok(248),
        "\\xF9" => Ok(249),
        "\\xFA" => Ok(250),
        "\\xFB" => Ok(251),
        "\\xFC" => Ok(252),
        "\\xFD" => Ok(253),
        "\\xFE" => Ok(254),
        "\\xFF" => Ok(255),
        _ => Err(())
    }
}

#[derive(Logos, Debug)]
pub enum Token {
    #[regex(r";[^\r\n]*(\r\n|\n)?", logos::skip)]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[error]
    Error,

    #[regex(r"'[\x00-\x7F]'", char)]
    #[regex(r"'\[0-9a-zA-Z]'", char_esc)]
    #[regex(r"'\\x[0-9a-fA-F][0-9a-fA-F]'", char_esc_hex)]
    Char(u8),

    #[regex(r#""[\x00-\xFF]+""#, string)]
    String([u8]),

    #[regex(r"#(\$|%)?[0-9a-fA-F]")]
    Immediate(ImmediateInfo),

    #[regex(r"(\$|%)?[0-9a-fA-F]p?")]
    Address(AddressInfo),

    #[regex(r".[a-zA-Z][a-zA-Z][a-zA-Z]", directive)]
    #[regex(r".[a-zA-Z][a-zA-Z][a-zA-Z][a-zA-Z]", directive_large)]
    Directive(DirectiveType),

    #[regex(r"[a-zA-Z][a-zA-Z]", instruction_small)]
    #[regex(r"[a-zA-Z][a-zA-Z][a-zA-Z]", instruction)]
    #[regex(r"[a-zA-Z][a-zA-Z][a-zA-Z][a-zA-Z]", instruction_large)]
    #[regex(r"[a-zA-Z][a-zA-Z][a-zA-Z][a-zA-Z][a-zA-Z]", instruction_xlarge)]
    #[regex(r"[a-zA-Z][a-zA-Z][a-zA-Z][a-zA-Z][a-zA-Z][a-zA-Z]", instruction_xxlarge)]
    Instruction(InstructionType),

    #[regex(r"[a-zA-Z_-]+:", label)]
    #[regex(r"[a-zA-Z_-]+", symbol)]
    Identifier(IdentifierInfo)
}