use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

// Musical constants
const A4_FREQUENCY: f32 = 440.0;
const NOTE_DURATION: f32 = 0.09; // seconds per note
const PAUSE_DURATION: f32 = 0.2; // seconds pause between scales
const REPETITIONS_PER_SCALE: usize = 3; // How many times to repeat each scale

// The 12 chromatic notes starting from C
const CHROMATIC_NOTES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

// Circle of 5ths progression (note indices)
const CIRCLE_OF_FIFTHS: [usize; 12] = [0, 7, 2, 9, 4, 11, 6, 1, 8, 3, 10, 5]; // C, G, D, A, E, B, F#, C#, G#, D#, A#, F

// Musical modes with their interval patterns (in semitones)
const MODES: [(&str, [i32; 7]); 7] = [
    ("Ionian (Major)", [2, 2, 1, 2, 2, 2, 1]), // W-W-H-W-W-W-H
    ("Dorian", [2, 1, 2, 2, 2, 1, 2]),         // W-H-W-W-W-H-W
    ("Phrygian", [1, 2, 2, 2, 1, 2, 2]),       // H-W-W-W-H-W-W
    ("Lydian", [2, 2, 2, 1, 2, 2, 1]),         // W-W-W-H-W-W-H
    ("Mixolydian", [2, 2, 1, 2, 2, 1, 2]),     // W-W-H-W-W-H-W
    ("Aeolian (Natural Minor)", [2, 1, 2, 2, 1, 2, 2]), // W-H-W-W-H-W-W
    ("Locrian", [1, 2, 2, 1, 2, 2, 2]),        // H-W-W-H-W-W-W
];

#[derive(Debug, Clone)]
struct Note {
    name: String,
    frequency: f32,
}

#[derive(Debug, Clone)]
struct Scale {
    mode_name: String,
    root_note: String,
    is_major: bool,
    notes: Vec<Note>,
}

struct SequencePlayer {
    current_scale: usize,
    current_note: usize,
    current_repetition: usize,
    scales: Vec<Scale>,
    note_samples: usize,
    pause_samples: usize,
    samples_played: usize,
    in_pause: bool,
    notes_since_harmony_change: usize,
    current_harmony_index: usize,
}

impl SequencePlayer {
    fn new(scales: Vec<Scale>, sample_rate: f32) -> Self {
        let note_samples = (sample_rate * NOTE_DURATION) as usize;
        let pause_samples = (sample_rate * PAUSE_DURATION) as usize;

        Self {
            current_scale: 0,
            current_note: 0,
            current_repetition: 0,
            scales,
            note_samples,
            pause_samples,
            samples_played: 0,
            in_pause: false,
            notes_since_harmony_change: 0,
            current_harmony_index: 0,
        }
    }

    fn get_current_frequency(&self) -> f32 {
        if self.current_scale >= self.scales.len() {
            return 0.0; // Finished playing
        }

        if self.in_pause {
            return 0.0; // Silence during pause
        }

        let scale = &self.scales[self.current_scale];
        if self.current_note >= scale.notes.len() {
            return 0.0; // Should not happen
        }

        scale.notes[self.current_note].frequency
    }

    fn get_current_harmony_frequency(&self) -> f32 {
        if self.current_scale >= self.scales.len() || self.in_pause {
            return 0.0;
        }

        // Get the current scale to determine its root note
        let scale = &self.scales[self.current_scale];
        let scale_root_name = &scale.root_note;

        // Find the root note index
        let root_index = CHROMATIC_NOTES
            .iter()
            .position(|&note| note == scale_root_name)
            .unwrap_or(0);

        // Calculate mode-relative circle of 5ths starting from this root
        let fifth_semitone_offset = CIRCLE_OF_FIFTHS[self.current_harmony_index % 12];
        let harmony_note_index = (root_index + fifth_semitone_offset) % 12;
        let harmony_semitone = (harmony_note_index + 3 * 12) as i32; // One octave lower (3rd octave)
        note_to_frequency(harmony_semitone)
    }

    fn advance_sample(&mut self) -> bool {
        if self.current_scale >= self.scales.len() {
            return false; // Finished
        }

        self.samples_played += 1;

        if self.in_pause {
            if self.samples_played >= self.pause_samples {
                // Pause finished, move to next scale
                self.current_scale += 1;
                self.current_repetition = 0;
                self.current_note = 0;
                self.samples_played = 0;
                self.in_pause = false;

                // Print next scale info
                if self.current_scale < self.scales.len() {
                    let scale = &self.scales[self.current_scale];
                    let note_names: Vec<String> =
                        scale.notes.iter().map(|n| n.name.clone()).collect();
                    let total_performances = self.scales.len() * REPETITIONS_PER_SCALE;
                    let current_performance =
                        self.current_scale * REPETITIONS_PER_SCALE + self.current_repetition + 1;
                    println!(
                        "Playing {}/{}: {} {} {} (Rep {}/{}) - Notes: [{}]",
                        current_performance,
                        total_performances,
                        scale.root_note,
                        scale.mode_name,
                        if scale.is_major { "Major" } else { "Minor" },
                        self.current_repetition + 1,
                        REPETITIONS_PER_SCALE,
                        note_names.join(", ")
                    );
                }
            }
        } else if self.samples_played >= self.note_samples {
            // Note finished
            self.current_note += 1;
            self.samples_played = 0;

            // Track harmony progression (every 12 notes)
            self.notes_since_harmony_change += 1;
            if self.notes_since_harmony_change >= 12 {
                self.notes_since_harmony_change = 0;
                self.current_harmony_index = (self.current_harmony_index + 1) % 12;
            }

            let scale = &self.scales[self.current_scale];
            if self.current_note >= scale.notes.len() {
                // Scale finished
                self.current_note = 0;

                // Check if we need to repeat this scale or move to next
                if self.current_repetition + 1 < REPETITIONS_PER_SCALE {
                    // Still have repetitions left, continue immediately without pause
                    self.current_repetition += 1;

                    // Print repetition info
                    let note_names: Vec<String> =
                        scale.notes.iter().map(|n| n.name.clone()).collect();
                    let total_performances = self.scales.len() * REPETITIONS_PER_SCALE;
                    let current_performance =
                        self.current_scale * REPETITIONS_PER_SCALE + self.current_repetition + 1;

                    // Calculate mode-relative harmony note
                    let root_index = CHROMATIC_NOTES
                        .iter()
                        .position(|&note| note == scale.root_note)
                        .unwrap_or(0);
                    let fifth_semitone_offset = CIRCLE_OF_FIFTHS[self.current_harmony_index % 12];
                    let harmony_note_index = (root_index + fifth_semitone_offset) % 12;
                    let harmony_note = CHROMATIC_NOTES[harmony_note_index];

                    println!(
                        "Playing {}/{}: {} {} {} (Rep {}/{}) + {} harmony - Notes: [{}]",
                        current_performance,
                        total_performances,
                        scale.root_note,
                        scale.mode_name,
                        if scale.is_major { "Major" } else { "Minor" },
                        self.current_repetition + 1,
                        REPETITIONS_PER_SCALE,
                        harmony_note,
                        note_names.join(", ")
                    );
                } else {
                    // All repetitions done, move directly to next scale
                    self.current_scale += 1;
                    self.current_repetition = 0;

                    // Don't reset harmony - let it continue through the full circle of 5ths

                    // Print next scale info
                    if self.current_scale < self.scales.len() {
                        let scale = &self.scales[self.current_scale];
                        let note_names: Vec<String> =
                            scale.notes.iter().map(|n| n.name.clone()).collect();
                        let total_performances = self.scales.len() * REPETITIONS_PER_SCALE;
                        let current_performance = self.current_scale * REPETITIONS_PER_SCALE
                            + self.current_repetition
                            + 1;

                        // Calculate mode-relative harmony note
                        let root_index = CHROMATIC_NOTES
                            .iter()
                            .position(|&note| note == scale.root_note)
                            .unwrap_or(0);
                        let fifth_semitone_offset =
                            CIRCLE_OF_FIFTHS[self.current_harmony_index % 12];
                        let harmony_note_index = (root_index + fifth_semitone_offset) % 12;
                        let harmony_note = CHROMATIC_NOTES[harmony_note_index];

                        println!(
                            "Playing {}/{}: {} {} {} (Rep {}/{}) + {} harmony - Notes: [{}]",
                            current_performance,
                            total_performances,
                            scale.root_note,
                            scale.mode_name,
                            if scale.is_major { "Major" } else { "Minor" },
                            self.current_repetition + 1,
                            REPETITIONS_PER_SCALE,
                            harmony_note,
                            note_names.join(", ")
                        );
                    }
                }
            }
        }

        self.current_scale < self.scales.len()
    }

    fn is_finished(&self) -> bool {
        self.current_scale >= self.scales.len()
    }
}

fn main() {
    println!("Starting Musical Mode and Scale Player with Circle of 5ths Harmony");
    println!("Skipping redundant Aeolian Minor (same as Aeolian Major)");

    // Generate all scales
    let scales = generate_all_scales();
    let scale_count = scales.len();
    let total_performances = scale_count * REPETITIONS_PER_SCALE;

    println!("Generated {} unique scales total", scale_count);
    println!(
        "Each scale repeated {} times = {} total performances",
        REPETITIONS_PER_SCALE, total_performances
    );
    println!(
        "Each note duration: {}s, Second voice: Circle of 5ths (changes every 12 notes)\n",
        NOTE_DURATION
    );

    // Audio setup
    let sample_rate = 44100.0_f32;
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("Failed to find output device");
    let config = device
        .default_output_config()
        .expect("Failed to get default config");

    println!(
        "Audio Config - Sample rate: {}Hz, Channels: {}",
        config.sample_rate().0,
        config.channels()
    );

    // Create sequence player
    let sequence_player = Arc::new(Mutex::new(SequencePlayer::new(scales, sample_rate)));

    // Start first scale
    {
        let player = sequence_player.lock().unwrap();
        if !player.scales.is_empty() {
            let scale = &player.scales[0];
            let note_names: Vec<String> = scale.notes.iter().map(|n| n.name.clone()).collect();

            // Calculate mode-relative harmony for first scale
            let root_index = CHROMATIC_NOTES
                .iter()
                .position(|&note| note == scale.root_note)
                .unwrap_or(0);
            let fifth_semitone_offset = CIRCLE_OF_FIFTHS[0]; // Start with root (index 0)
            let harmony_note_index = (root_index + fifth_semitone_offset) % 12;
            let harmony_note = CHROMATIC_NOTES[harmony_note_index];

            println!(
                "Playing 1/{}: {} {} {} (Rep 1/{}) + {} harmony - Notes: [{}]",
                total_performances,
                scale.root_note,
                scale.mode_name,
                if scale.is_major { "Major" } else { "Minor" },
                REPETITIONS_PER_SCALE,
                harmony_note,
                note_names.join(", ")
            );
        }
    }

    // Build audio stream
    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => build_sequence_stream::<f32>(
            &device,
            &config.into(),
            sequence_player.clone(),
            sample_rate,
        ),
        cpal::SampleFormat::I16 => build_sequence_stream::<i16>(
            &device,
            &config.into(),
            sequence_player.clone(),
            sample_rate,
        ),
        cpal::SampleFormat::U16 => build_sequence_stream::<u16>(
            &device,
            &config.into(),
            sequence_player.clone(),
            sample_rate,
        ),
        _ => panic!("Unsupported sample format"),
    };

    // Start playback
    stream.play().expect("Failed to start stream");

    // Wait for completion
    loop {
        thread::sleep(Duration::from_millis(100));
        let player = sequence_player.lock().unwrap();
        if player.is_finished() {
            break;
        }
    }

    println!(
        "\nPlayback complete! All {} unique scales have been played {} times each ({} total performances).",
        scale_count, REPETITIONS_PER_SCALE, total_performances
    );
}

fn note_to_frequency(note_index: i32) -> f32 {
    // Calculate frequency using equal temperament
    // A4 (note index 9 in 4th octave) = 440Hz
    // Each semitone is 2^(1/12) times the previous
    let a4_index = 9 + 4 * 12; // A in 4th octave
    let semitones_from_a4 = note_index - a4_index;
    A4_FREQUENCY * 2.0_f32.powf(semitones_from_a4 as f32 / 12.0)
}

fn generate_scale(
    root_note_index: usize,
    mode_intervals: &[i32],
    is_major_variant: bool,
) -> Vec<Note> {
    let mut notes = Vec::new();
    let mut current_semitone = (root_note_index + 4 * 12) as i32; // Start in 4th octave

    // Choose intervals based on major/minor variant
    let intervals_to_use = if is_major_variant {
        mode_intervals
    } else {
        // For minor variant, use natural minor intervals: W-H-W-W-H-W-W
        &[2, 1, 2, 2, 1, 2, 2]
    };

    // Add root note
    let note_name = CHROMATIC_NOTES[root_note_index].to_string();
    notes.push(Note {
        name: note_name,
        frequency: note_to_frequency(current_semitone),
    });

    // Generate ascending scale based on chosen intervals
    for &interval in intervals_to_use {
        current_semitone += interval;
        let note_index = (current_semitone % 12 + 12) % 12; // Handle negative modulo
        let note_name = CHROMATIC_NOTES[note_index as usize].to_string();
        notes.push(Note {
            name: note_name,
            frequency: note_to_frequency(current_semitone),
        });
    }

    // Add descending notes (skip the last/highest note to avoid repetition)
    // Also skip the final root note to make it circular
    for i in (1..notes.len() - 1).rev() {
        notes.push(notes[i].clone());
    }

    notes
}

fn generate_all_scales() -> Vec<Scale> {
    let mut all_scales = Vec::new();

    // Create circular chromatic pattern: C → C# → D → D# → E → F → F# → G → G# → A → A# → B → A# → A → G# → G → F# → F → E → D# → D → C#
    let mut circular_root_indices = Vec::new();
    // Ascending
    for i in 0..12 {
        circular_root_indices.push(i);
    }
    // Descending (skip the highest note to avoid repetition, and skip the final root)
    for i in (1..11).rev() {
        circular_root_indices.push(i);
    }

    // For each mode
    for (mode_name, mode_intervals) in &MODES {
        // For each root note in circular pattern
        for &root_index in &circular_root_indices {
            let root_note = CHROMATIC_NOTES[root_index];

            // Major variant
            let major_notes = generate_scale(root_index, mode_intervals, true);
            all_scales.push(Scale {
                mode_name: mode_name.to_string(),
                root_note: root_note.to_string(),
                is_major: true,
                notes: major_notes,
            });

            // Minor variant - Skip Aeolian Minor since it's identical to Aeolian Major
            if *mode_name != "Aeolian (Natural Minor)" {
                let minor_notes = generate_scale(root_index, mode_intervals, false);
                all_scales.push(Scale {
                    mode_name: mode_name.to_string(),
                    root_note: root_note.to_string(),
                    is_major: false,
                    notes: minor_notes,
                });
            }
        }
    }

    all_scales
}

fn build_sequence_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    sequence_player: Arc<Mutex<SequencePlayer>>,
    sample_rate: f32,
) -> cpal::Stream
where
    T: cpal::Sample + cpal::SizedSample + cpal::FromSample<f32>,
{
    let channels = config.channels as usize;
    let mut melody_phase = 0.0f32;
    let mut harmony_phase = 0.0f32;

    device
        .build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                for frame in data.chunks_mut(channels) {
                    let mut player = sequence_player.lock().unwrap();

                    let melody_frequency = player.get_current_frequency();
                    let harmony_frequency = player.get_current_harmony_frequency();
                    let still_playing = player.advance_sample();

                    if !still_playing || melody_frequency == 0.0 {
                        // Silence
                        for sample in frame.iter_mut() {
                            *sample = T::from_sample(0.0);
                        }
                        melody_phase = 0.0; // Reset phases only during silence
                        harmony_phase = 0.0;
                    } else {
                        // Generate mixed sine waves with continuous phase
                        let melody_amplitude = 0.25;
                        let harmony_amplitude = 0.15;

                        let melody_value =
                            melody_amplitude * (2.0 * std::f32::consts::PI * melody_phase).sin();
                        let harmony_value = if harmony_frequency > 0.0 {
                            harmony_amplitude * (2.0 * std::f32::consts::PI * harmony_phase).sin()
                        } else {
                            0.0
                        };

                        let mixed_sample = melody_value + harmony_value;

                        // Fill all channels
                        for sample in frame.iter_mut() {
                            *sample = T::from_sample(mixed_sample);
                        }

                        // Update phases continuously for smooth transitions
                        melody_phase += melody_frequency / sample_rate;
                        if melody_phase >= 1.0 {
                            melody_phase -= 1.0;
                        }

                        if harmony_frequency > 0.0 {
                            harmony_phase += harmony_frequency / sample_rate;
                            if harmony_phase >= 1.0 {
                                harmony_phase -= 1.0;
                            }
                        }
                    }
                }
            },
            |err| eprintln!("Audio stream error: {}", err),
            None,
        )
        .expect("Failed to build stream")
}
