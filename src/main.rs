// Import necessary crates and modules
use minifb::{Key, Window, WindowOptions}; // For creating a window and handling input
use rand::Rng;
use rodio::{OutputStream, Sink};
use sorting_algorithm_visualizer_rs::*;
use std::io;

// Define constants for the window resolution
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

fn main() {
    // --- User Input Section ---

    // 1. Ask for sound preference
    let mut sound_choice = String::new();
    println!("Do you want sound? (y/n)");
    io::stdin()
        .read_line(&mut sound_choice)
        .expect("Failed to read line");
    let use_sound = sound_choice.trim().eq_ignore_ascii_case("y");

    // 2. Ask for number of bars
    let mut num_bars_input = String::new();
    let num_bars: usize;
    loop {
        println!("How many bars do you want? (e.g., 50-400, max {})", WIDTH);
        io::stdin()
            .read_line(&mut num_bars_input)
            .expect("Failed to read line");
        match num_bars_input.trim().parse::<usize>() {
            Ok(n) if n > 0 && n <= WIDTH => {
                num_bars = n;
                break;
            }
            _ => {
                println!(
                    "Invalid input. Please enter a positive integer less than or equal to {}.",
                    WIDTH
                );
                num_bars_input.clear(); // Clear the buffer for next input
            }
        }
    }

    // Calculate bar_width based on user's num_bars
    let bar_width = WIDTH / num_bars;

    // 3. Ask for sorting algorithm choice
    let mut algo_choice_str = String::new();
    let algo_choice: usize;
    loop {
        println!("Which sorting algorithm?");
        println!("  1: Bubble Sort");
        println!("  2: Selection Sort");
        println!("  3: Insertion Sort");
        println!("  4: Merge Sort");
        println!("  5: Quick Sort"); // New option
        io::stdin()
            .read_line(&mut algo_choice_str)
            .expect("Failed to read line");
        match algo_choice_str.trim().parse::<usize>() {
            Ok(1) => {
                algo_choice = 1;
                break;
            }
            Ok(2) => {
                algo_choice = 2;
                break;
            }
            Ok(3) => {
                algo_choice = 3;
                break;
            }
            Ok(4) => {
                algo_choice = 4;
                break;
            }
            Ok(5) => {
                // Handle new option
                algo_choice = 5;
                break;
            }
            _ => {
                println!("Invalid choice. Please enter 1, 2, 3, 4, or 5.");
                algo_choice_str.clear(); // Clear the buffer for next input
            }
        }
    }

    // --- Visualization Setup ---

    // 1. Initialize the array with random values
    let mut rng = rand::rng();
    let mut array: Vec<u32> = (0..num_bars)
        .map(|_| rng.random_range(1..=(num_bars as u32))) // Values from 1 to num_bars
        .collect();

    // 2. Create a minifb window
    let mut window = Window::new(
        "Sorting Algorithm Visualizer - Press ESC to exit", // Window title
        WIDTH,                                              // Window width
        HEIGHT,                                             // Window height
        WindowOptions::default(),                           // Default window options
    )
    .unwrap_or_else(|e| {
        // Handle error if window creation fails
        panic!("{}", e);
    });

    // Limit update rate to 60 FPS (optional, but good for smooth animation)
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // 3. Create the pixel buffer
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // 4. Initialize audio output (only if sound is enabled)
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // 5. Initial draw of the unsorted array
    draw_bars(&array, &mut buffer, None, None, None, bar_width);
    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

    // 6. Run the visualized sorting algorithm based on user choice
    match algo_choice {
        1 => {
            println!("Starting Bubble Sort visualization...");
            bubble::bubble_sort_visualized(
                &mut array,
                &mut window,
                &mut buffer,
                &sink,
                use_sound,
                bar_width,
                num_bars,
            );
        }
        2 => {
            println!("Starting Selection Sort visualization...");
            selection::selection_sort_visualized(
                &mut array,
                &mut window,
                &mut buffer,
                &sink,
                use_sound,
                bar_width,
                num_bars,
            );
        }
        3 => {
            println!("Starting Insertion Sort visualization...");
            insertion::insertion_sort_visualized(
                &mut array,
                &mut window,
                &mut buffer,
                &sink,
                use_sound,
                bar_width,
                num_bars,
            );
        }
        4 => {
            println!("Starting Merge Sort visualization...");
            sorting_algorithm_visualizer_rs::merge::merge_sort_visualized(
                &mut array,
                &mut window,
                &mut buffer,
                &sink,
                use_sound,
                bar_width,
                num_bars,
            );
        }
        5 => {
            // New Quick Sort case
            println!("Starting Quick Sort visualization...");
            quick::quick_sort_visualized(
                &mut array,
                &mut window,
                &mut buffer,
                &sink,
                use_sound,
                bar_width,
                num_bars,
            );
        }
        _ => { /* Should not happen due to loop, but good for completeness */ }
    }
    println!("Sorting visualization finished.");

    // 7. Keep the window open until ESC is pressed
    // Draw the final sorted state (all green)
    draw_bars(&array, &mut buffer, None, None, Some(0), bar_width);
    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Continue to update the window to keep it responsive
        // and allow user to close it with ESC.
        // No need to redraw bars as the sort is complete.
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
