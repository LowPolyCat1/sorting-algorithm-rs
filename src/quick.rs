use crate::{draw_bars, play_tone};
use minifb::Window;
use rodio::Sink;

// Define constants for the window resolution
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

// Helper function for Quick Sort partitioning
#[allow(clippy::too_many_arguments)]
fn partition(
    array: &mut [u32],
    low: usize,
    high: usize,
    window: &mut Window,
    buffer: &mut [u32],
    sink: &Sink,
    use_sound: bool,
    bar_width: usize,
    num_bars: usize,
) -> usize {
    let pivot = array[high]; // Choose the last element as the pivot
    let mut i = low; // Index of smaller element

    for j in low..high {
        if use_sound {
            // Play tone for comparison (array[j] with pivot)
            play_tone(sink, array[j], 5, num_bars); // Shorter tone for faster Quick Sort
        }

        // Draw elements being compared to the pivot
        draw_bars(array, buffer, Some(j), Some(high), None, bar_width);
        window.update_with_buffer(buffer, WIDTH, HEIGHT).unwrap();

        // If current element is smaller than or equal to pivot
        if array[j] <= pivot {
            array.swap(i, j); // Swap elements
            if use_sound {
                // Play tone for swap
                play_tone(sink, array[i], 5, num_bars);
            }
            // Draw after swap
            draw_bars(array, buffer, Some(i), Some(j), None, bar_width);
            window.update_with_buffer(buffer, WIDTH, HEIGHT).unwrap();
            i += 1;
        }
    }
    array.swap(i, high); // Place the pivot at its correct sorted position
    if use_sound {
        // Play tone for final pivot placement
        play_tone(sink, array[i], 5, num_bars);
    }
    // Draw final pivot placement
    draw_bars(array, buffer, Some(i), Some(high), None, bar_width);
    window.update_with_buffer(buffer, WIDTH, HEIGHT).unwrap();
    i
}

// Recursive function for Quick Sort visualization
#[allow(clippy::too_many_arguments)]
fn quick_sort_recursive(
    array: &mut [u32],
    low: usize,
    high: usize,
    window: &mut Window,
    buffer: &mut [u32],
    sink: &Sink,
    use_sound: bool,
    bar_width: usize,
    num_bars: usize,
) {
    if low < high {
        let pi = partition(
            array, low, high, window, buffer, sink, use_sound, bar_width, num_bars,
        );

        // Recursively sort elements before partition and after partition
        if pi > 0 {
            // Ensure pi is not 0 to prevent underflow with pi - 1
            quick_sort_recursive(
                array,
                low,
                pi - 1,
                window,
                buffer,
                sink,
                use_sound,
                bar_width,
                num_bars,
            );
        }
        quick_sort_recursive(
            array,
            pi + 1,
            high,
            window,
            buffer,
            sink,
            use_sound,
            bar_width,
            num_bars,
        );
    }
    // After a sub-array is sorted, mark its elements as sorted (green)
    // This is a simplification for visualization; in true Quick Sort, elements are not 'sorted' until the very end.
    // However, for visual feedback, we can mark elements as sorted when their partition is complete.
    if low <= high && high < array.len() {
        // Ensure indices are valid
        draw_bars(array, buffer, None, None, Some(low), bar_width); // Mark from low onwards as sorted
        window.update_with_buffer(buffer, WIDTH, HEIGHT).unwrap();
    }
}

// Wrapper for Quick Sort visualization
pub fn quick_sort_visualized(
    array: &mut [u32],
    window: &mut Window,
    buffer: &mut [u32],
    sink: &Sink,
    use_sound: bool,
    bar_width: usize,
    num_bars: usize,
) {
    let n = array.len();
    if n == 0 {
        return;
    } // Handle empty array
    quick_sort_recursive(
        array,
        0,
        n - 1,
        window,
        buffer,
        sink,
        use_sound,
        bar_width,
        num_bars,
    );
    // Final draw to ensure all bars are green
    draw_bars(array, buffer, None, None, Some(0), bar_width);
    window.update_with_buffer(buffer, WIDTH, HEIGHT).unwrap();
}
