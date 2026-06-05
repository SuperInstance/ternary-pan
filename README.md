# ternary-pan

**Stereo panning and spatial distribution for ternary audio signals.**

Panning is the simplest spatial operation: place a sound somewhere between left and right. In ternary, the three states map naturally to three positions: `-1 = full left`, `0 = center`, `+1 = full right`. Every ternary signal is a panning sequence.

This crate provides panning, auto-pan (sweeping across the stereo field driven by a ternary rhythm), surround distribution (spread N agents across the field), and stereo processing with gain computation.

## What's Inside

- **`Pan`** — panning position in [-1.0, +1.0]. Construct from ternary: `Pan::from_ternary(1)` = right
- **`to_stereo_gains()`** — linear pan law: compute (left_gain, right_gain) for the pan position
- **`pan_position(signal)`** — map a ternary signal to panning positions
- **`auto_pan(rhythm, sweep_width)`** — sweep across stereo field driven by a ternary rhythm pattern
- **`surround(agent_count)`** — distribute N agents evenly from left to right
- **`stereo_process(signal, pan)`** — apply panning to a ternary signal, produce (left, right) channels

## Quick Example

```rust
use ternary_pan::*;

// Pan positions from ternary
let positions = pan_position(&[1, 0, -1, 0, 1]);
// [Right, Center, Left, Center, Right]

// Stereo gains for center panning
let center = Pan::center();
let (l, r) = center.to_stereo_gains();
assert_eq!((l, r), (0.5, 0.5));

// Stereo gains for hard left
let left = Pan::left();
let (l, r) = left.to_stereo_gains();
assert_eq!((l, r), (1.0, 0.0));

// Auto-pan: rhythm drives the sweep
let rhythm = vec![1, 0, -1, 0, 1, 0, -1, 0];
let sweeps = auto_pan(&rhythm, 1.0);
// Positions sweep left and right following the rhythm

// Distribute 5 agents across the field
let positions = surround(5);
// [Left, Left-Center, Center, Right-Center, Right]
```

## The Deeper Truth

**Ternary panning is quantized positioning.** With only three pan positions (left/center/right), you get a very specific spatial texture: sounds snap to positions rather than gliding between them. This is the audio equivalent of pixel art — deliberate, chunky, distinctive. When you need smooth spatial movement, use the `auto_pan` function with continuous accumulation. When you want the raw three-position aesthetic, use `from_ternary` directly.

The linear pan law (left = (1-pan)/2, right = (1+pan)/2) preserves energy at center but reduces perceived volume at the extremes. For ternary signals this is actually desirable — extreme-panned sounds *should* feel smaller. They're at the edges, not in the center of attention.

**Use cases:**
- **Audio spatialization** — place sounds in stereo space with ternary control
- **Game audio** — quick spatial positioning for sound effects
- **Music production** — auto-pan effects driven by ternary rhythms
- **Multi-agent sonification** — each agent gets a position in the field
- **Accessibility** — ternary spatial cues (left/center/right) are easy to perceive

## See Also

- **ternary-mixer** — blend panned signals together
- **ternary-echo** — echo creates spatial depth (front/back dimension)
- **ternary-vu** — meter the output of panned signals
- **ternary-wave** — generate the signals you're panning
- **ternary-crossfader** — crossfade between two panned sources

## Install

```bash
cargo add ternary-pan
```

## License

MIT
