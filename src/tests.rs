//! Mod contains utils function for tests

/* mod declaration */
pub mod constant;
pub mod generator;
pub mod io;

#[cfg(feature = "parallel")]
/// Perform transmutation on box
pub fn transmute<I, O>(data: &[I]) -> &[O]
where
    I: std::marker::Sized,
    O: std::marker::Sized,
{
    unsafe { std::mem::transmute::<&[I], &[O]>(data) }
}
