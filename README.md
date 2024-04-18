# greenhouse-controller

An automated greenhouse controller system built with Rust and Raspberry Pi.

## Features

- Monitor environmental conditions (temperature, humidity, light) in a greenhouse
- Automatically control greenhouse equipment (ventilation, heating, lighting) based on optimal conditions for plants
- Provide a user interface to visualize data and configure greenhouse settings

## Hardware Requirements

- Raspberry Pi
- Temperature, humidity, and light sensors
- Relays to control greenhouse equipment (fans, heating, lighting)

## Setup

1. Clone the repository
2. Install Rust and required dependencies
3. Configure the `config.toml` file with your database URL and server port
4. Run the SQL script `database.sql` to set up the database
5. Build and run the project using `cargo run`

## Usage

- Access the web interface at `http://localhost:<server_port>` to view real-time environmental data and configure settings
- The system will automatically control the greenhouse equipment based on the defined optimal ranges

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License.
