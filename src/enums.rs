pub enum Segment {
    None,
    Const,
    Text,
    Data,
}

pub enum LineType {
    Instruction,
    Variable,
    Untyped,
}

pub enum VarType {
    Word,
    Dword,
    Long,
    String,
    None,
}

pub enum InstrType {
    Alu,
    AluConst,
    AluTest,
    AluOneSrc,
    AluRot,
    MemIgpr,
    MemImp,
    MemDirect,
    Mem,
    MemRo,
    MemIo,
    Jmp,
    AddrArithm,
    AddrArImm,
    Mov,
    Movs,
    Misc3bit,
    Misc8bit,
    Misc,
}

pub enum ArgType {
    Gpr,
    Mpr,
    MP,
    Special,
    Value,
}