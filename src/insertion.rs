use crate::{draw_bars, play_tone};
use minifb::Window;
use rodio::Sink;

// Define constants for the window resolution
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

// Implements the Insertion Sort algorithm with visualization steps
// array: The array of numbers to be sorted (mutable)
// window: The minifb window to update (mutable)
// buffer: The pixel buffer (mutable)
// sink: The audio sink to play sounds on (mutable reference)
// use_sound: A boolean flag to control sound playback
// bar_width: The calculated width of each bar
// num_bars: The total number of bars
pub fn insertion_sort_visualized(
    array: &mut Vec<u32>,
    window: &mut Window,
    buffer: &mut Vec<u32>,
    sink: &Sink,
    use_sound: bool,
    bar_width: usize,
    num_bars: usize,
) {
    let n = array.len();
    // Iterate from the second element to the end of the array
    for i in 1..n {
        let key = array[i]; // The element to be inserted
        let mut j = i; // Index for comparison

        // Move elements of array[0..i-1], that are greater than key,
        // to one position ahead of their current position
        while j > 0 && array[j - 1] > key {
            if use_sound {
                // Play a tone for each comparison/shift
                play_tone(sink, array[j - 1], 15, num_bars);
            }
            array[j] = array[j - 1]; // Shift element to the right
            j -= 1;

            // Visualize the shift
            draw_bars(array, buffer, Some(j), Some(j + 1), Some(i), bar_width);
            window.update_with_buffer(buffer, WIDTH, HEIGHT).unwrap();
        }
        // Place the key at its correct position
        if array[j] != key {
            // Only update if a shift occurred or it's the first element
            array[j] = key;
            if use_sound {
                // Play a tone for the final placement of the key
                play_tone(sink, array[j], 15, num_bars);
            }
            draw_bars(array, buffer, Some(j), None, Some(i + 1), bar_width);
            window.update_with_buffer(buffer, WIDTH, HEIGHT).unwrap();
        } else {
            // If no shifts occurred, still update visualization for 'i' being sorted
            draw_bars(array, buffer, None, None, Some(i + 1), bar_width);
            window.update_with_buffer(buffer, WIDTH, HEIGHT).unwrap();
        }
    }
}
