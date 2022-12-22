use ckb_types::core::BlockNumber;
use ckb_types::packed::{Byte32, CellOutput, Bytes};

pub type TxIndex = u32;

pub struct DetailedLiveCell {
    pub block_number: BlockNumber,
    pub block_hash: Byte32,
    pub tx_index: TxIndex,
    pub cell_output: CellOutput,
    pub cell_data: Bytes,
}