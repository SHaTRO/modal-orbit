# Modal Orbit

**A Musical Mode and Scale Player with Circle of 5ths Harmony**

A comprehensive musical exploration system that systematically plays all Western musical modes across all keys with intelligent harmonic accompaniment. This project demonstrates the mathematical beauty underlying music theory through four interconnected circular patterns.

## Quick Start

```bash
cargo run
```

The program will play 858 unique scale performances over approximately 15 minutes, showcasing the complete spectrum of Western tonal harmony.

## System Overview

### Four-Tier Hierarchical Architecture

This system implements four interconnected circular patterns that create a complete exploration of Western musical harmony:

1. **Mode Cycle**: 7 musical modes (Ionian, Dorian, Phrygian, Lydian, Mixolydian, Aeolian, Locrian)
2. **Root Note Cycle**: 22-position circular chromatic progression (C→C#→...→B→A#→...→C#)
3. **Scale Pattern Cycle**: Each scale plays in circular ascending-descending pattern
4. **Harmony Cycle**: Mode-relative circle of 5ths providing harmonic foundation

### Mathematical Foundation

- **Total Unique Scales**: 286 (7 modes × 22 root positions, minus 12 redundant Aeolian Minor scales)
- **Total Performances**: 858 (286 scales × 3 repetitions each)
- **Notes per Scale**: 14 (circular pattern: up to octave, down without repeating highest note)
- **Harmony Changes**: Every 12 notes, following circle of 5ths relative to each scale's root
- **Audio Quality**: 44.1kHz, equal temperament tuning (A4 = 440Hz)

## Musical Education Guide

### Understanding Musical Modes

Each mode represents a unique rotation of the major scale pattern, creating distinct emotional and harmonic characteristics:

#### 🎵 Ionian (Major Mode)
- **Pattern**: W-W-H-W-W-W-H (Whole-Whole-Half-Whole-Whole-Whole-Half)
- **Character**: Bright, resolved, classical
- **What to Listen For**: Strong sense of resolution to the root, uplifting quality
- **Common Usage**: Traditional Western music, pop, classical
- **Example**: C Ionian = C-D-E-F-G-A-B-C

#### 🎵 Dorian
- **Pattern**: W-H-W-W-W-H-W
- **Character**: Minor with a raised 6th, sophisticated
- **What to Listen For**: Minor tonality but brighter than natural minor, jazz-like quality
- **Common Usage**: Jazz, folk, Celtic music, film scores
- **Harmonic Interest**: The raised 6th creates beautiful melodic possibilities
- **Example**: C Dorian = C-D-D#-F-G-A-A#-C

#### 🎵 Phrygian
- **Pattern**: H-W-W-W-H-W-W
- **Character**: Exotic, Spanish/Middle Eastern flavor
- **What to Listen For**: The flat 2nd (half-step from root) creates immediate tension
- **Common Usage**: Flamenco, metal, Spanish music, film scores for mysterious scenes
- **Harmonic Interest**: Very distinctive sound due to the prominent flat 2nd interval
- **Example**: C Phrygian = C-C#-D#-F-G-G#-A#-C

#### 🎵 Lydian
- **Pattern**: W-W-W-H-W-W-H
- **Character**: Dreamy, ethereal, floating quality
- **What to Listen For**: The raised 4th creates an "otherworldly" brightness
- **Common Usage**: Film scores, progressive rock, jazz fusion
- **Harmonic Interest**: The tritone between root and 4th creates harmonic ambiguity
- **Example**: C Lydian = C-D-E-F#-G-A-B-C

#### 🎵 Mixolydian
- **Pattern**: W-W-H-W-W-H-W
- **Character**: Major with bluesy edge, rock-oriented
- **What to Listen For**: Major feel with a flat 7th that never fully resolves
- **Common Usage**: Rock, blues, folk, Celtic music
- **Harmonic Interest**: The flat 7th creates subdued, less resolved feeling than Ionian
- **Example**: C Mixolydian = C-D-E-F-G-A-A#-C

#### 🎵 Aeolian (Natural Minor)
- **Pattern**: W-H-W-W-H-W-W
- **Character**: Pure minor, melancholic, classical minor sound
- **What to Listen For**: Classic "sad" minor quality, emotionally expressive
- **Common Usage**: Classical music, ballads, emotional pieces
- **Harmonic Interest**: The natural minor provides the foundation for minor key harmony
- **Example**: C Aeolian = C-D-D#-F-G-G#-A#-C

#### 🎵 Locrian
- **Pattern**: H-W-W-H-W-W-W
- **Character**: Diminished, unstable, unresolved
- **What to Listen For**: Constant tension due to diminished 5th, very unstable
- **Common Usage**: Rarely used traditionally, modern jazz, experimental music
- **Harmonic Interest**: The diminished 5th makes it extremely difficult to establish as a tonal center
- **Example**: C Locrian = C-C#-D#-F-F#-G#-A#-C

### Circle of 5ths Harmony System

The harmony voice follows the circle of 5ths pattern relative to each scale's root note:

**Pattern**: Root → 5th → 2nd → 6th → 3rd → 7th → #4th → #1st → #5th → #2nd → #6th → 4th

For a C-based scale: **C → G → D → A → E → B → F# → C# → G# → D# → A# → F**

**Key Features**:
- Harmony changes every 12 melody notes
- Played one octave below the melody (3rd octave)
- Creates strong harmonic foundation that reinforces the modal character
- Each scale gets a complete harmonic journey through its relative circle of 5ths

### What to Listen For During Playback

**Mode Transitions**: Notice how the character changes dramatically between modes even with the same root note

**Harmonic Movement**: Pay attention to how the bass harmony supports and sometimes creates tension with the melody

**Circular Patterns**: Listen for how scales flow seamlessly into their repetitions without jarring transitions

**Major vs Minor Variants**: Compare how the same mode sounds in major vs minor (except Aeolian which only appears once)

**Root Note Patterns**: Notice the smooth chromatic progression through all 12 starting notes, then the descent

## Technical Implementation

### Core Algorithm

The system generates scales using a four-step process:

1. **Mode Selection**: Iterate through all 7 musical modes
2. **Root Note Selection**: Follow circular chromatic pattern (22 positions total)
3. **Scale Generation**: Create both major and minor variants (where applicable)
4. **Harmonic Assignment**: Calculate mode-relative circle of 5ths progression

### Key Technical Features

#### Frequency Calculation
```rust
fn note_to_frequency(note_index: i32) -> f32 {
    let a4_index = 9 + 4 * 12; // A4 reference
    let semitones_from_a4 = note_index - a4_index;
    A4_FREQUENCY * 2.0_f32.powf(semitones_from_a4 as f32 / 12.0)
}
```
Uses equal temperament tuning with A4 = 440Hz as reference.

#### Circular Scale Pattern
Each scale is generated as a 14-note circular pattern:
- 8 ascending notes (root to octave)
- 6 descending notes (skipping octave and final root for seamless repetition)

#### Mode-Relative Harmony
```rust
let harmony_note_index = (root_index + CIRCLE_OF_FIFTHS[harmony_index]) % 12;
```
Harmony is calculated relative to each scale's root, ensuring musically appropriate relationships.

#### Dual-Voice Audio Synthesis
- **Melody Voice**: 0.25 amplitude, 4th octave
- **Harmony Voice**: 0.15 amplitude, 3rd octave (one octave lower)
- **Continuous Phase**: Smooth transitions between notes prevent audio artifacts

### System Performance

- **Scale Generation**: 286 unique scales
- **Audio Duration**: ~15 minutes continuous playback
- **Note Timing**: 0.09 seconds per note
- **Memory Efficiency**: Pre-calculated scales stored in memory
- **Real-time Processing**: Sub-millisecond audio buffer processing

### Code Organization

- **`MODES`**: Interval patterns for all 7 musical modes
- **`CIRCLE_OF_FIFTHS`**: Harmonic progression template
- **`SequencePlayer`**: State management for playback progression
- **`generate_scale()`**: Creates circular scale patterns with major/minor variants
- **`build_sequence_stream()`**: Real-time audio synthesis with dual-voice mixing

## Customization Options

### Timing Parameters
```rust
const NOTE_DURATION: f32 = 0.09;     // seconds per note
const REPETITIONS_PER_SCALE: usize = 3; // repetitions per scale
```

### Audio Balance
```rust
let melody_amplitude = 0.25;   // melody voice volume
let harmony_amplitude = 0.15;  // harmony voice volume  
```

### Harmonic Progression
Modify `CIRCLE_OF_FIFTHS` array to experiment with different harmonic patterns.

## Musical Learning Applications

### Ear Training
- **Mode Recognition**: Learn to distinguish between modal characters
- **Harmonic Progression**: Understand circle of 5ths relationships
- **Scale Patterns**: Internalize ascending/descending scale movements

### Composition
- **Modal Inspiration**: Discover unique characteristics of lesser-known modes
- **Harmonic Ideas**: Hear how different bass notes support modal melodies
- **Voice Leading**: Understand smooth harmonic transitions

### Music Theory Study
- **Interval Relationships**: Experience how different interval patterns create mode character
- **Harmonic Function**: Understand how circle of 5ths provides harmonic foundation
- **Chromatic Exploration**: Systematic exposure to all possible tonal combinations

## Dependencies

- **cpal**: Cross-platform audio I/O library for real-time audio output
- **Rust standard library**: Threading, synchronization, mathematical functions

## Building and Running

### Prerequisites
- Rust toolchain (1.70 or later)
- Audio output device

### Compilation
```bash
cd modal-orbit
cargo build --release
```

### Execution
```bash
cargo run
```

The program will automatically detect your default audio output device and begin the complete musical exploration sequence.

## Mathematical Insights

### Circular Mathematics
The system demonstrates how musical relationships can be expressed as circular mathematical functions:
- **Chromatic Circle**: 12-tone equal temperament as modular arithmetic
- **Fifth Relationships**: Perfect mathematical ratios in equal temperament approximation
- **Pattern Symmetry**: Ascending/descending patterns create musical symmetry

### Harmonic Series
The dual-voice system creates interference patterns that highlight harmonic relationships inherent in the mathematical structure of musical intervals.

### Algorithmic Composition
This system represents a form of algorithmic composition where mathematical structures generate musically meaningful content through systematic exploration of tonal space.

---

*This project demonstrates how systematic mathematical approaches can reveal the deep structures underlying musical harmony and create comprehensive educational tools for musical exploration.*
