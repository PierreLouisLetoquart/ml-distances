pub fn euclidean<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| (p_i - q_i).powf(2.0))
        .sum::<f64>()
        .sqrt()
}

pub fn manhattan<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| (p_i - q_i).abs())
        .sum()
}

pub fn minkowski<T: Into<f64> + Copy>(p: &[T], q: &[T], d: f64) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| (p_i - q_i).abs().powf(d))
        .sum::<f64>()
        .powf(1.0 / d)
}

pub fn chebyshev<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| (p_i - q_i).abs())
        .fold(0.0, f64::max)
}

pub fn sorensen<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| (p_i - q_i).abs())
        .sum::<f64>()
        / p.iter()
            .map(|&p| p.into())
            .zip(q.iter().map(|&q| q.into()))
            .map(|(p_i, q_i)| p_i + q_i)
            .sum::<f64>()
}

pub fn gower<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| (p_i - q_i).abs())
        .sum::<f64>()
        / p.len() as f64
}

pub fn soergel<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| (p_i - q_i).abs())
        .sum::<f64>()
        / p.iter()
            .map(|&p| p.into())
            .zip(q.iter().map(|&q| q.into()))
            .map(|(p_i, q_i)| p_i.max(q_i))
            .sum::<f64>()
}

pub fn kulczynski<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| (p_i - q_i).abs())
        .sum::<f64>()
        / p.iter()
            .map(|&p| p.into())
            .zip(q.iter().map(|&q| q.into()))
            .map(|(p_i, q_i)| p_i.min(q_i))
            .sum::<f64>()
}

pub fn canberra<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| (p_i - q_i).abs() / (p_i.abs() + q_i.abs()))
        .sum()
}

pub fn lorentzian<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| (p_i - q_i).abs().ln_1p())
        .sum()
}

// d(p,q)=\frac{1}{2}\sum\limits_{i=1}^{n}{\left|p_i - q_i\right|}
pub fn intersection<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| (p_i - q_i).abs())
        .sum::<f64>()
        / 2.0
}
// d(p,q)=\sum\limits_{i=1}^{n}{\frac{\left|p_i - q_i\right|}{max(p_i,q_i)}}
pub fn wave_hedges<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| (p_i - q_i).abs() / p_i.max(q_i))
        .sum()
}

pub fn czekanowski<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    1.0 - 2.0
        * p.iter()
            .map(|&p| p.into())
            .zip(q.iter().map(|&q| q.into()))
            .map(|(p_i, q_i)| p_i.min(q_i))
            .sum::<f64>()
        / p.iter()
            .map(|&p| p.into())
            .zip(q.iter().map(|&q| q.into()))
            .map(|(p_i, q_i)| p_i + q_i)
            .sum::<f64>()
}

pub fn motyka<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| p_i.max(q_i))
        .sum::<f64>()
        / p.iter()
            .map(|&p| p.into())
            .zip(q.iter().map(|&q| q.into()))
            .map(|(p_i, q_i)| p_i + q_i)
            .sum::<f64>()
}

pub fn ruzicka<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| p_i.min(q_i))
        .sum::<f64>()
        / p.iter()
            .map(|&p| p.into())
            .zip(q.iter().map(|&q| q.into()))
            .map(|(p_i, q_i)| p_i.max(q_i))
            .sum::<f64>()
}

pub fn tanimoto<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn inner_product<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn harmonic_mean<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn cosine<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn kumar_hassebrook<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn jaccard<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn dice<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn fidelity<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn bhattacharyya<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn hellinger<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn matusita<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn squared_chord<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn squared_euclidean<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn pearson<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn neyman<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn squared<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn probabilistic_symmetric<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn divergence<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn clark<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn additive_symmetric<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn kullback_leibler<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn jeffreys<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn k_divergence<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn topsoe<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn jensen_shannon<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn jensen_difference<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn taneja<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn kumar_johnson<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

pub fn avg<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    unimplemented!()
}

// Used to test the results of the functions with a specific precision
#[allow(unused_imports)] // to fix the wtf warning
#[macro_use]
extern crate approx;

#[cfg(test)]
mod tests {
    use crate as distance;

    const P: [f64; 3] = [0.000, 1.700, 2.350];
    const Q: [f64; 3] = [0.300, 1.700, 1.001];

    const P2: [u32; 3] = [1, 2, 3];
    const Q2: [u32; 3] = [2, 0, 1];

    #[test]
    fn euclidean_f64() {
        let result = distance::euclidean(&P, &Q);
        assert_relative_eq!(result, 1.381955499, epsilon = 1e-9);
    }

    #[test]
    fn euclidean_u32() {
        let result = distance::euclidean(&P2, &Q2);
        assert_relative_eq!(result, 3.0, epsilon = 1e-9);
    }

    #[test]
    fn manhattan_f64() {
        let result = distance::manhattan(&P, &Q);
        assert_relative_eq!(result, 1.649000000, epsilon = 1e-9);
    }

    #[test]
    fn manhattan_u32() {
        let result = distance::manhattan(&P2, &Q2);
        assert_relative_eq!(result, 5.0, epsilon = 1e-9);
    }

    #[test]
    fn minkowski_f64() {
        let result = distance::minkowski(&P, &Q, 2.0);
        assert_relative_eq!(result, 1.381955499, epsilon = 1e-9);
    }

    #[test]
    fn minkowski_u32() {
        let result = distance::minkowski(&P2, &Q2, 2.0);
        assert_relative_eq!(result, 3.0, epsilon = 1e-9);
    }

    #[test]
    fn chebyshev_f64() {
        let result = distance::chebyshev(&P, &Q);
        assert_relative_eq!(result, 1.349, epsilon = 1e-9);
    }

    #[test]
    fn chebyshev_u32() {
        let result = distance::chebyshev(&P2, &Q2);
        assert_relative_eq!(result, 2.0, epsilon = 1e-9);
    }

    #[test]
    fn sorensen_f64() {
        let result = distance::sorensen(&P, &Q);
        assert_relative_eq!(result, 0.233867536, epsilon = 1e-9);
    }

    #[test]
    fn sorensen_u32() {
        let result = distance::sorensen(&P2, &Q2);
        assert_relative_eq!(result, 0.555555556, epsilon = 1e-9);
    }

    #[test]
    fn gower_f64() {
        let result = distance::gower(&P, &Q);
        assert_relative_eq!(result, 0.549666667, epsilon = 1e-9);
    }

    #[test]
    fn gower_u32() {
        let result = distance::gower(&P2, &Q2);
        assert_relative_eq!(result, 1.666666667, epsilon = 1e-9);
    }

    #[test]
    fn soergel_f64() {
        let result = distance::soergel(&P, &Q);
        assert_relative_eq!(result, 0.3790804598, epsilon = 1e-9);
    }

    #[test]
    fn soergel_u32() {
        let result = distance::soergel(&P2, &Q2);
        assert_relative_eq!(result, 0.714285714, epsilon = 1e-9);
    }

    #[test]
    fn kulczynski_f64() {
        let result = distance::kulczynski(&P, &Q);
        assert_relative_eq!(result, 0.610514624, epsilon = 1e-9);
    }

    #[test]
    fn kulczynski_u32() {
        let result = distance::kulczynski(&P2, &Q2);
        assert_relative_eq!(result, 2.5, epsilon = 1e-9);
    }

    #[test]
    fn canberra_f64() {
        let result = distance::canberra(&P, &Q);
        assert_relative_eq!(result, 1.402566398, epsilon = 1e-9);
    }

    #[test]
    fn canberra_u32() {
        let result = distance::canberra(&P2, &Q2);
        assert_relative_eq!(result, 1.833333333, epsilon = 1e-9);
    }

    #[test]
    fn lorentzian_f64() {
        let result = distance::lorentzian(&P, &Q);
        assert_relative_eq!(result, 1.11635397, epsilon = 1e-9);
    }

    #[test]
    fn lorentzian_u32() {
        let result = distance::lorentzian(&P2, &Q2);
        assert_relative_eq!(result, 2.890371758, epsilon = 1e-9);
    }

    #[test]
    fn intersection_f64() {
        let result = distance::intersection(&P, &Q);
        assert_relative_eq!(result, 0.8245, epsilon = 1e-9);
    }

    #[test]
    fn intersection_u32() {
        let result = distance::intersection(&P2, &Q2);
        assert_relative_eq!(result, 2.5, epsilon = 1e-9);
    }

    #[test]
    fn wave_hedges_f64() {
        let result = distance::wave_hedges(&P, &Q);
        assert_relative_eq!(result, 1.574042553, epsilon = 1e-9);
    }

    #[test]
    fn wave_hedges_u32() {
        let result = distance::wave_hedges(&P2, &Q2);
        assert_relative_eq!(result, 2.166666667, epsilon = 1e-9);
    }

    #[test]
    fn czekanowski_f64() {
        let result = distance::czekanowski(&P, &Q);
        assert_relative_eq!(result, 0.2338675365, epsilon = 1e-9);
    }

    #[test]
    fn czekanowski_u32() {
        let result = distance::czekanowski(&P2, &Q2);
        assert_relative_eq!(result, 0.555555556, epsilon = 1e-9);
    }

    #[test]
    fn motyka_f64() {
        let result = distance::motyka(&P, &Q);
        assert_relative_eq!(result, 0.6169337683, epsilon = 1e-9);
    }

    #[test]
    fn motyka_u32() {
        let result = distance::motyka(&P2, &Q2);
        assert_relative_eq!(result, 0.777777778, epsilon = 1e-9);
    }

    #[test]
    fn ruzicka_f64() {
        let result = distance::ruzicka(&P, &Q);
        assert_relative_eq!(result, 0.620919540, epsilon = 1e-9);
    }

    #[test]
    fn ruzicka_u32() {
        let result = distance::ruzicka(&P2, &Q2);
        assert_relative_eq!(result, 0.285714286, epsilon = 1e-9);
    }
}
