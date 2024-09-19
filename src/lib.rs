//! An auto-scaling approximate histogram, following the opentelemetry algorithm.
//!
//! Quick and convenient, but there are quicker histogram implementations available.
//! The auto-scaling nature of the exponential histogram in this crate offers you
//! precision that is relative to the spread of the data observed in each observation
//! window.
//!
//!

mod exponential_histogram;

pub use exponential_histogram::ExponentialHistogram;
