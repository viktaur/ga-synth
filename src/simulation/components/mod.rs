pub(crate) mod filters;
pub(crate) mod envelope;
pub(crate) mod harmonics;
pub mod oscillator;

// pub trait Component {
//     type Params;
//     /// Creates a new component.
//     fn create() -> Self;
//
//     fn combine(&self, other: &Self, r: f32) -> Option<Self>
//         where
//             Self: Sized;
//
//     fn evolve(&self, step_size: f32) -> Self;
// }