# STM32 CLI tool

Allows configuring PHYsec algorithms on STM32 boards.

## Usage

### Main actions
- Configure PHYsec by hand
    `cargo r --release -- -R    # -R for resetting the board`
- Configure PHYsec using a configuration file
    `cargo r --release -- -c <config_file> -R`
- Monitor serial logging and telemetry data
    `cargo r --release -- -s -l -R`

### Configuration file

Look at `master.toml` and `slave.toml` files.

### csi_parse.py

Allow simple visualization of CSI files (plot or python list).

```bash
python3 csi_parse.py --plot csi_alice_sf9_15-meters.bin csi_bob_sf9_15-meters.bin

# Rebase both sample by substracting mean of the set 
python3 csi_parse.py --sm csi_alice_sf9_15-meters.bin csi_bob_sf9_15-meters.bin
```

## Implementation

The packets layout are defined by the C structs in `SubGHz_Phy/App/physec_config.h` and `SubGHz_Phy/App/physec_telemetry.h`.
They are implemented using struct encapsulation to have no memory overhead (no heap allocation).

The `generate-rust-bindings.sh` script is used to generate rust bindings for the headers using `bindgen`. Then the constants
are used within rust code base. We try to avoid using the structs layouts generated from bindings because it might require more
unsafe and less rust-idiomatic code.

The Rust code in `src/packets` is responsible for re-implementing the C structs, with serialization and deserialization methods. 

### Adding more configuration packets

1. Creates a C struct representing new packet for config or telemetry
    - assignate a new enum member for the packet type in C code (in `physec_config.h` or `physec_telemetry.h`)
    - Implement retrieving of the packet size in `physec_telemetry_get_size` (in `physec_telemetry.h`) or parsing procedure in `HAL_UART_RxCpltCallback` for config packets (in `subghz_phy_app.c`)
2. Generate rust bindings
3. Creates the Rust structure layout matching C struct:
    - for config: `src/packets/config/config_<packet_type>.rs`
    - for telemetry: `src/packets/telemetry/telemetry_<packet_type>.rs`
4. Implement `PHYsecPayload` trait, and `TelemetryLogging` trait additionally for telemetry packets
5. Add parsing of the payload into `PHYsecConfigPacket::from_bytes` and/or `TelemetryPacket::from_bytes` in main packets (`src/packets/config/mod.rs` and `src/packets/telemetry/mod.rs`)
    - (the `to_bytes` method will already handle your new payload thanks to the traits)
