#![deny(rustdoc::missing_doc_code_examples)]
#![deny(missing_docs)]
//! # Genetic algorithms for solving TSPs.
//!
//! This crates contains utitlities to run genetic algorithms and solve Traveling Salesman Problems.

/// Represent a distance Matrix as a Vec<Vec<f64>>.
pub mod distance_mat;
/// Traits used for abstraction of gen-algo
pub mod gen_traits;
/// The `route`-module contains the `Route`-class, the individual element of the TSP that implements
/// important methods like `crossover` or `mutate`.
pub mod route;
/// The `routes`-module contains the main class of this crate which is the `Routes`-class that contains
/// your current subset of routes and with which you can evolve them.
pub mod routes;
/// The `subsequence`-module contains a helper function, `Subsequence` that gives you functionality to select elements
/// before, in and after a subsequence of a Vector. It is used extensively in the `ordered_crossover`-function.
mod subsequence;
/// the `test-utils`-module contains utitlities for testing and include for example the construction of test-data
/// or the comparison of specializied objects (like permutations).
mod test_utils;
/// The `utils`-module contains utility that are used throughout the rest of the code base. The underlying `ordered_crossover`-
/// function is implemented here.
mod utils;
