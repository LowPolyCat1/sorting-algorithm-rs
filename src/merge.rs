use crate::{draw_bars, play_tone};
use minifb::Window;
use rodio::Sink;

// Define constants for the window resolution
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

#[allow(clippy::too_many_arguments)]
pub fn merge(
    array: &mut [u32],
    left: usize,
    mid: usize,
    right: usize,
    window: &mut Window,
    buffer: &mut [u32],
    sink: &Sink,
    use_sound: bool,
    bar_width: usize,
    num_bars: usize,
) {
    let n1 = mid - left + 1;
    let n2 = right - mid;

    #[allow(non_snake_case)]
    let mut L = vec![0; n1];
    #[allow(non_snake_case)]
    let mut R = vec![0; n2];

    L[..n1].copy_from_slice(&array[left..left + n1]);
    for j in 0..n2 {
        R[j] = array[mid + 1 + j];
    }

    let mut i = 0; // Initial index of first subarray
    let mut j = 0; // Initial index of second subarray
    let mut k = left; // Initial index of merged subarray

    while i < n1 && j < n2 {
        if use_sound {
            // Play a tone for comparison during merge
            play_tone(sink, L[i].min(R[j]), 5, num_bars); // Shorter tone for faster merge
        }

        if L[i] <= R[j] {
            array[k] = L[i];
            i += 1;
        } else {
            array[k] = R[j];
            j += 1;
        }
        // Visualize the element being placed
        draw_bars(array, buffer, Some(k), None, None, bar_width);
        window.update_with_buffer(buffer, WIDTH, HEIGHT).unwrap();
        k += 1;
    }

    // Copy the remaining elements of L[], if any
    while i < n1 {
        if use_sound {
            play_tone(sink, L[i], 5, num_bars);
        }
        array[k] = L[i];
        // Visualize the element being placed
        draw_bars(array, buffer, Some(k), None, None, bar_width);
        window.update_with_buffer(buffer, WIDTH, HEIGHT).unwrap();
        i += 1;
        k += 1;
    }

    // Copy the remaining elements of R[], if any
    while j < n2 {
        if use_sound {
            play_tone(sink, R[j], 5, num_bars);
        }
        array[k] = R[j];
        // Visualize the element being placed
        draw_bars(array, buffer, Some(k), None, None, bar_width);
        window.update_with_buffer(buffer, WIDTH, HEIGHT).unwrap();
        j += 1;
        k += 1;
    }
}

// Helper function for Merge Sort

// Recursive function for Merge Sort visualization
#[allow(clippy::too_many_arguments)]
pub fn merge_sort_recursive(
    array: &mut [u32],
    left: usize,
    right: usize,
    window: &mut Window,
    buffer: &mut [u32],
    sink: &Sink,
    use_sound: bool,
    bar_width: usize,
    num_bars: usize,
) {
    if left < right {
        let mid = left + (right - left) / 2; // Avoid overflow for large left and right

        // Recursively sort first and second halves
        merge_sort_recursive(
            array, left, mid, window, buffer, sink, use_sound, bar_width, num_bars,
        );
        merge_sort_recursive(
            array,
            mid + 1,
            right,
            window,
            buffer,
            sink,
            use_sound,
            bar_width,
            num_bars,
        );

        // Merge the sorted halves
        merge(
            array, left, mid, right, window, buffer, sink, use_sound, bar_width, num_bars,
        );
    }
}

// Wrapper for Merge Sort visualization
pub fn merge_sort_visualized(
    array: &mut [u32],
    window: &mut Window,
    buffer: &mut [u32],
    sink: &Sink,
    use_sound: bool,
    bar_width: usize,
    num_bars: usize,
) {
    let n = array.len();
    merge_sort_recursive(
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
    // After sorting, draw the final sorted state in green
    draw_bars(array, buffer, None, None, Some(0), bar_width);
    window.update_with_buffer(buffer, WIDTH, HEIGHT).unwrap();
}
