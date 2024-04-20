// Used to test the results of the functions with a specific precision
#[allow(unused_imports)] // to fix the wtf warning
#[macro_use]
extern crate approx;

pub mod distance;
pub mod similarity;

pub use distance::additive_symmetric;
pub use distance::avg;
pub use distance::bhattacharyya;
pub use distance::canberra;
pub use distance::chebyshev;
pub use distance::clark;
pub use distance::czekanowski;
pub use distance::dice;
pub use distance::divergence;
pub use distance::euclidean;
pub use distance::fidelity;
pub use distance::gower;
pub use distance::harmonic_mean;
pub use distance::hellinger;
pub use distance::inner_product;
pub use distance::intersection;
pub use distance::jaccard;
pub use distance::jeffreys;
pub use distance::jensen_difference;
pub use distance::jensen_shannon;
pub use distance::k_divergence;
pub use distance::kulczynski;
pub use distance::kullback_leibler;
pub use distance::kumar_hassebrook;
pub use distance::kumar_johnson;
pub use distance::lorentzian;
pub use distance::manhattan;
pub use distance::matusita;
pub use distance::minkowski;
pub use distance::motyka;
pub use distance::neyman;
pub use distance::pearson;
pub use distance::ruzicka;
pub use distance::soergel;
pub use distance::sorensen;
pub use distance::squared;
pub use distance::squared_chord;
pub use distance::squared_euclidean;
pub use distance::taneja;
pub use distance::tanimoto;
pub use distance::topsoe;
pub use distance::wave_hedges;

pub use similarity::cosine;
