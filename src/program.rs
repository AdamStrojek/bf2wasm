#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
	PtrInc(usize),
	PtrDec(usize),
	ValInc(u8),
	ValDec(u8),
	GetCh,
	PutCh,
	Block(Vec<Instruction>),
}
