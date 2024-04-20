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

#[cfg(test)]
mod tests {
    use crate as distance;

    const P: [f64; 3] = [0.000, 1.700, 2.350];
    const Q: [f64; 3] = [0.300, 1.700, 1.001];

    const P2: [u32; 3] = [1, 2, 3];
    const Q2: [u32; 3] = [2, 0, 1];

    #[test]
    fn cosine_f64() {
        let result = distance::cosine(&P, &Q);
        assert_relative_eq!(result, 0.905759279, epsilon = 1e-9);
    }

    #[test]
    fn cosine_u32() {
        let result = distance::cosine(&P2, &Q2);
        assert_relative_eq!(result, 0.5976143047, epsilon = 1e-9);
    }
}
