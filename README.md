# ML Distance

This Rust crate is based on the paper [Comprehensive Survey on Distance/Similarity Measures between Probability Density Functions](https://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf) and implemented to port the [ml-distance](https://www.npmjs.com/package/ml-distance) pkg from js/ts to rust.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
ml_distance = "0.1.0"
```

And place this in your code:

```rust
let a: [f64; 3] = [0.000, 1.700, 2.350];
let b: [f64; 3] = [0.300, 1.700, 1.001];

let distance = euclidean(&a, &b);
assert_eq!(distance, 1.3819554985599212);
```

> Can be used with any `T` that implement `Into<f64>`, (e.g. `let a = vec![1,4,2]` works etc.)

## Distances Implemented

| Name                   | Formula Link                                                                       | Status |
| ---------------------- | ---------------------------------------------------------------------------------- | ------ |
| euclidean              | [Link](http://en.wikipedia.org/wiki/Euclidean_distance#n_dimensions)               | ✅     |
| manhattan              | [Link](http://en.wikipedia.org/wiki/Taxicab_geometry)                              | ✅     |
| minkowski              | [Link](http://en.wikipedia.org/wiki/Minkowski_distance)                            | ✅     |
| chebyshev              | [Link](http://en.wikipedia.org/wiki/Chebyshev_distance)                            | ✅     |
| sorensen               | [Link](http://en.wikipedia.org/wiki/S%C3%B8rensen%E2%80%93Dice_coefficient)        | ✅     |
| gower                  | [Link](https://stat.ethz.ch/education/semesters/ss2012/ams/slides/v4.2.pdf)        | ✅     |
| soergel                | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | ✅     |
| kulczynski             | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | ✅     |
| canberra               | [Link](http://en.wikipedia.org/wiki/Canberra_distance)                             | ✅     |
| lorentzian             | [Link](https://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                         | ✅     |
| intersection           | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | ✅     |
| waveHedges             | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | ✅     |
| czekanowski            | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | ✅     |
| motyka                 | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | ✅     |
| ruzicka                | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | ✅     |
| tanimoto               | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | 🔜     |
| innerProduct           | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | ✅     |
| harmonicMean           | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | ✅     |
| cosine                 | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | ✅     |
| kumarHassebrook        | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | 🔜     |
| jaccard                | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | 🔜     |
| dice                   | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | 🔜     |
| fidelity               | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | 🔜     |
| bhattacharyya          | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | 🔜     |
| hellinger              | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | 🔜     |
| matusita               | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | 🔜     |
| squaredChord           | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | 🔜     |
| squaredEuclidean       | [Link](http://en.wikipedia.org/wiki/Euclidean_distance#Squared_Euclidean_distance) | 🔜     |
| pearson                | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | 🔜     |
| neyman                 | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | 🔜     |
| squared                | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | 🔜     |
| probabilisticSymmetric | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | 🔜     |
| divergence             | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | 🔜     |
| clark                  | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | 🔜     |
| additiveSymmetric      | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | 🔜     |
| kullbackLeibler        | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | 🔜     |
| jeffreys               | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | 🔜     |
| kdivergence            | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | 🔜     |
| topsoe                 | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | 🔜     |
| jensenShannon          | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | 🔜     |
| jensenDifference       | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | 🔜     |
| taneja                 | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | 🔜     |
| kumarJohnson           | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | 🔜     |
| avg                    | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf)                          | 🔜     |

## Similarities Implemented

| Name         | Formula Link                                              | Status |
| ------------ | --------------------------------------------------------- | ------ |
| intersection | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf) | 🔜     |
| czekanowski  | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf) | 🔜     |
| motyka       | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf) | 🔜     |
| kulczynski   | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf) | 🔜     |
| squaredChord | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf) | 🔜     |
| jaccard      | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf) | 🔜     |
| dice         | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf) | 🔜     |
| tanimoto     | [Link](http://www.naun.org/main/NAUN/ijmmas/mmmas-49.pdf) | 🔜     |
