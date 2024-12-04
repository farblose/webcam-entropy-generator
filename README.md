# Webcam Entropy Generator

The **Webcam Entropy Generator** is a Rust-based application designed to generate entropy by capturing frames from a webcam. It utilizes the `nokhwa` library to interface with the camera, capturing images and calculating entropy through pixel-level operations. This entropy is then converted into a large unsigned integer, which can be used for cryptographic or random number generation purposes.

## Features

- **Camera Integration**: Uses the `nokhwa` library to access and control webcam devices.
- **Entropy Calculation**: Captures multiple frames and calculates entropy by performing XOR operations on pixel data.
- **Binary to Decimal Conversion**: Converts the generated binary entropy string into a `BigUint` for further use.
- **Configurable Output Size**: Allows users to specify the desired size of the output number in bits via command line arguments.

## Installation
```bash
git clone https://github.com/farblose/webcam-entropy-generator.git && \
cd webcam-entropy-generator && \
cargo build -r && \
cd target/release && \
./weg [size of number in bits]
```

Replace `[size of number in bits]` with the desired bit length of the output number.

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](LICENSE) file for details.