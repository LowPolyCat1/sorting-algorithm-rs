use crate::{draw_bars, play_tone};
use minifb::Window;
use rodio::Sink;

// Define constants for the window resolution
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

// Implements the Selection Sort algorithm with visualization steps
// array: The array of numbers to be sorted (mutable)
// window: The minifb window to update (mutable)
// buffer: The pixel buffer (mutable)
// sink: The audio sink to play sounds on (mutable reference)
// use_sound: A boolean flag to control sound playback
// bar_width: The calculated width of each bar
// num_bars: The total number of bars
pub fn selection_sort_visualized(
    array: &mut Vec<u32>,
    window: &mut Window,
    buffer: &mut Vec<u32>,
    sink: &Sink,
    use_sound: bool,
    bar_width: usize,
    num_bars: usize,
) {
    let n = array.len();
    // Outer loop: Iterate through the unsorted portion of the array
    for i in 0..(n - 1) {
        let mut min_idx = i; // Assume the current element is the minimum

        // Inner loop: Find the minimum element in the remaining unsorted array
        for j in (i + 1)..n {
            if use_sound {
                // Play a tone for every comparison step, pitch dependent on the value of array[j]
                play_tone(sink, array[j], 15, num_bars); // Duration is 15ms
            }

            // Draw the current state:
            // array[i] is the element being placed
            // array[j] is the element currently being compared
            // array[min_idx] is the current minimum found
            draw_bars(array, buffer, Some(j), Some(min_idx), Some(i), bar_width); // Highlight j and min_idx
            window.update_with_buffer(buffer, WIDTH, HEIGHT).unwrap();

            // Compare elements
            if array[j] < array[min_idx] {
                min_idx = j; // Update min_idx if a smaller element is found
            }
        }

        // After the inner loop, if the minimum element is not at the current position 'i', swap them
        if min_idx != i {
            array.swap(i, min_idx);
            if use_sound {
                // Play a tone for the swap, pitch dependent on the value that moved to position i
                play_tone(sink, array[i], 15, num_bars); // Play tone for swapped element
            }
            draw_bars(
                array,
                buffer,
                Some(i),
                Some(min_idx),
                Some(i + 1),
                bar_width,
            ); // Highlight swapped elements
            window.update_with_buffer(buffer, WIDTH, HEIGHT).unwrap();
        } else {
            // If no swap occurred, still update the visualization to show 'i' is now sorted
            draw_bars(array, buffer, None, None, Some(i + 1), bar_width);
            window.update_with_buffer(buffer, WIDTH, HEIGHT).unwrap();
        }
    }
}
