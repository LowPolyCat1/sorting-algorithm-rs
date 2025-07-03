use rodio::{Sink, Source};
use std::time::Duration;

#[cfg(test)]
pub mod tests;

pub mod bubble;
pub mod insertion;
pub mod merge;
pub mod quick;
pub mod selection;

// Define constants for the window resolution
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

// Function to draw the bars on the pixel buffer
// array: The current state of the array to visualize
// buffer: The pixel buffer (Vec<u32>) where pixels are drawn
// highlight_idx1: Optional index of the first bar to highlight (e.g., during comparison)
// highlight_idx2: Optional index of the second bar to highlight (e.g., during comparison or swap)
// sorted_until_idx: Index up to which elements are considered sorted (e.g., for Bubble Sort, the end of the sorted portion)
// bar_width: The calculated width of each bar
pub fn draw_bars(
    array: &[u32],
    buffer: &mut Vec<u32>,
    highlight_idx1: Option<usize>,
    highlight_idx2: Option<usize>,
    sorted_until_idx: Option<usize>,
    bar_width: usize,
) {
    // Clear the buffer by filling it with black (0x000000)
    buffer.fill(0x000000);

    // Iterate through the array to draw each bar
    for (i, &value) in array.iter().enumerate() {
        // Calculate the height of the bar based on its value.
        // Normalize the value to fit within the window height.
        let bar_height = (value as f32 / array.len() as f32 * HEIGHT as f32) as usize;

        // Calculate the starting X position for the current bar
        let x_start = i * bar_width;

        // Determine the color of the bar
        let color = if Some(i) == highlight_idx1 || Some(i) == highlight_idx2 {
            // If the bar is highlighted, use red (0xFF0000)
            0xFF0000 // Hexadecimal literal for red
        } else if let Some(sorted_idx) = sorted_until_idx {
            if i >= sorted_idx {
                // If the bar is in the sorted portion, use green (0x00FF00)
                0x00FF00 // Hexadecimal literal for green
            } else {
                // Default color for unsorted bars is blue (0x0000FF)
                0x0000FF // Hexadecimal literal for blue
            }
        } else {
            // Default color if no specific highlighting or sorted portion is defined
            0x0000FF // Hexadecimal literal for blue
        };

        // Draw the vertical bar pixel by pixel
        for y in (HEIGHT - bar_height)..HEIGHT {
            // From bottom of the window up to bar_height
            for x in x_start..(x_start + bar_width) {
                // Ensure coordinates are within buffer bounds
                if x < WIDTH && y < HEIGHT {
                    buffer[y * WIDTH + x] = color;
                }
            }
        }
    }
}

// Function to play a short tone
// sink: The audio sink to append the sound to
// value: The value of the bar, used to determine pitch
// duration_ms: The duration of the tone in milliseconds
// num_bars: The total number of bars, used for pitch normalization
pub fn play_tone(sink: &Sink, value: u32, duration_ms: u64, num_bars: usize) {
    // Map the bar value (1 to num_bars) to a frequency range (e.g., 400 Hz to 1600 Hz)
    // This range is chosen to be clearly audible and provide a good sweep.
    let min_freq = 400.0;
    let max_freq = 1600.0;
    let freq_range = max_freq - min_freq;
    // Normalize the value: (value - min_value) / (max_value - min_value)
    // Here, min_value is 1 and max_value is num_bars
    let value_normalized = (value - 1) as f32 / (num_bars - 1) as f32;
    let freq = min_freq + value_normalized * freq_range;

    let source = rodio::source::SineWave::new(freq)
        .take_duration(Duration::from_millis(duration_ms))
        .amplify(0.15); // Adjust volume (0.0 to 1.0)
    sink.append(source);
    sink.sleep_until_end(); // This will block until the sound has finished playing
}
