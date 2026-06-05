#![forbid(unsafe_code)]

/// Panning position: -1.0 = full left, 0.0 = center, 1.0 = full right
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pan(pub f64);

impl Pan {
    pub fn left() -> Self {
        Self(-1.0)
    }
    pub fn center() -> Self {
        Self(0.0)
    }
    pub fn right() -> Self {
        Self(1.0)
    }
    pub fn from_ternary(t: i8) -> Self {
        // map -1, 0, +1 to -1.0, 0.0, 1.0
        Pan(t as f64)
    }
    pub fn to_stereo_gains(&self) -> (f64, f64) {
        // linear pan law
        let l = (1.0 - self.0) * 0.5;
        let r = (1.0 + self.0) * 0.5;
        (l.max(0.0).min(1.0), r.max(0.0).min(1.0))
    }
}

/// Map ternary signal values to pan positions
pub fn pan_position(signal: &[i8]) -> Vec<Pan> {
    signal.iter().map(|&t| Pan::from_ternary(t)).collect()
}

/// Auto-pan: sweep across stereo field based on rhythm (ternary pattern)
/// position advances by each ternary value mapped to a step
pub fn auto_pan(rhythm: &[i8], sweep_width: f64) -> Vec<Pan> {
    let mut pos = 0.0f64;
    rhythm
        .iter()
        .map(|&t| {
            let step = t as f64 * sweep_width / rhythm.len().max(1) as f64;
            pos = (pos + step).clamp(-1.0, 1.0);
            Pan(pos)
        })
        .collect()
}

/// Distribute N agents evenly in the stereo field
pub fn surround(agent_count: usize) -> Vec<Pan> {
    if agent_count == 0 {
        return Vec::new();
    }
    if agent_count == 1 {
        return vec![Pan::center()];
    }
    (0..agent_count)
        .map(|i| {
            let t = i as f64 / (agent_count - 1) as f64;
            Pan(t * 2.0 - 1.0)
        })
        .collect()
}

/// Equal power pan law: gain_l = cos(θ), gain_r = sin(θ)
pub fn pan_law_equal_power(pan: Pan) -> (f64, f64) {
    let theta = (pan.0 * std::f64::consts::FRAC_PI_4 + std::f64::consts::FRAC_PI_4)
        .clamp(0.0, std::f64::consts::FRAC_PI_2);
    (theta.cos(), theta.sin())
}

/// Linear pan law: simple linear crossfade
pub fn pan_law_linear(pan: Pan) -> (f64, f64) {
    pan.to_stereo_gains()
}

/// Constant power pan law: sqrt-based for constant perceived loudness
pub fn pan_law_constant_power(pan: Pan) -> (f64, f64) {
    let t = (pan.0 + 1.0) * 0.5;
    let l = (1.0 - t).sqrt();
    let r = t.sqrt();
    (l, r)
}

/// Apply pan to a ternary signal, returning interleaved stereo (L, R, L, R, ...)
pub fn apply_pan(signal: &[i8], pan: Pan) -> Vec<i8> {
    let (gl, gr) = pan_law_linear(pan);
    let mut out = Vec::with_capacity(signal.len() * 2);
    for &s in signal {
        out.push((s as f64 * gl).round().clamp(-128.0, 127.0) as i8);
        out.push((s as f64 * gr).round().clamp(-128.0, 127.0) as i8);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pan_from_ternary() {
        assert_eq!(Pan::from_ternary(-1), Pan::left());
        assert_eq!(Pan::from_ternary(0), Pan::center());
        assert_eq!(Pan::from_ternary(1), Pan::right());
    }

    #[test]
    fn test_pan_position() {
        let sig = &[-1i8, 0, 1];
        let pans = pan_position(sig);
        assert_eq!(pans.len(), 3);
        assert_eq!(pans[0], Pan::left());
        assert_eq!(pans[1], Pan::center());
        assert_eq!(pans[2], Pan::right());
    }

    #[test]
    fn test_auto_pan_basic() {
        let rhythm = &[1i8, 1, 1];
        let pans = auto_pan(rhythm, 1.0);
        assert_eq!(pans.len(), 3);
        // pan should move rightward
        assert!(pans[1].0 > pans[0].0);
    }

    #[test]
    fn test_auto_pan_clamps() {
        let rhythm = &[1i8; 100];
        let pans = auto_pan(rhythm, 1.0);
        assert!(pans.last().unwrap().0 <= 1.0);
    }

    #[test]
    fn test_surround_zero() {
        assert!(surround(0).is_empty());
    }

    #[test]
    fn test_surround_one() {
        let s = surround(1);
        assert_eq!(s, vec![Pan::center()]);
    }

    #[test]
    fn test_surround_even() {
        let s = surround(4);
        assert_eq!(s.len(), 4);
        assert_eq!(s[0], Pan::left());
        assert_eq!(s[3], Pan::right());
    }

    #[test]
    fn test_surround_two() {
        let s = surround(2);
        assert_eq!(s[0], Pan::left());
        assert_eq!(s[1], Pan::right());
    }

    #[test]
    fn test_pan_law_equal_power_center() {
        let (l, r) = pan_law_equal_power(Pan::center());
        assert!((l - r).abs() < 0.01);
    }

    #[test]
    fn test_pan_law_equal_power_left() {
        let (l, _r) = pan_law_equal_power(Pan::left());
        assert!(l > 0.99);
    }

    #[test]
    fn test_pan_law_linear_center() {
        let (l, r) = pan_law_linear(Pan::center());
        assert!((l - 0.5).abs() < 0.001);
        assert!((r - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_pan_law_constant_power_center() {
        let (l, r) = pan_law_constant_power(Pan::center());
        assert!((l - r).abs() < 0.01);
    }

    #[test]
    fn test_pan_law_constant_power_sum_squares() {
        // constant power: l^2 + r^2 should be ~1
        let (l, r) = pan_law_constant_power(Pan(0.3));
        assert!((l * l + r * r - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_apply_pan() {
        let sig = &[100i8];
        let stereo = apply_pan(sig, Pan::center());
        assert_eq!(stereo.len(), 2);
    }

    #[test]
    fn test_apply_pan_full_left() {
        let sig = &[50i8];
        let stereo = apply_pan(sig, Pan::left());
        assert_eq!(stereo[0], 50); // left channel full
        assert_eq!(stereo[1], 0);  // right channel zero
    }
}
