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

pub fn intersection<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| (p_i - q_i).abs())
        .sum::<f64>()
        / 2.0
}

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
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| p_i * q_i)
        .sum()
}

pub fn harmonic_mean<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    2.0 * p
        .iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| (p_i * q_i) / (p_i + q_i))
        .sum::<f64>()
}

pub fn kumar_hassebrook<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    let pq = p
        .iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| p_i * q_i)
        .sum::<f64>();

    pq / (p
        .iter()
        .map(|&p| p.into())
        .map(|p_i| p_i * p_i)
        .sum::<f64>()
        + q.iter()
            .map(|&q| q.into())
            .map(|q_i| q_i * q_i)
            .sum::<f64>()
        - pq)
}

pub fn jaccard<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    1.0 - kumar_hassebrook(p, q)
}

pub fn dice<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| (p_i - q_i) * (p_i - q_i))
        .sum::<f64>()
        / (p.iter()
            .map(|&p| p.into())
            .map(|p_i| p_i * p_i)
            .sum::<f64>()
            + q.iter()
                .map(|&q| q.into())
                .map(|q_i| q_i * q_i)
                .sum::<f64>())
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

#[cfg(test)]
mod tests {
    use crate::distance;

    const P: [f64; 3] = [0.000, 1.700, 2.350];
    const Q: [f64; 3] = [0.300, 1.700, 1.001];

    #[test]
    fn euclidean() {
        let result = distance::euclidean(&P, &Q);
        assert_relative_eq!(result, 1.381955499, epsilon = 1e-9);
    }

    #[test]
    fn manhattan() {
        let result = distance::manhattan(&P, &Q);
        assert_relative_eq!(result, 1.649000000, epsilon = 1e-9);
    }

    #[test]
    fn minkowski() {
        let result = distance::minkowski(&P, &Q, 2.0);
        assert_relative_eq!(result, 1.381955499, epsilon = 1e-9);
    }

    #[test]
    fn chebyshev() {
        let result = distance::chebyshev(&P, &Q);
        assert_relative_eq!(result, 1.349, epsilon = 1e-9);
    }

    #[test]
    fn sorensen() {
        let result = distance::sorensen(&P, &Q);
        assert_relative_eq!(result, 0.233867536, epsilon = 1e-9);
    }

    #[test]
    fn gower() {
        let result = distance::gower(&P, &Q);
        assert_relative_eq!(result, 0.549666667, epsilon = 1e-9);
    }

    #[test]
    fn soergel() {
        let result = distance::soergel(&P, &Q);
        assert_relative_eq!(result, 0.3790804598, epsilon = 1e-9);
    }

    #[test]
    fn kulczynski() {
        let result = distance::kulczynski(&P, &Q);
        assert_relative_eq!(result, 0.610514624, epsilon = 1e-9);
    }

    #[test]
    fn canberra() {
        let result = distance::canberra(&P, &Q);
        assert_relative_eq!(result, 1.402566398, epsilon = 1e-9);
    }

    #[test]
    fn lorentzian() {
        let result = distance::lorentzian(&P, &Q);
        assert_relative_eq!(result, 1.11635397, epsilon = 1e-9);
    }

    #[test]
    fn intersection() {
        let result = distance::intersection(&P, &Q);
        assert_relative_eq!(result, 0.8245, epsilon = 1e-9);
    }

    #[test]
    fn wave_hedges() {
        let result = distance::wave_hedges(&P, &Q);
        assert_relative_eq!(result, 1.574042553, epsilon = 1e-9);
    }

    #[test]
    fn czekanowski() {
        let result = distance::czekanowski(&P, &Q);
        assert_relative_eq!(result, 0.2338675365, epsilon = 1e-9);
    }

    #[test]
    fn motyka() {
        let result = distance::motyka(&P, &Q);
        assert_relative_eq!(result, 0.6169337683, epsilon = 1e-9);
    }

    #[test]
    fn ruzicka() {
        let result = distance::ruzicka(&P, &Q);
        assert_relative_eq!(result, 0.620919540, epsilon = 1e-9);
    }

    #[test]
    fn inner_product() {
        let result = distance::inner_product(&P, &Q);
        assert_relative_eq!(result, 5.24235, epsilon = 1e-9);
    }

    #[test]
    fn harmonic_mean() {
        let result = distance::harmonic_mean(&P, &Q);
        assert_relative_eq!(result, 3.1039689645, epsilon = 1e-9);
    }

    #[test]
    fn kumar_hassebrook() {
        let result = distance::kumar_hassebrook(&P, &Q);
        assert_relative_eq!(result, 0.732975296, epsilon = 1e-9);
    }

    #[test]
    fn jaccard() {
        let result = distance::jaccard(&P, &Q);
        assert_relative_eq!(result, 0.267024704, epsilon = 1e-9);
    }

    #[test]
    fn dice() {
        let result = distance::dice(&P, &Q);
        assert_relative_eq!(result, 0.154084541, epsilon = 1e-9);
    }
}
