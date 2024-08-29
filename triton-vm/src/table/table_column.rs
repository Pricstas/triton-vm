//! Enums that convert table column names into `usize` indices. Allows addressing columns by name
//! rather than their hard-to-remember index.

use std::hash::Hash;

use strum::Display;
use strum::EnumCount;
use strum::EnumIter;

use crate::table::degree_lowering_table::DegreeLoweringBaseTableColumn;
use crate::table::degree_lowering_table::DegreeLoweringExtTableColumn;
use crate::table::master_table::CASCADE_TABLE_START;
use crate::table::master_table::DEGREE_LOWERING_TABLE_START;
use crate::table::master_table::EXT_CASCADE_TABLE_START;
use crate::table::master_table::EXT_DEGREE_LOWERING_TABLE_START;
use crate::table::master_table::EXT_HASH_TABLE_START;
use crate::table::master_table::EXT_JUMP_STACK_TABLE_START;
use crate::table::master_table::EXT_LOOKUP_TABLE_START;
use crate::table::master_table::EXT_OP_STACK_TABLE_START;
use crate::table::master_table::EXT_PROCESSOR_TABLE_START;
use crate::table::master_table::EXT_PROGRAM_TABLE_START;
use crate::table::master_table::EXT_RAM_TABLE_START;
use crate::table::master_table::EXT_U32_TABLE_START;
use crate::table::master_table::HASH_TABLE_START;
use crate::table::master_table::JUMP_STACK_TABLE_START;
use crate::table::master_table::LOOKUP_TABLE_START;
use crate::table::master_table::OP_STACK_TABLE_START;
use crate::table::master_table::PROCESSOR_TABLE_START;
use crate::table::master_table::PROGRAM_TABLE_START;
use crate::table::master_table::RAM_TABLE_START;
use crate::table::master_table::U32_TABLE_START;

#[repr(usize)]
#[derive(Debug, Display, Copy, Clone, Eq, PartialEq, Hash, EnumCount, EnumIter)]
pub enum ProgramBaseTableColumn {
    /// An instruction's address.
    Address,

    /// The (opcode of the) instruction.
    Instruction,

    /// How often an instruction has been executed.
    LookupMultiplicity,

    /// The index in the vector of length [`Rate`] that is to be absorbed in the Sponge
    /// in order to compute the program's digest.
    /// In other words:
    /// [`Address`] modulo [`Rate`].
    ///
    /// [`Address`]: ProgramBaseTableColumn::Address
    /// [`Rate`]: twenty_first::math::tip5::RATE
    IndexInChunk,

    /// The inverse-or-zero of [`Rate`] - 1 - [`IndexInChunk`].
    /// Helper variable to guarantee [`IndexInChunk`]'s correct transition.
    ///
    /// [`IndexInChunk`]: ProgramBaseTableColumn::IndexInChunk
    /// [`Rate`]: twenty_first::math::tip5::RATE
    MaxMinusIndexInChunkInv,

    /// Padding indicator for absorbing the program into the Sponge.
    IsHashInputPadding,

    /// Padding indicator for rows only required due to the dominating length of some other table.
    IsTablePadding,
}

#[repr(usize)]
#[derive(Debug, Display, Copy, Clone, Eq, PartialEq, Hash, EnumCount, EnumIter)]
pub enum ProgramExtTableColumn {
    /// The server part of the instruction lookup.
    ///
    /// The counterpart to [`InstructionLookupClientLogDerivative`][client].
    ///
    /// [client]: ProcessorExtTableColumn::InstructionLookupClientLogDerivative
    InstructionLookupServerLogDerivative,

    /// An evaluation argument accumulating [`RATE`][rate] many instructions before
    /// they are sent using [`SendChunkEvalArg`](ProgramExtTableColumn::SendChunkRunningEvaluation).
    /// Resets to zero after each chunk.
    /// Relevant for program attestation.
    ///
    /// [rate]: twenty_first::math::tip5::RATE
    PrepareChunkRunningEvaluation,

    /// An evaluation argument over all [`RATE`][rate]-sized chunks of instructions,
    /// which are prepared in [`PrepareChunkEvalArg`][prep].
    /// This bus is used for sending those chunks to the Hash Table.
    /// Relevant for program attestation.
    ///
    /// The counterpart to [`RcvChunkEvalArg`](HashExtTableColumn::ReceiveChunkRunningEvaluation).
    ///
    /// [rate]: twenty_first::math::tip5::RATE
    /// [prep]: ProgramExtTableColumn::PrepareChunkRunningEvaluation
    SendChunkRunningEvaluation,
}

#[repr(usize)]
#[derive(Debug, Display, Copy, Clone, Eq, PartialEq, Hash, EnumCount, EnumIter)]
pub enum ProcessorBaseTableColumn {
    CLK,
    IsPadding,
    IP,
    CI,
    NIA,
    IB0,
    IB1,
    IB2,
    IB3,
    IB4,
    IB5,
    IB6,
    JSP,
    JSO,
    JSD,
    ST0,
    ST1,
    ST2,
    ST3,
    ST4,
    ST5,
    ST6,
    ST7,
    ST8,
    ST9,
    ST10,
    ST11,
    ST12,
    ST13,
    ST14,
    ST15,
    OpStackPointer,
    HV0,
    HV1,
    HV2,
    HV3,
    HV4,
    HV5,
    /// The number of clock jump differences of magnitude `CLK` in all memory-like tables.
    ClockJumpDifferenceLookupMultiplicity,
}

#[repr(usize)]
#[derive(Debug, Display, Copy, Clone, Eq, PartialEq, Hash, EnumCount, EnumIter)]
pub enum ProcessorExtTableColumn {
    InputTableEvalArg,
    OutputTableEvalArg,
    InstructionLookupClientLogDerivative,
    OpStackTablePermArg,
    RamTablePermArg,
    JumpStackTablePermArg,

    /// For copying the hash function's input to the hash coprocessor.
    HashInputEvalArg,
    /// For copying the hash digest from the hash coprocessor.
    HashDigestEvalArg,
    /// For copying the RATE next to-be-absorbed to the hash coprocessor and the RATE squeezed
    /// elements from the hash coprocessor, depending on the executed instruction.
    SpongeEvalArg,

    /// The (running sum of the) logarithmic derivative for the Lookup Argument with the U32 Table.
    U32LookupClientLogDerivative,

    /// The (running sum of the) logarithmic derivative for the clock jump difference Lookup
    /// Argument with the memory-like tables.
    ClockJumpDifferenceLookupServerLogDerivative,
}

#[repr(usize)]
#[derive(Debug, Display, Copy, Clone, Eq, PartialEq, Hash, EnumCount, EnumIter)]
pub enum OpStackBaseTableColumn {
    CLK,
    IB1ShrinkStack,
    StackPointer,
    FirstUnderflowElement,
}

#[repr(usize)]
#[derive(Debug, Display, Copy, Clone, Eq, PartialEq, Hash, EnumCount, EnumIter)]
pub enum OpStackExtTableColumn {
    RunningProductPermArg,
    /// The (running sum of the) logarithmic derivative for the clock jump difference Lookup
    /// Argument with the Processor Table.
    ClockJumpDifferenceLookupClientLogDerivative,
}

#[repr(usize)]
#[derive(Debug, Display, Copy, Clone, Eq, PartialEq, Hash, EnumCount, EnumIter)]
pub enum RamBaseTableColumn {
    CLK,

    /// Is [`INSTRUCTION_TYPE_READ`] for instruction `read_mem` and [`INSTRUCTION_TYPE_WRITE`]
    /// for instruction `write_mem`. For padding rows, this is set to [`PADDING_INDICATOR`].
    ///
    /// [`INSTRUCTION_TYPE_READ`]: crate::table::ram_table::INSTRUCTION_TYPE_READ
    /// [`INSTRUCTION_TYPE_WRITE`]: crate::table::ram_table::INSTRUCTION_TYPE_WRITE
    /// [`PADDING_INDICATOR`]: crate::table::ram_table::PADDING_INDICATOR
    InstructionType,
    RamPointer,
    RamValue,
    InverseOfRampDifference,
    BezoutCoefficientPolynomialCoefficient0,
    BezoutCoefficientPolynomialCoefficient1,
}

#[repr(usize)]
#[derive(Debug, Display, Copy, Clone, Eq, PartialEq, Hash, EnumCount, EnumIter)]
pub enum RamExtTableColumn {
    RunningProductOfRAMP,
    FormalDerivative,
    BezoutCoefficient0,
    BezoutCoefficient1,
    RunningProductPermArg,
    /// The (running sum of the) logarithmic derivative for the clock jump difference Lookup
    /// Argument with the Processor Table.
    ClockJumpDifferenceLookupClientLogDerivative,
}

#[repr(usize)]
#[derive(Debug, Display, Copy, Clone, Eq, PartialEq, Hash, EnumCount, EnumIter)]
pub enum JumpStackBaseTableColumn {
    CLK,
    CI,
    JSP,
    JSO,
    JSD,
}

#[repr(usize)]
#[derive(Debug, Display, Copy, Clone, Eq, PartialEq, Hash, EnumCount, EnumIter)]
pub enum JumpStackExtTableColumn {
    RunningProductPermArg,
    /// The (running sum of the) logarithmic derivative for the clock jump difference Lookup
    /// Argument with the Processor Table.
    ClockJumpDifferenceLookupClientLogDerivative,
}

#[repr(usize)]
#[derive(Debug, Display, Copy, Clone, Eq, PartialEq, Hash, EnumCount, EnumIter)]
pub enum HashBaseTableColumn {
    /// The indicator for the [`HashTableMode`][mode].
    ///
    /// [mode]: crate::table::hash_table::HashTableMode
    Mode,

    /// The current instruction. Only relevant for [`Mode`][mode] [`Sponge`][mode_sponge]
    /// in order to distinguish between the different Sponge instructions.
    ///
    /// [mode]: HashBaseTableColumn::Mode
    /// [mode_sponge]: crate::table::hash_table::HashTableMode::Sponge
    CI,

    /// The number of the current round in the permutation. The round number evolves as
    /// - 0 → 1 → 2 → 3 → 4 → 5 (→ 0) in [`Mode`][mode]s
    ///   [`ProgramHashing`][mode_prog_hash], [`Sponge`][mode_sponge] and [`Hash`][mode_hash],
    /// - 0 → 0 in [`Mode`][mode] [`Sponge`][mode_sponge] if the current instruction [`CI`][ci] is
    ///   `sponge_init`, as an exception to above rule, and
    /// - 0 → 0 in [`Mode`][mode] [`Pad`][mode_pad].
    ///
    /// [ci]: HashBaseTableColumn::CI
    /// [mode]: HashBaseTableColumn::Mode
    /// [mode_prog_hash]: crate::table::hash_table::HashTableMode::ProgramHashing
    /// [mode_sponge]: crate::table::hash_table::HashTableMode::Sponge
    /// [mode_hash]: crate::table::hash_table::HashTableMode::Hash
    /// [mode_pad]: crate::table::hash_table::HashTableMode::Pad
    RoundNumber,

    State0HighestLkIn,
    State0MidHighLkIn,
    State0MidLowLkIn,
    State0LowestLkIn,
    State1HighestLkIn,
    State1MidHighLkIn,
    State1MidLowLkIn,
    State1LowestLkIn,
    State2HighestLkIn,
    State2MidHighLkIn,
    State2MidLowLkIn,
    State2LowestLkIn,
    State3HighestLkIn,
    State3MidHighLkIn,
    State3MidLowLkIn,
    State3LowestLkIn,
    State0HighestLkOut,
    State0MidHighLkOut,
    State0MidLowLkOut,
    State0LowestLkOut,
    State1HighestLkOut,
    State1MidHighLkOut,
    State1MidLowLkOut,
    State1LowestLkOut,
    State2HighestLkOut,
    State2MidHighLkOut,
    State2MidLowLkOut,
    State2LowestLkOut,
    State3HighestLkOut,
    State3MidHighLkOut,
    State3MidLowLkOut,
    State3LowestLkOut,
    State4,
    State5,
    State6,
    State7,
    State8,
    State9,
    State10,
    State11,
    State12,
    State13,
    State14,
    State15,

    State0Inv,
    State1Inv,
    State2Inv,
    State3Inv,

    Constant0,
    Constant1,
    Constant2,
    Constant3,
    Constant4,
    Constant5,
    Constant6,
    Constant7,
    Constant8,
    Constant9,
    Constant10,
    Constant11,
    Constant12,
    Constant13,
    Constant14,
    Constant15,
}

#[repr(usize)]
#[derive(Debug, Display, Copy, Clone, Eq, PartialEq, Hash, EnumCount, EnumIter)]
pub enum HashExtTableColumn {
    /// The evaluation argument corresponding to receiving instructions in chunks of size
    /// [`RATE`][rate]. The chunks are hashed in Sponge mode.
    /// This allows program attestation.
    ///
    /// The counterpart to [`SendChunkEvalArg`](ProgramExtTableColumn::SendChunkRunningEvaluation).
    ///
    /// [rate]: twenty_first::math::tip5::RATE
    ReceiveChunkRunningEvaluation,

    HashInputRunningEvaluation,
    HashDigestRunningEvaluation,

    SpongeRunningEvaluation,

    CascadeState0HighestClientLogDerivative,
    CascadeState0MidHighClientLogDerivative,
    CascadeState0MidLowClientLogDerivative,
    CascadeState0LowestClientLogDerivative,

    CascadeState1HighestClientLogDerivative,
    CascadeState1MidHighClientLogDerivative,
    CascadeState1MidLowClientLogDerivative,
    CascadeState1LowestClientLogDerivative,

    CascadeState2HighestClientLogDerivative,
    CascadeState2MidHighClientLogDerivative,
    CascadeState2MidLowClientLogDerivative,
    CascadeState2LowestClientLogDerivative,

    CascadeState3HighestClientLogDerivative,
    CascadeState3MidHighClientLogDerivative,
    CascadeState3MidLowClientLogDerivative,
    CascadeState3LowestClientLogDerivative,
}

#[repr(usize)]
#[derive(Debug, Display, Copy, Clone, Eq, PartialEq, Hash, EnumCount, EnumIter)]
pub enum CascadeBaseTableColumn {
    /// Indicator for padding rows.
    IsPadding,

    /// The more significant bits of the lookup input.
    LookInHi,

    /// The less significant bits of the lookup input.
    LookInLo,

    /// The more significant bits of the lookup output.
    LookOutHi,

    /// The less significant bits of the lookup output.
    LookOutLo,

    /// The number of times the S-Box is evaluated, _i.e._, the value is looked up.
    LookupMultiplicity,
}

#[repr(usize)]
#[derive(Debug, Display, Copy, Clone, Eq, PartialEq, Hash, EnumCount, EnumIter)]
pub enum CascadeExtTableColumn {
    /// The (running sum of the) logarithmic derivative for the Lookup Argument with the Hash Table.
    /// In every row, the sum accumulates `LookupMultiplicity / (X - Combo)` where `X` is a
    /// verifier-supplied challenge and `Combo` is the weighted sum of
    /// - `2^8·LookInHi + LookInLo`, and
    /// - `2^8·LookOutHi + LookOutLo`
    ///   with weights supplied by the verifier.
    HashTableServerLogDerivative,

    /// The (running sum of the) logarithmic derivative for the Lookup Argument with the Lookup
    /// Table. In every row, accumulates the two summands
    /// - `1 / combo_hi` where `combo_hi` is the verifier-weighted combination of `LookInHi` and
    ///   `LookOutHi`, and
    /// - `1 / combo_lo` where `combo_lo` is the verifier-weighted combination of `LookInLo` and
    ///   `LookOutLo`.
    LookupTableClientLogDerivative,
}

#[repr(usize)]
#[derive(Debug, Display, Copy, Clone, Eq, PartialEq, Hash, EnumCount, EnumIter)]
pub enum LookupBaseTableColumn {
    /// Indicator for padding rows.
    IsPadding,

    /// The lookup input.
    LookIn,

    /// The lookup output.
    LookOut,

    /// The number of times the value is looked up.
    LookupMultiplicity,
}

#[repr(usize)]
#[derive(Debug, Display, Copy, Clone, Eq, PartialEq, Hash, EnumCount, EnumIter)]
pub enum LookupExtTableColumn {
    /// The (running sum of the) logarithmic derivative for the Lookup Argument with the Cascade
    /// Table. In every row, accumulates the summand `LookupMultiplicity / Combo` where `Combo` is
    /// the verifier-weighted combination of `LookIn` and `LookOut`.
    CascadeTableServerLogDerivative,

    /// The running sum for the public evaluation argument of the Lookup Table.
    /// In every row, accumulates `LookOut`.
    PublicEvaluationArgument,
}

#[repr(usize)]
#[derive(Debug, Display, Copy, Clone, Eq, PartialEq, Hash, EnumCount, EnumIter)]
pub enum U32BaseTableColumn {
    /// Marks the beginning of an independent section within the U32 table.
    CopyFlag,

    /// The number of bits that LHS and RHS have already been shifted by.
    Bits,

    /// The inverse-or-zero of the difference between
    /// 1. the first disallowed number of bits to shift LHS and RHS by, _i.e.,_ 33, and
    /// 2. the number of bits that LHS and RHS have already been shifted by.
    BitsMinus33Inv,

    /// Current Instruction, the instruction the processor is currently executing.
    CI,

    /// Left-hand side of the operation.
    LHS,

    /// The inverse-or-zero of LHS. Needed to check whether `LHS` is unequal to 0.
    LhsInv,

    /// Right-hand side of the operation.
    RHS,

    /// The inverse-or-zero of RHS. Needed to check whether `RHS` is unequal to 0.
    RhsInv,

    /// The result (or intermediate result) of the instruction requested by the processor.
    Result,

    /// The number of times the processor has executed the current instruction with the same
    /// arguments.
    LookupMultiplicity,
}

#[repr(usize)]
#[derive(Debug, Display, Copy, Clone, Eq, PartialEq, Hash, EnumCount, EnumIter)]
pub enum U32ExtTableColumn {
    /// The (running sum of the) logarithmic derivative for the Lookup Argument with the
    /// Processor Table.
    LookupServerLogDerivative,
}

/// A trait for the columns of the master base table. This trait is implemented for all enums
/// relating to the base tables. This trait provides two methods:
/// - one to get the index of the column in the “local” base table, _i.e., not the master base
///   table, and
/// - one to get the index of the column in the master base table.
pub trait MasterBaseTableColumn {
    /// The index of the column in the “local” base table, _i.e., not the master base table.
    fn base_table_index(&self) -> usize;

    /// The index of the column in the master base table.
    fn master_base_table_index(&self) -> usize;
}

impl MasterBaseTableColumn for ProgramBaseTableColumn {
    #[inline]
    fn base_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_base_table_index(&self) -> usize {
        PROGRAM_TABLE_START + self.base_table_index()
    }
}

impl MasterBaseTableColumn for ProcessorBaseTableColumn {
    #[inline]
    fn base_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_base_table_index(&self) -> usize {
        PROCESSOR_TABLE_START + self.base_table_index()
    }
}

impl MasterBaseTableColumn for OpStackBaseTableColumn {
    #[inline]
    fn base_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_base_table_index(&self) -> usize {
        OP_STACK_TABLE_START + self.base_table_index()
    }
}

impl MasterBaseTableColumn for RamBaseTableColumn {
    #[inline]
    fn base_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_base_table_index(&self) -> usize {
        RAM_TABLE_START + self.base_table_index()
    }
}

impl MasterBaseTableColumn for JumpStackBaseTableColumn {
    #[inline]
    fn base_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_base_table_index(&self) -> usize {
        JUMP_STACK_TABLE_START + self.base_table_index()
    }
}

impl MasterBaseTableColumn for HashBaseTableColumn {
    #[inline]
    fn base_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_base_table_index(&self) -> usize {
        HASH_TABLE_START + self.base_table_index()
    }
}

impl MasterBaseTableColumn for CascadeBaseTableColumn {
    #[inline]
    fn base_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_base_table_index(&self) -> usize {
        CASCADE_TABLE_START + self.base_table_index()
    }
}

impl MasterBaseTableColumn for LookupBaseTableColumn {
    #[inline]
    fn base_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_base_table_index(&self) -> usize {
        LOOKUP_TABLE_START + self.base_table_index()
    }
}

impl MasterBaseTableColumn for U32BaseTableColumn {
    #[inline]
    fn base_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_base_table_index(&self) -> usize {
        U32_TABLE_START + self.base_table_index()
    }
}

impl MasterBaseTableColumn for DegreeLoweringBaseTableColumn {
    #[inline]
    fn base_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_base_table_index(&self) -> usize {
        DEGREE_LOWERING_TABLE_START + self.base_table_index()
    }
}

/// A trait for the columns in the master extension table. This trait is implemented for all enums
/// relating to the extension tables. The trait provides two methods:
/// - one to get the index of the column in the “local” extension table, _i.e._, not the master
///   extension table, and
/// - one to get the index of the column in the master extension table.
pub trait MasterExtTableColumn {
    /// The index of the column in the “local” extension table, _i.e._, not the master extension
    /// table.
    fn ext_table_index(&self) -> usize;

    /// The index of the column in the master extension table.
    fn master_ext_table_index(&self) -> usize;
}

impl MasterExtTableColumn for ProgramExtTableColumn {
    #[inline]
    fn ext_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_ext_table_index(&self) -> usize {
        EXT_PROGRAM_TABLE_START + self.ext_table_index()
    }
}

impl MasterExtTableColumn for ProcessorExtTableColumn {
    #[inline]
    fn ext_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_ext_table_index(&self) -> usize {
        EXT_PROCESSOR_TABLE_START + self.ext_table_index()
    }
}

impl MasterExtTableColumn for OpStackExtTableColumn {
    #[inline]
    fn ext_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_ext_table_index(&self) -> usize {
        EXT_OP_STACK_TABLE_START + self.ext_table_index()
    }
}

impl MasterExtTableColumn for RamExtTableColumn {
    #[inline]
    fn ext_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_ext_table_index(&self) -> usize {
        EXT_RAM_TABLE_START + self.ext_table_index()
    }
}

impl MasterExtTableColumn for JumpStackExtTableColumn {
    #[inline]
    fn ext_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_ext_table_index(&self) -> usize {
        EXT_JUMP_STACK_TABLE_START + self.ext_table_index()
    }
}

impl MasterExtTableColumn for HashExtTableColumn {
    #[inline]
    fn ext_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_ext_table_index(&self) -> usize {
        EXT_HASH_TABLE_START + self.ext_table_index()
    }
}

impl MasterExtTableColumn for CascadeExtTableColumn {
    #[inline]
    fn ext_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_ext_table_index(&self) -> usize {
        EXT_CASCADE_TABLE_START + self.ext_table_index()
    }
}

impl MasterExtTableColumn for LookupExtTableColumn {
    #[inline]
    fn ext_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_ext_table_index(&self) -> usize {
        EXT_LOOKUP_TABLE_START + self.ext_table_index()
    }
}

impl MasterExtTableColumn for U32ExtTableColumn {
    #[inline]
    fn ext_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_ext_table_index(&self) -> usize {
        EXT_U32_TABLE_START + self.ext_table_index()
    }
}

impl MasterExtTableColumn for DegreeLoweringExtTableColumn {
    #[inline]
    fn ext_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_ext_table_index(&self) -> usize {
        EXT_DEGREE_LOWERING_TABLE_START + self.ext_table_index()
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use crate::table::cascade_table;
    use crate::table::hash_table;
    use crate::table::jump_stack_table;
    use crate::table::lookup_table;
    use crate::table::op_stack_table;
    use crate::table::processor_table;
    use crate::table::program_table;
    use crate::table::ram_table;
    use crate::table::u32_table;

    use super::*;

    #[test]
    fn column_max_bound_matches_table_width() {
        assert_eq!(
            program_table::BASE_WIDTH,
            ProgramBaseTableColumn::iter()
                .last()
                .unwrap()
                .base_table_index()
                + 1,
            "ProgramTable's BASE_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            processor_table::BASE_WIDTH,
            ProcessorBaseTableColumn::iter()
                .last()
                .unwrap()
                .base_table_index()
                + 1,
            "ProcessorTable's BASE_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            op_stack_table::BASE_WIDTH,
            OpStackBaseTableColumn::iter()
                .last()
                .unwrap()
                .base_table_index()
                + 1,
            "OpStackTable's BASE_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            ram_table::BASE_WIDTH,
            RamBaseTableColumn::iter()
                .last()
                .unwrap()
                .base_table_index()
                + 1,
            "RamTable's BASE_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            jump_stack_table::BASE_WIDTH,
            JumpStackBaseTableColumn::iter()
                .last()
                .unwrap()
                .base_table_index()
                + 1,
            "JumpStackTable's BASE_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            hash_table::BASE_WIDTH,
            HashBaseTableColumn::iter()
                .last()
                .unwrap()
                .base_table_index()
                + 1,
            "HashTable's BASE_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            cascade_table::BASE_WIDTH,
            CascadeBaseTableColumn::iter()
                .last()
                .unwrap()
                .base_table_index()
                + 1,
            "CascadeTable's BASE_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            lookup_table::BASE_WIDTH,
            LookupBaseTableColumn::iter()
                .last()
                .unwrap()
                .base_table_index()
                + 1,
            "LookupTable's BASE_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            u32_table::BASE_WIDTH,
            U32BaseTableColumn::iter()
                .last()
                .unwrap()
                .base_table_index()
                + 1,
            "U32Table's BASE_WIDTH is 1 + its max column index",
        );

        assert_eq!(
            program_table::EXT_WIDTH,
            ProgramExtTableColumn::iter()
                .last()
                .unwrap()
                .ext_table_index()
                + 1,
            "ProgramTable's EXT_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            processor_table::EXT_WIDTH,
            ProcessorExtTableColumn::iter()
                .last()
                .unwrap()
                .ext_table_index()
                + 1,
            "ProcessorTable's EXT_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            op_stack_table::EXT_WIDTH,
            OpStackExtTableColumn::iter()
                .last()
                .unwrap()
                .ext_table_index()
                + 1,
            "OpStack:Table's EXT_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            ram_table::EXT_WIDTH,
            RamExtTableColumn::iter().last().unwrap().ext_table_index() + 1,
            "RamTable's EXT_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            jump_stack_table::EXT_WIDTH,
            JumpStackExtTableColumn::iter()
                .last()
                .unwrap()
                .ext_table_index()
                + 1,
            "JumpStack:Table's EXT_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            hash_table::EXT_WIDTH,
            HashExtTableColumn::iter().last().unwrap().ext_table_index() + 1,
            "HashTable's EXT_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            cascade_table::EXT_WIDTH,
            CascadeExtTableColumn::iter()
                .last()
                .unwrap()
                .ext_table_index()
                + 1,
            "CascadeTable's EXT_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            lookup_table::EXT_WIDTH,
            LookupExtTableColumn::iter()
                .last()
                .unwrap()
                .ext_table_index()
                + 1,
            "LookupTable's EXT_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            u32_table::EXT_WIDTH,
            U32ExtTableColumn::iter().last().unwrap().ext_table_index() + 1,
            "U32Table's EXT_WIDTH is 1 + its max column index",
        );
    }

    #[test]
    fn master_base_table_is_contiguous() {
        let mut expected_column_index = 0;
        for column in ProgramBaseTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_base_table_index());
            expected_column_index += 1;
        }
        for column in ProcessorBaseTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_base_table_index());
            expected_column_index += 1;
        }
        for column in OpStackBaseTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_base_table_index());
            expected_column_index += 1;
        }
        for column in RamBaseTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_base_table_index());
            expected_column_index += 1;
        }
        for column in JumpStackBaseTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_base_table_index());
            expected_column_index += 1;
        }
        for column in HashBaseTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_base_table_index());
            expected_column_index += 1;
        }
        for column in CascadeBaseTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_base_table_index());
            expected_column_index += 1;
        }
        for column in LookupBaseTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_base_table_index());
            expected_column_index += 1;
        }
        for column in U32BaseTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_base_table_index());
            expected_column_index += 1;
        }
        for column in DegreeLoweringBaseTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_base_table_index());
            expected_column_index += 1;
        }
    }

    #[test]
    fn master_ext_table_is_contiguous() {
        let mut expected_column_index = 0;
        for column in ProgramExtTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_ext_table_index());
            expected_column_index += 1;
        }
        for column in ProcessorExtTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_ext_table_index());
            expected_column_index += 1;
        }
        for column in OpStackExtTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_ext_table_index());
            expected_column_index += 1;
        }
        for column in RamExtTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_ext_table_index());
            expected_column_index += 1;
        }
        for column in JumpStackExtTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_ext_table_index());
            expected_column_index += 1;
        }
        for column in HashExtTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_ext_table_index());
            expected_column_index += 1;
        }
        for column in CascadeExtTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_ext_table_index());
            expected_column_index += 1;
        }
        for column in LookupExtTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_ext_table_index());
            expected_column_index += 1;
        }
        for column in U32ExtTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_ext_table_index());
            expected_column_index += 1;
        }
        for column in DegreeLoweringExtTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_ext_table_index());
            expected_column_index += 1;
        }
    }
}
