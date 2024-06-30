# solana-nfo-datebase

This program is designed to query information on the Solana blockchain using CLI commands.

## Why is this necessary?

Accessing information on the Solana network instantly via commands ensures immediate availability and usability of crucial blockchain data. This capability is essential for developers, researchers, and users who need real-time data to make informed decisions and enhance their applications.

### Development Roadmap

In the initial stages, the program will be built using Rust, leveraging its performance and safety features to ensure a robust and efficient backend. Once the core functionality is established, a user-friendly UI will be developed using WebAssembly, making the tool accessible to a broader audience.

### Future Enhancements

As the project evolves, we plan to continuously add new features to increase its utility and reach. By expanding the program's capabilities and refining its interface, we aim to make it a comprehensive tool for interacting with the Solana blockchain.

By providing seamless access to Solana's blockchain data through both CLI and web interfaces, we aspire to enhance the user experience and drive greater adoption of Solana's ecosystem.

#### Command line

- cargo run -- --validators
- cargo run -- --top-validators
- cargo run -- --stakes (edit)

#### Coming

- cargo run -- --token-info <MINT_TOKEN_ADDRESS>

#### Task

- A testable structure,
- Modular structure,
- Token creation,
- Creating a staking account,
- Wallet integration,
- Blink integration
