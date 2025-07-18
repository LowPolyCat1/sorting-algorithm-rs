# Sorting Algorithm Visualizer

[![License](https://img.shields.io/github/license/lowpolycat1/sorting-algorithm-visualization-rs.svg?style=flat-square)](LICENSE.txt)
&nbsp;
[![Contributors](https://img.shields.io/github/contributors/lowpolycat1/sorting-algorithm-visualization-rs.svg?style=flat-square)](https://GitHub.com/lowpolycat1/sorting-algorithm-rs/graphs/contributors/)
&nbsp;
[![Crate Versions](https://img.shields.io/badge/Crates-Updated-663399.svg?style=flat-square)](https://github.com/lowpolycat1/sorting-algorithm-visualization-rs)
&nbsp;
[![minifb](https://img.shields.io/crates/v/minifb.svg)](https://crates.io/crates/minifb)
&nbsp;
[![rodio](https://img.shields.io/crates/v/rodio.svg)](https://crates.io/crates/rodio)

This project visualizes various sorting algorithms using a graphical interface and audio feedback. Users can interactively see sorting steps for educational purposes.

## Built With

* [minifb crate](https://github.com/takm-oss/minifb) - Window/context for rendering frames
* [rodio crate](https://github.com/Geal/rodio) - Audio playback for visualization feedback
* Pure Rust - All sorting algorithms written natively without external dependencies

## Getting Started

### Prerequisites

* Rust toolchain (>=1.36.0)
* Cargo package manager
* Ensure audio device works (for sound visualization)

### Installation

1. Clone the repository

    ```bash
    git clone https://github.com/lowpolycat1/sorting-algorithm-rs.git
    cd sorting-algorithm-rs
    ```

2. Install dependencies (should be handled by Cargo)

   ```bash
   cargo build
   ```

3. Run the program

   ```bash
   cargo run
   ```

## Usage

The visualizer allows selecting from:

1. Bubble Sort
2. Selection Sort
3. Insertion Sort
4. Merge Sort
5. Quick Sort

**Sound Effects:**
Enable/disable audio feedback during sorting animations.

**Bar Selection:**
Configure number of bars (50-400 pixels) to fit your screen resolution.

<!--## Contributing

See [CONTRIBUTING.md](.github/CONTRIBUTING.md) guidelines.
<!-- ## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) guidelines.

## License

MIT license (see [LICENSE.txt](LICENSE.txt) file).-->

## Images

### Bubble Sort

![Bubble Sort](/showcase/bubble)

### Selection Sort

![Selection Sort](/showcase/selection)

### Insertion Sort

![Insertion Sort](/showcase/insertion.png)

### Merge Sort

![Merge Sort](/showcase/merge.png)

### Quick Sort

![Quick Sort](/showcase/quick.png)

### Finished Sorting

![Finished Sorting](/showcase/finished.png)
