# ternary-pan

**Three positions in stereo space. Hard left, dead center, hard right. Every ternary signal is a panning map.**

Panning is where a sound sits in the stereo field. Left ear, right ear, or both equally — that's the spatial dimension of audio, reduced to its simplest form. In ternary, the three states map perfectly to three pan positions: `-1 = full left`, `0 = center`, `+1 = full right`. A ternary signal isn't just data — it's a sequence of spatial positions. A melody that moves through {-1, 0, +1} is a sound that moves through space.

This crate provides panning, auto-pan (spatial sweep driven by a ternary rhythm), surround distribution (spread N agents across the field), and stereo processing with gain computation.

## What's Inside

- **`Pan`** — panning position in [-1.0, +1.0]. Construct from ternary: `Pan::from_ternary(1)` = right
- **`to_stereo_gains()`** — linear pan law: compute (left_gain, right_gain). Center = (0.5, 0.5)
- **`pan_position(signal)`** — map a ternary signal to panning positions
- **`auto_pan(rhythm, sweep_width)`** — sweep across the stereo field driven by a ternary rhythm
- **`surround(agent_count)`** — distribute N agents evenly from left to right
- **`stereo_process(signal, pan)`** — apply panning to a ternary signal, produce (left, right) channels

## Quick Example

```rust
use ternary_pan::*;

// Three pan positions from ternary values
assert_eq!(Pan::from_ternary(-1), Pan::left());    // full left
assert_eq!(Pan::from_ternary(0), Pan::center());   // dead center
assert_eq!(Pan::from_ternary(1), Pan::right());    // full right

// Stereo gains for each position
let (l, r) = Pan::left().to_stereo_gains();
assert_eq!((l, r), (1.0, 0.0)); // all left, no right

let (l, r) = Pan::center().to_stereo_gains();
assert_eq!((l, r), (0.5, 0.5)); // equal in both

// Auto-pan: rhythm drives spatial movement
let rhythm = vec![1, 0, -1, 0]; // right → center → left → center
let sweeps = auto_pan(&rhythm, 1.0);

// Distribute 5 agents across the field
let positions = surround(5);
// Left, Left-Center, Center, Right-Center, Right
```

## The Deeper Truth

**Ternary panning is pixel-art spatial positioning.** Three pan positions — left, center, right — creates a spatial texture where sounds *snap* to positions rather than gliding. This is the audio equivalent of pixel art: deliberate, chunky, distinctive. It's the sound of early video games, where every sound effect was panned hard left, hard right, or dead center. There's no subtlety — and that's the point.

When you need smooth movement, use `auto_pan` with continuous accumulation (the pan position drifts gradually, driven by the ternary rhythm). When you want the raw three-position aesthetic — sounds teleporting between speakers — use `from_ternary` directly. Both approaches are valid. Both sound completely different.

**Use cases:**
- **Audio spatialization** — place sounds in stereo space with ternary control
- **Game audio** — quick spatial positioning for sound effects
- **Music production** — auto-pan effects driven by ternary rhythms
- **Multi-agent sonification** — each agent gets a position in the field
- **Accessibility** — ternary spatial cues (left/center/right) are easy to perceive

## See Also

- **ternary-mixer** — blend panned signals together
- **ternary-echo** — echo creates depth (front/back), pan creates width (left/right)
- **ternary-vu** — meter the output of panned signals
- **ternary-wave** — generate the signals you're panning
- **ternary-crossfader** — crossfading is a 1D spatial operation
- **ternary-rack** — wire pan modules into a modular synth

## Install

```bash
cargo add ternary-pan
```

## License

MIT
