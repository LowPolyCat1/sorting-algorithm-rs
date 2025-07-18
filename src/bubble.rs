use crate::{draw_bars, play_tone};
use minifb::Window;
use rodio::Sink;

// Define constants for the window resolution
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

// Implements the Bubble Sort algorithm with visualization steps
// array: The array of numbers to be sorted (mutable)
// window: The minifb window to update (mutable)
// buffer: The pixel buffer (mutable)
// sink: The audio sink to play sounds on (mutable reference)
// use_sound: A boolean flag to control sound playback
// bar_width: The calculated width of each bar
// num_bars: The total number of bars
pub fn bubble_sort_visualized(
    array: &mut [u32],
    window: &mut Window,
    buffer: &mut [u32],
    sink: &Sink,
    use_sound: bool,
    bar_width: usize,
    num_bars: usize,
) {
    let n = array.len();
    // Outer loop for passes
    for i in 0..n {
        // Inner loop for comparisons and swaps
        for j in 0..(n - 1 - i) {
            if use_sound {
                // Play a short tone for every comparison step, pitch dependent on the value of array[j]
                play_tone(sink, array[j], 15, num_bars); // Duration is 15ms
            }

            // Draw the current state with elements being compared highlighted in red
            draw_bars(array, buffer, Some(j), Some(j + 1), Some(n - i), bar_width);
            // Perform the comparison and swap if necessary
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
            // Update the window to show the current frame
            window.update_with_buffer(buffer, WIDTH, HEIGHT).unwrap();
        }
    }
}
