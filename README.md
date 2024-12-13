# estop

## Overview
estop is a proof-of-concept project for monitoring Elasticsearch (ES). This project is written entirely in Rust and aims to provide efficient and reliable monitoring capabilities for Elasticsearch environments.

## Features
- Real-time monitoring of Elasticsearch clusters
- Performance metrics collection
- Easy setup and configuration

## Installation
To install and run estop, you need to have Rust installed on your system. If you don't have Rust installed, you can get it from [here](https://www.rust-lang.org/).

Clone the repository:
```sh
git clone https://github.com/Anaethelion/estop.git
cd estop
```

Build the project:
```sh
cargo build --release
```

## Usage
To use estop, run the following command:
```sh
./target/release/estop
```

## Configuration
Configuration for estop can be done through the `ELASTICSEARCH_URL` env var.

## Contributing
Contributions are welcome! Please fork the repository and create a pull request with your changes.

## License
This project is licensed under the Apache 2.0 License. See the [LICENSE](LICENSE) file for details.
