//! Fibonacci Growth v2 — scale-invariant dynamics for the Grand Pattern.
//!
//! Key insight from Casey: "Fibonacci works both directions — Penrose inflation
//! outward, Mandelbrot roughness inward. CR is scale-invariant by construction."

use serde::{Deserialize, Serialize};

/// Golden ratio φ = (1 + √5) / 2
pub const PHI: f64 = 1.618033988749895;

/// Golden ratio conjugate ψ = (1 - √5) / 2
pub const PSI: f64 = -0.618033988749895;

/// Generate Fibonacci sequence up to n terms.
pub fn fibonacci_sequence(n: usize) -> Vec<u64> {
    let mut seq = Vec::with_capacity(n);
    if n > 0 { seq.push(0); }
    if n > 1 { seq.push(1); }
    for i in 2..n {
        seq.push(seq[i - 1] + seq[i - 2]);
    }
    seq
}

/// Binet's formula: F(n) = (φ^n - ψ^n) / √5
pub fn fibonacci_binet(n: u64) -> f64 {
    (PHI.powi(n as i32) - PSI.powi(n as i32)) / 5_f64.sqrt()
}

/// Penrose inflation: expand a sequence by Fibonacci substitution rules.
/// 0 → 01, 1 → 0 (Fibonacci word substitution)
pub fn penrose_inflate(sequence: &[u8], iterations: usize) -> Vec<u8> {
    let mut current = sequence.to_vec();
    for _ in 0..iterations {
        let mut next = Vec::new();
        for &bit in &current {
            match bit {
                0 => { next.push(0); next.push(1); }
                _ => { next.push(0); }
            }
        }
        current = next;
    }
    current
}

/// Mandelbrot roughness: measure how "rough" a sequence is by computing
/// the fractal dimension estimate via box-counting on the signal.
pub fn mandelbrot_roughness(values: &[f64]) -> f64 {
    if values.len() < 4 { return 0.0; }
    // Compute total variation as a proxy for roughness
    let variation: f64 = values.windows(2)
        .map(|w| (w[1] - w[0]).abs())
        .sum();
    // Normalize by range and log-scale
    let range = values.iter().cloned().fold(f64::INFINITY, f64::min)
        ..=values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let span = (*range.end() - *range.start()).max(f64::EPSILON);
    (variation / span).ln().abs()
}

/// Golden ratio scaling: scale a value by φ^n.
pub fn golden_scale(value: f64, n: i32) -> f64 {
    value * PHI.powi(n)
}

/// Golden spiral radius at angle θ: r = a * φ^(θ / 90°)
pub fn golden_spiral(a: f64, theta_deg: f64) -> (f64, f64) {
    let r = a * PHI.powf(theta_deg / 90.0);
    let theta_rad = theta_deg.to_radians();
    (r * theta_rad.cos(), r * theta_rad.sin())
}

/// Check if a number is a Fibonacci number: n is Fibonacci iff 5n²+4 or 5n²-4 is a perfect square.
pub fn is_fibonacci(n: u64) -> bool {
    fn is_perfect_square(x: u64) -> bool {
        let s = (x as f64).sqrt() as u64;
        s * s == x || (s + 1) * (s + 1) == x
    }
    is_perfect_square(5 * n * n + 4) || is_perfect_square(5 * n * n - 4)
}

/// Zeckendorf representation: decompose a number into non-adjacent Fibonacci numbers.
pub fn zeckendorf(mut n: u64) -> Vec<u64> {
    if n == 0 { return vec![0]; }
    let mut fibs = fibonacci_sequence(93); // F(93) < 2^64
    fibs.reverse();
    fibs.retain(|&f| f > 0);

    let mut result = Vec::new();
    for &f in &fibs {
        if f <= n {
            result.push(f);
            n -= f;
        }
    }
    result
}

/// Growth model: simulate a system that grows according to Fibonacci dynamics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FibonacciGrowth {
    pub values: Vec<f64>,
    pub growth_rates: Vec<f64>,
    pub golden_ratio_deviation: Vec<f64>,
}

impl FibonacciGrowth {
    /// Simulate Fibonacci growth from initial values.
    pub fn simulate(a0: f64, a1: f64, steps: usize) -> Self {
        let mut values = vec![a0, a1];
        let mut growth_rates = vec![0.0, a1 / a0.max(f64::EPSILON)];

        for i in 2..=steps {
            let next = values[i - 1] + values[i - 2];
            values.push(next);
            let rate = next / values[i - 1].max(f64::EPSILON);
            growth_rates.push(rate);
        }

        let golden_ratio_deviation: Vec<f64> = growth_rates.iter()
            .map(|&r| (r - PHI).abs())
            .collect();

        Self { values, growth_rates, golden_ratio_deviation }
    }

    /// Does the system converge to golden ratio growth?
    pub fn converges_to_golden(&self) -> bool {
        if self.golden_ratio_deviation.len() < 5 { return false; }
        let recent = &self.golden_ratio_deviation[self.golden_ratio_deviation.len() - 5..];
        recent.iter().all(|&d| d < 0.01)
    }

    /// Lyapunov-like measure: how sensitive is growth to initial conditions?
    pub fn sensitivity(&self) -> f64 {
        if self.growth_rates.len() < 2 { return 0.0; }
        self.growth_rates.windows(2)
            .map(|w| (w[1] - w[0]).abs().ln())
            .filter(|v| v.is_finite())
            .sum::<f64>() / (self.growth_rates.len() - 1).max(1) as f64
    }
}

/// Ratio convergence: track how F(n+1)/F(n) converges to φ.
pub fn ratio_convergence(n_terms: usize) -> Vec<f64> {
    let fibs = fibonacci_sequence(n_terms);
    fibs.windows(2)
        .skip(1) // skip first (0/1 = 0)
        .map(|w| w[1] as f64 / w[0].max(1) as f64)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_sequence() {
        let seq = fibonacci_sequence(10);
        assert_eq!(seq, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn test_binet_formula() {
        for n in 1..20u64 {
            let expected = fibonacci_sequence(30)[n as usize] as f64;
            let binet = fibonacci_binet(n);
            assert!((binet - expected).abs() < 0.5, "F({n}): binet={binet}, expected={expected}");
        }
    }

    #[test]
    fn test_penrose_inflate() {
        let start = vec![0];
        let inflated = penrose_inflate(&start, 3);
        // 0 → 01 → 010 → 01001
        assert_eq!(inflated, vec![0, 1, 0, 0, 1]);
    }

    #[test]
    fn test_penrose_growth_rate() {
        let start = vec![0];
        let lengths: Vec<usize> = (0..8).map(|i| penrose_inflate(&start, i).len()).collect();
        // Should grow approximately as F(n+2)
        assert!(lengths[5] > lengths[4]);
    }

    #[test]
    fn test_mandelbrot_roughness_smooth() {
        let smooth: Vec<f64> = (0..100).map(|i| i as f64 * 0.1).collect();
        let r = mandelbrot_roughness(&smooth);
        assert!(r >= 0.0);
    }

    #[test]
    fn test_mandelbrot_roughness_noisy() {
        let noisy: Vec<f64> = (0..100).map(|i| (i as f64 * 0.1).sin() * 100.0).collect();
        let r_noisy = mandelbrot_roughness(&noisy);
        let smooth: Vec<f64> = (0..100).map(|i| i as f64 * 0.1).collect();
        let r_smooth = mandelbrot_roughness(&smooth);
        // Noisy signal should be rougher
        assert!(r_noisy > r_smooth);
    }

    #[test]
    fn test_golden_scale() {
        let v = golden_scale(1.0, 1);
        assert!((v - PHI).abs() < 1e-10);
        let v2 = golden_scale(1.0, -1);
        assert!((v2 - 1.0 / PHI).abs() < 1e-10);
    }

    #[test]
    fn test_golden_spiral() {
        let (x, y) = golden_spiral(1.0, 0.0);
        // r = a * φ^(0/90) = 1.0 * φ^0 = 1.0
        // x = r * cos(0) = 1.0, y = r * sin(0) ≈ 0
        assert!((x - 1.0).abs() < 1e-10);
        assert!(y.abs() < 1e-10);
    }

    #[test]
    fn test_is_fibonacci() {
        assert!(is_fibonacci(0));
        assert!(is_fibonacci(1));
        assert!(is_fibonacci(5));
        assert!(is_fibonacci(8));
        assert!(is_fibonacci(13));
        assert!(!is_fibonacci(4));
        assert!(!is_fibonacci(6));
        assert!(!is_fibonacci(7));
    }

    #[test]
    fn test_zeckendorf() {
        let z = zeckendorf(100);
        assert_eq!(z.iter().sum::<u64>(), 100);
        // No two adjacent Fibonacci numbers
        let fibs = fibonacci_sequence(93);
        for w in z.windows(2) {
            let idx0 = fibs.iter().position(|&f| f == w[0]).unwrap();
            let idx1 = fibs.iter().position(|&f| f == w[1]).unwrap();
            assert!((idx0 as i64 - idx1 as i64).abs() >= 2, "Adjacent: {} and {}", w[0], w[1]);
        }
    }

    #[test]
    fn test_zeckendorf_zero() {
        assert_eq!(zeckendorf(0), vec![0]);
    }

    #[test]
    fn test_fibonacci_growth_simulate() {
        let growth = FibonacciGrowth::simulate(1.0, 1.0, 20);
        // values starts with [a0, a1] then appends `steps` more
        assert_eq!(growth.values.len(), 21); // 2 initial + 19 steps (2..=20)
        assert_eq!(growth.values[2], 2.0);
        assert_eq!(growth.values[3], 3.0);
    }

    #[test]
    fn test_converges_to_golden() {
        let growth = FibonacciGrowth::simulate(1.0, 1.0, 30);
        assert!(growth.converges_to_golden());
    }

    #[test]
    fn test_sensitivity() {
        let growth = FibonacciGrowth::simulate(1.0, 1.0, 50);
        let s = growth.sensitivity();
        assert!(s.is_finite());
    }

    #[test]
    fn test_ratio_convergence() {
        let ratios = ratio_convergence(20);
        assert!(!ratios.is_empty());
        // Should converge to φ
        let last = ratios.last().unwrap();
        assert!((last - PHI).abs() < 0.01, "Last ratio: {last}, φ: {PHI}");
    }

    #[test]
    fn test_phi_psi_identity() {
        // φ * ψ = -1
        assert!((PHI * PSI - (-1.0)).abs() < 1e-10);
        // φ + ψ = 1
        assert!((PHI + PSI - 1.0).abs() < 1e-10);
        // φ - 1 = 1/φ
        assert!((PHI - 1.0 - 1.0 / PHI).abs() < 1e-10);
    }

    #[test]
    fn test_penrose_penrose_match() {
        // After enough iterations, ratio of consecutive lengths → φ
        let start = vec![0];
        let prev = penrose_inflate(&start, 7).len() as f64;
        let curr = penrose_inflate(&start, 8).len() as f64;
        let ratio = curr / prev;
        assert!((ratio - PHI).abs() < 0.1, "Ratio: {ratio}");
    }

    #[test]
    fn test_zeckendorf_single_fib() {
        let z = zeckendorf(13);
        assert_eq!(z, vec![13]);
    }
}
