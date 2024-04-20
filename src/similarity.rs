pub fn cosine<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    let numerator = p
        .iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| p_i * q_i)
        .sum::<f64>();
    let denominator = p
        .iter()
        .map(|&p| p.into())
        .map(|p_i| p_i * p_i)
        .sum::<f64>()
        .sqrt()
        * q.iter()
            .map(|&q| q.into())
            .map(|q_i| q_i * q_i)
            .sum::<f64>()
            .sqrt();
    numerator / denominator
}

pub fn kulczynski<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    1.0 / crate::distance::kulczynski(p, q)
}

pub fn dice<T: Into<f64> + Copy>(p: &[T], q: &[T]) -> f64 {
    (2.0 * p
        .iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| p_i * q_i)
        .sum::<f64>())
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
    p.iter()
        .map(|&p| p.into())
        .zip(q.iter().map(|&q| q.into()))
        .map(|(p_i, q_i)| (p_i * q_i).sqrt())
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::similarity;

    const P: [f64; 3] = [0.000, 1.700, 2.350];
    const Q: [f64; 3] = [0.300, 1.700, 1.001];

    #[test]
    fn cosine() {
        let result = similarity::cosine(&P, &Q);
        assert_relative_eq!(result, 0.905759279, epsilon = 1e-9);
    }

    #[test]
    fn kulczynski() {
        let result = similarity::kulczynski(&P, &Q);
        assert_relative_eq!(result, 1.637962402, epsilon = 1e-9);
    }

    #[test]
    fn fidelity() {
        let result = similarity::fidelity(&P, &Q);
        assert_relative_eq!(result, 3.233737266, epsilon = 1e-9);
    }
}
