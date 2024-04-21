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

pub fn tanimoto<T: Into<f64> + Copy>(_p: &[T], _q: &[T]) -> f64 {
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

pub fn jaccard<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    let pq = p
        .iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| p_i * q_i)
        .sum::<f64>();

    1.0 - (pq
        / (p.iter()
            .map(|&p| p.into())
            .map(|p_i| p_i * p_i)
            .sum::<f64>()
            + q.iter()
                .map(|&q| q.into())
                .map(|q_i| q_i * q_i)
                .sum::<f64>()
            - pq))
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

pub fn bhattacharyya<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    -(p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| (p_i * q_i).sqrt())
        .sum::<f64>())
    .ln()
}

pub fn hellinger<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    (2.0 * p
        .iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| (p_i.sqrt() - q_i.sqrt()).powi(2))
        .sum::<f64>())
    .sqrt()
}

pub fn matusita<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| (p_i.sqrt() - q_i.sqrt()).powi(2))
        .sum::<f64>()
        .sqrt()
}

pub fn squared_chord<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| (p_i.sqrt() - q_i.sqrt()).powi(2))
        .sum::<f64>()
}

pub fn squared_euclidean<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| (p_i - q_i).powi(2))
        .sum::<f64>()
}

pub fn pearson<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| ((p_i - q_i).powi(2) / q_i))
        .sum::<f64>()
}

pub fn neyman<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| ((p_i - q_i).powi(2) / p_i))
        .sum::<f64>()
}

pub fn squared<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| ((p_i - q_i).powi(2) / (p_i + q_i)))
        .sum::<f64>()
}

pub fn probabilistic_symmetric<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    2.0 * squared(p, q)
}

pub fn divergence<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    2.0 * p
        .iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| ((p_i - q_i).powi(2) / (p_i + q_i).powi(2)))
        .sum::<f64>()
}

pub fn clark<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| ((p_i - q_i).abs() / (p_i + q_i)).powi(2))
        .sum::<f64>()
        .sqrt()
}

pub fn additive_symmetric<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| ((p_i - q_i).powi(2) * (p_i + q_i)) / (p_i * q_i))
        .sum::<f64>()
}

pub fn kullback_leibler<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| p_i * (p_i / q_i).ln())
        .sum::<f64>()
}

pub fn jeffreys<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| (p_i - q_i) * (p_i / q_i).ln())
        .sum::<f64>()
}

pub fn k_divergence<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| p_i * ((2.0 * p_i) / (p_i + q_i)).ln())
        .sum::<f64>()
}

pub fn topsoe<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| {
            (p_i * ((2.0 * p_i) / (p_i + q_i)).ln()) + (q_i * ((2.0 * q_i) / (p_i + q_i)).ln())
        })
        .sum::<f64>()
}

pub fn jensen_shannon<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    let s1 = p
        .iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| p_i * ((2.0 * p_i) / (p_i + q_i)).ln())
        .sum::<f64>();

    let s2 = p
        .iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| q_i * ((2.0 * q_i) / (p_i + q_i)).ln())
        .sum::<f64>();

    (s1 + s2) / 2.0
}

pub fn jensen_difference<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| {
            ((p_i * p_i.ln() + q_i * q_i.ln()) / 2.0)
                - ((p_i + q_i) / 2.0) * ((p_i + q_i) / 2.0).ln()
        })
        .sum::<f64>()
}

pub fn taneja<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| ((p_i + q_i) / 2.0) * ((p_i + q_i) / (2.0 * (p_i * q_i).sqrt())).ln())
        .sum::<f64>()
}

pub fn kumar_johnson<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| ((p_i * p_i - q_i * q_i).powi(2)) / (2.0 * (p_i * q_i).powf(3.0 / 2.0)))
        .sum::<f64>()
}

pub fn avg<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    let sum = p
        .iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| (p_i - q_i).abs())
        .sum::<f64>();

    let max = p
        .iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| (p_i - q_i).abs())
        .fold(f64::NEG_INFINITY, f64::max);

    (sum + max) / 2.0
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
    fn jaccard() {
        let result = distance::jaccard(&P, &Q);
        assert_relative_eq!(result, 0.267024704, epsilon = 1e-9);
    }

    #[test]
    fn dice() {
        let result = distance::dice(&P, &Q);
        assert_relative_eq!(result, 0.154084541, epsilon = 1e-9);
    }

    #[test]
    fn bhattacharyya() {
        let result = distance::bhattacharyya(&P, &Q);
        assert_relative_eq!(result, -1.173638517, epsilon = 1e-9);
    }

    #[test]
    fn hellinger() {
        let result = distance::hellinger(&P, &Q);
        assert_relative_eq!(result, 1.080301318, epsilon = 1e-9);
    }

    #[test]
    fn matusita() {
        let result = distance::matusita(&P, &Q);
        assert_relative_eq!(result, 0.7638883876, epsilon = 1e-9);
    }

    #[test]
    fn squared_chord() {
        let result = distance::squared_chord(&P, &Q);
        assert_relative_eq!(result, 0.5835254687, epsilon = 1e-9);
    }

    #[test]
    fn squared_euclidean() {
        let result = distance::squared_euclidean(&P, &Q);
        assert_relative_eq!(result, 1.909801, epsilon = 1e-9);
    }

    #[test]
    fn pearson() {
        let result = distance::pearson(&P, &Q);
        assert_relative_eq!(result, 2.117983017, epsilon = 1e-9);
    }

    #[test]
    fn neyman() {
        // invert P and Q to avoid division by zero
        let result = distance::neyman(&Q, &P);
        assert_relative_eq!(result, 2.117983017, epsilon = 1e-9);
    }

    #[test]
    fn squared() {
        let result = distance::squared(&P, &Q);
        assert_relative_eq!(result, 0.843062071, epsilon = 1e-9);
    }

    #[test]
    fn probabilistic_symmetric() {
        let result = distance::probabilistic_symmetric(&P, &Q);
        assert_relative_eq!(result, 1.686124142, epsilon = 1e-9);
    }

    #[test]
    fn divergence() {
        let result = distance::divergence(&P, &Q);
        assert_relative_eq!(result, 2.32411941, epsilon = 1e-9);
    }
}
