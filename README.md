# Webcam Entropy Generator

The **Webcam Entropy Generator** is a Rust-based application designed to generate entropy by measuring thermal noise in local webcam sensors. It utilizes the `nokhwa` library to interface with the camera and capture frames. This entropy is then converted into a random large unsigned integer, which can be used for cryptographic purposes.

## Understanding Thermal Noise in Camera Sensors

Thermal noise, also known as dark current noise, is a type of noise that affects the performance of camera sensors. It arises from the random motion of electrons within the sensor's semiconductor material, which is influenced by temperature. As the temperature increases, the kinetic energy of the electrons also increases, leading to more frequent and random electron movements.

### Causes of Thermal Noise

1. **Electron Movement**: Atoms in the semiconductor lattice vibrate due to thermal energy, causing electrons to be knocked loose and generate a small current even in the absence of light.
2. **Temperature Dependence**: The level of thermal noise is directly proportional to the temperature of the sensor. Higher temperatures result in higher noise levels.

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