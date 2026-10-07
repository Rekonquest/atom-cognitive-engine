//! The cognitive engine: executes the canonical cycles over the root
//! stages, driven by whatever ordering-validated stack it is handed.
//!
//! Execution is stage-order-driven by design: hand the engine a mis-ordered
//! (but legal) stack and it genuinely executes in that order — which is how
//! the permutation suite proves the ordering laws, and how the ungated
//! query variant (the O7 failure mode, kept deliberately reachable) shows
//! what happens without the gate.

pub mod config;
pub mod engine;
pub mod stacks;
pub mod trace;

pub use config::EngineConfig;
pub use engine::{
    BlendReport, BottleneckReport, CognitiveEngine, EngineError, FluencyReport, FluencyState,
    FrontMember, MarkingOutcome, MergeReport, ObservationKind, ObservationPlan, ObserveOutcome,
    QueryOutcome, QueryTuning, QuorumReport, RenormalizationReport, Transition,
};
pub use trace::{MemoryTrace, TraceTier, cosine_similarity, softmax};
