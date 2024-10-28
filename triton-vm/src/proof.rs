use arbitrary::Arbitrary;
use get_size::GetSize;
use isa::program::Program;
use serde::Deserialize;
use serde::Serialize;
use twenty_first::prelude::*;

use crate::error::ProofStreamError;
use crate::proof_stream::ProofStream;

/// A version tag for the combination of Triton VM's
/// [instruction set architecture (ISA)][isa] as well as the
/// [STARK proof system][crate::stark::Stark].
/// This version changes whenever either of the two changes.
///
/// # Rationale
///
/// A change in the ISA might give a [`Program`] a new meaning, and an existing
/// proof might erroneously attest to the “new” program's graceful halt. By
/// bumping this version when changing the ISA, the old proof is surely invalid
/// under the new version. If the program's meaning has not changed, or the new
/// meaning is accepted, a new proof can be generated.
///
/// A change in the STARK proof system generally means that the verifier has to
/// perform different operations to verify a proof. This means that existing
/// proofs about some program _should_ be accepted as valid, but (generally) are
/// not. This version helps to make the discrepancy explicit.
///
/// Note that proofs remain valid for their matching versions indefinitely.
///
/// This version is separate from the crate's semantic version to allow software
/// upgrades with no semantic changes to both, the ISA and the proof system.
pub const CURRENT_VERSION: u32 = 0;

/// Contains the necessary cryptographic information to verify a computation.
/// Should be used together with a [`Claim`].
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, GetSize, BFieldCodec, Arbitrary)]
pub struct Proof(pub Vec<BFieldElement>);

impl Proof {
    /// Get the height of the trace used during proof generation.
    /// This is an upper bound on the length of the computation this proof is for.
    /// It is one of the main contributing factors to the length of the FRI domain.
    pub fn padded_height(&self) -> Result<usize, ProofStreamError> {
        let mut log_2_padded_heights = ProofStream::try_from(self)?
            .items
            .into_iter()
            .filter_map(|item| item.try_into_log2_padded_height().ok());

        let log_2_padded_height = log_2_padded_heights
            .next()
            .ok_or(ProofStreamError::NoLog2PaddedHeight)?;
        if log_2_padded_heights.next().is_some() {
            return Err(ProofStreamError::TooManyLog2PaddedHeights);
        }

        Ok(1 << log_2_padded_height)
    }
}

/// Contains the public information of a verifiably correct computation.
/// A corresponding [`Proof`] is needed to verify the computation.
/// One additional piece of public information not explicitly listed in the [`Claim`] is the
/// `padded_height`, an upper bound on the length of the computation.
/// It is derivable from a [`Proof`] by calling [`Proof::padded_height()`].
#[derive(
    Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, GetSize, BFieldCodec, Arbitrary,
)]
pub struct Claim {
    /// The hash digest of the program that was executed. The hash function in use is [`Tip5`].
    pub program_digest: Digest,

    /// The version of the Triton VM instruction set architecture the
    /// [`program_digest`][digest] is about, as well as of the STARK proof system
    /// in use. See also: [`CURRENT_VERSION`].
    ///
    /// [digest]: Self::program_digest
    pub version: u32,

    /// The public input to the computation.
    pub input: Vec<BFieldElement>,

    /// The public output of the computation.
    pub output: Vec<BFieldElement>,
}

impl Claim {
    /// Create a new Claim.
    ///
    /// Assumes the version to be [`CURRENT_VERSION`]. The version can be changed
    /// with method [`about_version`][Self::about_version].
    pub fn new(program_digest: Digest) -> Self {
        Self {
            program_digest,
            version: CURRENT_VERSION,
            input: vec![],
            output: vec![],
        }
    }

    #[must_use]
    pub fn about_program(program: &Program) -> Self {
        Self::new(program.hash())
    }

    #[must_use]
    pub fn with_input(mut self, input: impl Into<Vec<BFieldElement>>) -> Self {
        self.input = input.into();
        self
    }

    #[must_use]
    pub fn with_output(mut self, output: Vec<BFieldElement>) -> Self {
        self.output = output;
        self
    }

    #[must_use]
    pub fn about_version(mut self, version: u32) -> Self {
        self.version = version;
        self
    }
}

#[cfg(test)]
mod tests {
    use assert2::assert;
    use fs_err as fs;
    use proptest::collection::vec;
    use proptest::prelude::*;
    use proptest_arbitrary_interop::arb;
    use test_strategy::proptest;

    use crate::prelude::*;
    use crate::proof_item::ProofItem;

    use super::*;

    impl Default for Claim {
        /// For testing purposes only.
        fn default() -> Self {
            Self::new(Digest::default())
        }
    }

    #[test]
    fn claim_accepts_various_types_for_public_input() {
        let _claim = Claim::default()
            .with_input(bfe_vec![42])
            .with_input(bfe_array![42])
            .with_input(PublicInput::new(bfe_vec![42]));
    }

    #[proptest]
    fn decode_proof(#[strategy(arb())] proof: Proof) {
        let encoded = proof.encode();
        let decoded = *Proof::decode(&encoded).unwrap();
        prop_assert_eq!(proof, decoded);
    }

    #[proptest]
    fn decode_claim(#[strategy(arb())] claim: Claim) {
        let encoded = claim.encode();
        let decoded = *Claim::decode(&encoded).unwrap();
        prop_assert_eq!(claim, decoded);
    }

    #[proptest(cases = 10)]
    fn proof_with_no_padded_height_gives_err(#[strategy(arb())] root: Digest) {
        let mut proof_stream = ProofStream::new();
        proof_stream.enqueue(ProofItem::MerkleRoot(root));
        let proof: Proof = proof_stream.into();
        let maybe_padded_height = proof.padded_height();
        assert!(maybe_padded_height.is_err());
    }

    #[proptest(cases = 10)]
    fn proof_with_multiple_padded_height_gives_err(#[strategy(arb())] root: Digest) {
        let mut proof_stream = ProofStream::new();
        proof_stream.enqueue(ProofItem::Log2PaddedHeight(8));
        proof_stream.enqueue(ProofItem::MerkleRoot(root));
        proof_stream.enqueue(ProofItem::Log2PaddedHeight(7));
        let proof: Proof = proof_stream.into();
        let maybe_padded_height = proof.padded_height();
        assert!(maybe_padded_height.is_err());
    }

    #[proptest]
    fn decoding_arbitrary_proof_data_does_not_panic(
        #[strategy(vec(arb(), 0..1_000))] proof_data: Vec<BFieldElement>,
    ) {
        let _proof = Proof::decode(&proof_data);
    }

    #[test]
    fn current_proof_version_is_still_current() {
        // todo: Once the prover can be de-randomized (issue #334), change this test.
        //  In particular:
        //  Seed prover, generate proof, hash the proof, compare to hardcoded digest.
        //  Remove the proof stored on disk, and remove dependency `bincode`.

        fn generate_proof_file(program: Program, claim: Claim) {
            let input = claim.input.clone().into();
            let non_determinism = NonDeterminism::default();
            let (aet, _) = VM::trace_execution(program, input, non_determinism).unwrap();
            let proof = Stark::default().prove(&claim, &aet).unwrap();
            let proof = bincode::serialize(&proof).unwrap();
            fs::create_dir_all("./test_data/").unwrap();
            fs::write("./test_data/current_version_is_current.proof.new", proof).unwrap();
            eprintln!("New proof generated. Delete “.new” from its file name & commit to accept.");
        }

        let program = triton_program! {
            pick 11 pick 12 pick 13 pick 14 pick 15
            read_io 5 assert_vector halt
        };
        let claim = Claim::about_program(&program).with_input(program.hash());

        let Ok(proof) = fs::read("./test_data/current_version_is_current.proof") else {
            generate_proof_file(program, claim);
            panic!("Proof file does not exist.");
        };
        let proof = bincode::deserialize(&proof).unwrap();

        if Stark::default().verify(&claim, &proof).is_err() {
            generate_proof_file(program, claim);
            panic!("Verification of existing proof failed. Need to bump proof version?");
        };
    }
}
