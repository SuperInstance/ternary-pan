# ternary-pan

Stereo panning and spatial audio distribution for ternary {-1, 0, +1} signals. Maps ternary values to pan positions with linear, equal-power, and constant-power pan laws, plus auto-panning and multi-agent surround placement.

## Why It Matters

In ternary agent systems, the three states {-1, 0, +1} map naturally to spatial positions in a stereo field: left, center, right. This crate provides the signal-processing primitives to:

- **Spatialize agent populations**: distribute N agents across the stereo field
- **Pan signals with physical accuracy**: choose the pan law that matches your psychoacoustic model
- **Auto-pan with rhythmic control**: ternary patterns drive LFO-like panning sweeps
- **Generate interleaved stereo** from mono ternary signals

The choice of pan law is critical for perceived loudness consistency as sources move across the field.

## How It Works

### Pan Position

A pan value $p \in [-1, +1]$ where -1 = full left, 0 = center, +1 = full right. The ternary mapping is direct:

$$\{-1 \mapsto L,\; 0 \mapsto C,\; +1 \mapsto R\}$$

### Linear Pan Law

Simple linear crossfade:

$$g_L = \frac{1 - p}{2}, \quad g_R = \frac{1 + p}{2}$$

**Power ratio:** at center, each channel carries 50% power (-6 dB). Total power varies with pan position — sources sound quieter at center.

### Equal-Power Pan Law

Constant total power across the stereo field:

$$g_L = \cos\!\left(\frac{p+1}{2} \cdot \frac{\pi}{2}\right), \quad g_R = \sin\!\left(\frac{p+1}{2} \cdot \frac{\pi}{2}\right)$$

At center: $g_L = g_R = \cos(\pi/4) \approx 0.707$ (-3 dB per channel), total power = 1. **Preferred for most mixing applications.**

### Constant-Power Pan Law

Square-root crossfade ensuring $g_L^2 + g_R^2 = 1$ for all $p$:

$$g_L = \sqrt{1 - t}, \quad g_R = \sqrt{t}, \quad \text{where } t = \frac{p+1}{2}$$

Equivalent to equal-power but parameterized differently. At center: both gains ≈ 0.707.

**Verification:**

$$g_L^2 + g_R^2 = (1-t) + t = 1 \quad \forall\; t$$

### Auto-Pan

Given a rhythm pattern $\{r_i\} \in \{-1, 0, +1\}$, the pan position accumulates:

$$\text{pos}(i) = \text{clamp}\!\left(\text{pos}(i-1) + \frac{r_i \cdot w}{N},\;-1,\;+1\right)$$

where $w$ is the sweep width and $N$ is the pattern length.

### Surround Distribution

For $k$ agents, evenly distribute across the stereo field:

$$\text{pos}_i = -1 + \frac{2i}{k-1}, \quad i = 0, 1, \ldots, k-1$$

**Complexity:** O(k).

## Quick Start

```rust
use ternary_pan::*;

// Map ternary signal to pan positions
let signal = &[-1i8, 0, 1];
let pans = pan_position(signal);
assert_eq!(pans[0], Pan::left());
assert_eq!(pans[1], Pan::center());

// Equal-power panning at center
let (l, r) = pan_law_equal_power(Pan::center());
assert!((l - r).abs() < 0.01);

// Constant-power: l² + r² = 1
let (l, r) = pan_law_constant_power(Pan(0.3));
assert!((l*l + r*r - 1.0).abs() < 0.01);

// Distribute 5 agents across stereo field
let positions = surround(5);
assert_eq!(positions[0], Pan::left());
assert_eq!(positions[4], Pan::right());

// Auto-pan with ternary rhythm
let rhythm = &[1i8, 1, -1, 0, 1, -1];
let sweep = auto_pan(rhythm, 1.0);

// Apply pan to produce interleaved stereo
let stereo = apply_pan(&[100i8, -50, 75], Pan::center());
// [L0, R0, L1, R1, L2, R2]
```

## API

| Function | Description |
|---|---|
| `Pan::left() / center() / right()` | Canonical pan positions |
| `Pan::from_ternary(i8)` | Map {-1, 0, +1} → pan position |
| `.to_stereo_gains() → (f64, f64)` | Linear pan law gains |
| `pan_law_linear(Pan) → (f64, f64)` | Linear crossfade |
| `pan_law_equal_power(Pan) → (f64, f64)` | Cosine/sine panning |
| `pan_law_constant_power(Pan) → (f64, f64)` | Square-root panning |
| `pan_position(signal) → Vec<Pan>` | Map ternary signal to pan positions |
| `auto_pan(rhythm, width) → Vec<Pan>` | Accumulating sweep from ternary pattern |
| `surround(n) → Vec<Pan>` | Even spatial distribution of n agents |
| `apply_pan(signal, Pan) → Vec<i8>` | Interleaved stereo output |

## Architecture Notes

Stereo panning embodies the **γ + η = C** identity through energy conservation. The left channel carries the constructive mass γ (positive pan direction), the right channel carries the inhibitory mass η (negative pan direction), and the center (neutral) distributes energy equally. Under the constant-power law, $g_L^2 + g_R^2 = 1$ is exactly the conservation bound $C = 1$ — no matter where the source is placed, the total acoustic energy is invariant.

This makes constant-power panning the natural choice for ternary systems: moving an agent from left to right converts γ-mass to η-mass (or vice versa) while preserving $C$, exactly as the conservation identity demands.

## References

- Ballou, G. M. (2013). *Handbook for Sound Engineers.* 4th ed. Focal Press. (Pan laws)
- Pulkki, V. (2001). *Spatial Sound Generation and Perception by Amplitude Panning Techniques.* AES Convention Paper.
- Moore, B. C. J. (2012). *An Introduction to the Psychology of Hearing.* 6th ed. Brill. (Psychoacoustics)
- Roads, C. (1996). *The Computer Music Tutorial.* MIT Press.

## License

MIT
