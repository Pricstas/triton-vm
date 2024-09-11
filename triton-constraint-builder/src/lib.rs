use air::cross_table_argument::GrandCrossTableArg;
use air::table::cascade::CascadeTable;
use air::table::hash::HashTable;
use air::table::jump_stack::JumpStackTable;
use air::table::lookup::LookupTable;
use air::table::op_stack::OpStackTable;
use air::table::processor::ProcessorTable;
use air::table::program::ProgramTable;
use air::table::ram::RamTable;
use air::table::u32::U32Table;
use air::AIR;
use constraint_circuit::ConstraintCircuit;
use constraint_circuit::ConstraintCircuitBuilder;
use constraint_circuit::ConstraintCircuitMonad;
use constraint_circuit::DegreeLoweringInfo;
use constraint_circuit::DualRowIndicator;
use constraint_circuit::InputIndicator;
use constraint_circuit::SingleRowIndicator;
use itertools::Itertools;
use proc_macro2::TokenStream;
use std::fs::write;

use crate::codegen::Codegen;
use crate::codegen::RustBackend;
use crate::codegen::TasmBackend;
use crate::substitutions::AllSubstitutions;
use crate::substitutions::Substitutions;

pub mod codegen;
mod substitutions;

pub fn gen(mut constraints: Constraints, info: DegreeLoweringInfo) {
    let substitutions = constraints.lower_to_target_degree_through_substitutions(info);
    let degree_lowering_table_code = substitutions.generate_degree_lowering_table_code();

    let constraints = constraints.combine_with_substitution_induced_constraints(substitutions);
    let rust = RustBackend::constraint_evaluation_code(&constraints);
    let tasm = TasmBackend::constraint_evaluation_code(&constraints);

    write_code_to_file(
        degree_lowering_table_code,
        "triton-vm/src/table/degree_lowering_table.rs",
    );
    write_code_to_file(rust, "triton-vm/src/table/constraints.rs");
    write_code_to_file(tasm, "triton-vm/src/air/tasm_air_constraints.rs");
}

fn write_code_to_file(code: TokenStream, file_name: &str) {
    let syntax_tree = syn::parse2(code).unwrap();
    let code = prettyplease::unparse(&syntax_tree);
    write(file_name, code).unwrap();
}

#[derive(Debug, Clone)]
pub struct Constraints {
    pub init: Vec<ConstraintCircuitMonad<SingleRowIndicator>>,
    pub cons: Vec<ConstraintCircuitMonad<SingleRowIndicator>>,
    pub tran: Vec<ConstraintCircuitMonad<DualRowIndicator>>,
    pub term: Vec<ConstraintCircuitMonad<SingleRowIndicator>>,
}

impl Constraints {
    pub fn all() -> Constraints {
        Constraints {
            init: Self::initial_constraints(),
            cons: Self::consistency_constraints(),
            tran: Self::transition_constraints(),
            term: Self::terminal_constraints(),
        }
    }

    pub fn initial_constraints() -> Vec<ConstraintCircuitMonad<SingleRowIndicator>> {
        let circuit_builder = ConstraintCircuitBuilder::new();
        vec![
            ProgramTable::initial_constraints(&circuit_builder),
            ProcessorTable::initial_constraints(&circuit_builder),
            OpStackTable::initial_constraints(&circuit_builder),
            RamTable::initial_constraints(&circuit_builder),
            JumpStackTable::initial_constraints(&circuit_builder),
            HashTable::initial_constraints(&circuit_builder),
            CascadeTable::initial_constraints(&circuit_builder),
            LookupTable::initial_constraints(&circuit_builder),
            U32Table::initial_constraints(&circuit_builder),
            GrandCrossTableArg::initial_constraints(&circuit_builder),
        ]
        .concat()
    }

    pub fn consistency_constraints() -> Vec<ConstraintCircuitMonad<SingleRowIndicator>> {
        let circuit_builder = ConstraintCircuitBuilder::new();
        vec![
            ProgramTable::consistency_constraints(&circuit_builder),
            ProcessorTable::consistency_constraints(&circuit_builder),
            OpStackTable::consistency_constraints(&circuit_builder),
            RamTable::consistency_constraints(&circuit_builder),
            JumpStackTable::consistency_constraints(&circuit_builder),
            HashTable::consistency_constraints(&circuit_builder),
            CascadeTable::consistency_constraints(&circuit_builder),
            LookupTable::consistency_constraints(&circuit_builder),
            U32Table::consistency_constraints(&circuit_builder),
            GrandCrossTableArg::consistency_constraints(&circuit_builder),
        ]
        .concat()
    }

    pub fn transition_constraints() -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        let circuit_builder = ConstraintCircuitBuilder::new();
        vec![
            ProgramTable::transition_constraints(&circuit_builder),
            ProcessorTable::transition_constraints(&circuit_builder),
            OpStackTable::transition_constraints(&circuit_builder),
            RamTable::transition_constraints(&circuit_builder),
            JumpStackTable::transition_constraints(&circuit_builder),
            HashTable::transition_constraints(&circuit_builder),
            CascadeTable::transition_constraints(&circuit_builder),
            LookupTable::transition_constraints(&circuit_builder),
            U32Table::transition_constraints(&circuit_builder),
            GrandCrossTableArg::transition_constraints(&circuit_builder),
        ]
        .concat()
    }

    pub fn terminal_constraints() -> Vec<ConstraintCircuitMonad<SingleRowIndicator>> {
        let circuit_builder = ConstraintCircuitBuilder::new();
        vec![
            ProgramTable::terminal_constraints(&circuit_builder),
            ProcessorTable::terminal_constraints(&circuit_builder),
            OpStackTable::terminal_constraints(&circuit_builder),
            RamTable::terminal_constraints(&circuit_builder),
            JumpStackTable::terminal_constraints(&circuit_builder),
            HashTable::terminal_constraints(&circuit_builder),
            CascadeTable::terminal_constraints(&circuit_builder),
            LookupTable::terminal_constraints(&circuit_builder),
            U32Table::terminal_constraints(&circuit_builder),
            GrandCrossTableArg::terminal_constraints(&circuit_builder),
        ]
        .concat()
    }

    pub fn lower_to_target_degree_through_substitutions(
        &mut self,
        lowering_info: DegreeLoweringInfo,
    ) -> AllSubstitutions {
        let mut info = lowering_info;

        let (init_base_substitutions, init_ext_substitutions) =
            ConstraintCircuitMonad::lower_to_degree(&mut self.init, info);
        info.num_main_cols += init_base_substitutions.len();
        info.num_aux_cols += init_ext_substitutions.len();

        let (cons_base_substitutions, cons_ext_substitutions) =
            ConstraintCircuitMonad::lower_to_degree(&mut self.cons, info);
        info.num_main_cols += cons_base_substitutions.len();
        info.num_aux_cols += cons_ext_substitutions.len();

        let (tran_base_substitutions, tran_ext_substitutions) =
            ConstraintCircuitMonad::lower_to_degree(&mut self.tran, info);
        info.num_main_cols += tran_base_substitutions.len();
        info.num_aux_cols += tran_ext_substitutions.len();

        let (term_base_substitutions, term_ext_substitutions) =
            ConstraintCircuitMonad::lower_to_degree(&mut self.term, info);

        AllSubstitutions {
            main: Substitutions {
                lowering_info,
                init: init_base_substitutions,
                cons: cons_base_substitutions,
                tran: tran_base_substitutions,
                term: term_base_substitutions,
            },
            aux: Substitutions {
                lowering_info,
                init: init_ext_substitutions,
                cons: cons_ext_substitutions,
                tran: tran_ext_substitutions,
                term: term_ext_substitutions,
            },
        }
    }

    #[must_use]
    pub fn combine_with_substitution_induced_constraints(
        self,
        AllSubstitutions {
            main: base,
            aux: ext,
        }: AllSubstitutions,
    ) -> Self {
        Self {
            init: [self.init, base.init, ext.init].concat(),
            cons: [self.cons, base.cons, ext.cons].concat(),
            tran: [self.tran, base.tran, ext.tran].concat(),
            term: [self.term, base.term, ext.term].concat(),
        }
    }

    pub fn init(&self) -> Vec<ConstraintCircuit<SingleRowIndicator>> {
        Self::consume(&self.init)
    }

    pub fn cons(&self) -> Vec<ConstraintCircuit<SingleRowIndicator>> {
        Self::consume(&self.cons)
    }

    pub fn tran(&self) -> Vec<ConstraintCircuit<DualRowIndicator>> {
        Self::consume(&self.tran)
    }

    pub fn term(&self) -> Vec<ConstraintCircuit<SingleRowIndicator>> {
        Self::consume(&self.term)
    }

    fn consume<II: InputIndicator>(
        constraints: &[ConstraintCircuitMonad<II>],
    ) -> Vec<ConstraintCircuit<II>> {
        let mut constraints = constraints.iter().map(|c| c.consume()).collect_vec();
        ConstraintCircuit::assert_unique_ids(&mut constraints);
        constraints
    }
}

#[cfg(test)]
mod tests {
    use constraint_circuit::ConstraintCircuitBuilder;
    use twenty_first::prelude::*;

    use super::*;

    #[repr(usize)]
    enum TestChallenges {
        Ch0,
        Ch1,
    }

    impl From<TestChallenges> for usize {
        fn from(challenge: TestChallenges) -> Self {
            challenge as usize
        }
    }

    fn degree_lowering_info() -> DegreeLoweringInfo {
        DegreeLoweringInfo {
            target_degree: 4,
            num_main_cols: 42,
            num_aux_cols: 13,
        }
    }

    #[test]
    fn public_types_implement_usual_auto_traits() {
        fn implements_auto_traits<T: Sized + Send + Sync + Unpin>() {}

        implements_auto_traits::<RustBackend>();
        implements_auto_traits::<TasmBackend>();

        // maybe some day
        // implements_auto_traits::<Constraints>();
        // implements_auto_traits::<substitutions::Substitutions>();
        // implements_auto_traits::<substitutions::AllSubstitutions>();
    }

    #[test]
    fn test_constraints_can_be_fetched() {
        Constraints::test_constraints();
    }

    #[test]
    fn degree_lowering_tables_code_can_be_generated_for_test_constraints() {
        let mut constraints = Constraints::test_constraints();
        let substitutions =
            constraints.lower_to_target_degree_through_substitutions(degree_lowering_info());
        let _unused = substitutions.generate_degree_lowering_table_code();
    }

    #[test]
    fn degree_lowering_tables_code_can_be_generated_from_all_constraints() {
        let mut constraints = Constraints::all();
        let substitutions =
            constraints.lower_to_target_degree_through_substitutions(degree_lowering_info());
        let _unused = substitutions.generate_degree_lowering_table_code();
    }

    #[test]
    fn constraints_and_substitutions_can_be_combined() {
        let mut constraints = Constraints::test_constraints();
        let substitutions =
            constraints.lower_to_target_degree_through_substitutions(degree_lowering_info());
        let _combined = constraints.combine_with_substitution_induced_constraints(substitutions);
    }

    impl Constraints {
        /// For testing purposes only. There is no meaning behind any of the constraints.
        pub(crate) fn test_constraints() -> Self {
            Self {
                init: Self::small_init_constraints(),
                cons: vec![],
                tran: Self::small_transition_constraints(),
                term: vec![],
            }
        }

        fn small_init_constraints() -> Vec<ConstraintCircuitMonad<SingleRowIndicator>> {
            let circuit_builder = ConstraintCircuitBuilder::new();
            let challenge = |c| circuit_builder.challenge(c);
            let constant = |c: u32| circuit_builder.b_constant(bfe!(c));
            let input = |i| circuit_builder.input(SingleRowIndicator::Main(i));
            let input_to_the_4th = |i| input(i) * input(i) * input(i) * input(i);

            vec![
                input(0) * input(1) - input(2),
                input_to_the_4th(0) - challenge(TestChallenges::Ch1) - constant(16),
                input(2) * input_to_the_4th(0) - input_to_the_4th(1),
            ]
        }

        fn small_transition_constraints() -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
            let circuit_builder = ConstraintCircuitBuilder::new();
            let challenge = |c| circuit_builder.challenge(c);
            let constant = |c: u32| circuit_builder.x_constant(c);

            let curr_b_row = |col| circuit_builder.input(DualRowIndicator::CurrentMain(col));
            let next_b_row = |col| circuit_builder.input(DualRowIndicator::NextMain(col));
            let curr_x_row = |col| circuit_builder.input(DualRowIndicator::CurrentAux(col));
            let next_x_row = |col| circuit_builder.input(DualRowIndicator::NextAux(col));

            vec![
                curr_b_row(0) * next_x_row(1) - next_b_row(1) * curr_x_row(0),
                curr_b_row(1) * next_x_row(2) - next_b_row(2) * curr_x_row(1),
                curr_b_row(2) * next_x_row(0) * next_x_row(1) * next_x_row(3) + constant(42),
                curr_b_row(0) * challenge(TestChallenges::Ch0) - challenge(TestChallenges::Ch1),
            ]
        }
    }
}