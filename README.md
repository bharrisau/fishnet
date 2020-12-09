fishnet: distributed Stockfish analysis for lichess.org
=======================================================

[![crates.io](https://img.shields.io/crates/v/fishnet.svg)](https://crates.io/crates/fishnet)
[![Build](https://github.com/niklasf/fishnet/workflows/Build/badge.svg)](https://github.com/niklasf/fishnet/actions?query=workflow%3ABuild)
[![Docker](https://img.shields.io/docker/cloud/build/niklasf/fishnet)](https://hub.docker.com/r/niklasf/fishnet)

Installation
------------

1. Request your personal fishnet key: https://lichess.org/get-fishnet

2. Install and run the fishnet client.

   **Download standalone binary**

   Select the binary for your platform
   [from the latest release](https://github.com/niklasf/fishnet/releases)
   and run it.

   ```sh
   # After download:
   chmod +x fishnet-x86_64-unknown-linux-gnu
   ./fishnet-x86_64-unknown-linux-gnu --auto-update
   ```

   Other useful commands:

   ```sh
   ./fishnet-x86_64-unknown-linux-gnu configure              # Rerun config dialog
   ./fishnet-x86_64-unknown-linux-gnu systemd --auto-update  # Print a .service file
   ./fishnet-x86_64-unknown-linux-gnu --help                 # List commands and options
   ```

   **From source**

   Assuming you have [a recent Rust toolchain](https://rustup.rs/) installed:

   ```sh
   git clone --recursive https://github.com/niklasf/fishnet.git
   cd fishnet
   cargo run --release --
   ```

   **Docker**

   ```sh
   docker run -it -e KEY=abcdef niklasf/fishnet:latest
   ```

FAQ
---

### Which engine does fishnet use?

fishnet uses [Stockfish 12](https://github.com/official-stockfish/Stockfish)
(hence the name) and [a fork of Stockfish with multi-variant support](https://github.com/ddugovic/Stockfish).

[Precompiled builds](https://github.com/niklasf/fishnet-assets)
for various CPU models come bundled with fishnet. To get another architecture
included, all we need is a reproducible build process (so everyone can verify
that the compiled binary matches the source).

### What are the requirements?

* Available for:
  | CPU | 64-bit Intel and AMD | ARMv8 |
  | --- | --- | --- |
  | **Linux** | `x86_64-unknown-linux-gnu` | `aarch64-unknown-linux-gnu` |
  | **Windows** | `x86_64-pc-windows-msvc.exe` | |
  | **macOS** | `x86_64-apple-darwin` | |
* An operating system from around 2016 or later
* Will max out the configured number of CPU cores
* Uses about 64 MiB RAM per CPU core
* A small amount of disk space
* Low-bandwidth network communication with Lichess servers
  (only outgoing HTTP requests, so probably no firewall configuration
  required)

### Is my CPU fast enough?

Almost all processors will be able to meet the requirement of ~2 meganodes in
6 seconds. Clients on the faster end will automatically be assigned
analysis jobs that have humans waiting for the result (the user queue, as
opposed to the system queue for slower clients).

### What happens if I stop my client?

Feel free to turn your client on and off at any time. By default, the client
will try to finish any batches it has already started. On immediate shutdown,
the client tries to inform Lichess that batches should be reassigned.
If even that fails, Lichess will reassign the batches after a timeout.

### Will fishnet use my GPU?

No, Stockfish is a classical alpha-beta engine. The neural network evaluation
of Stockfish NNUE works efficiently on CPUs.

### Is fishnet secure?

To the best of our knowledge. However you implicitly trust the authors and the
GitHub infrastructure when running with `--auto-update`.

You can mitigate this by running fishnet as an unprivileged user.

Stockfish builds are reproducible, so you can verify
that the distributed binaries match the source.

### Is there a leaderboard of contributors?

No, sorry, not publically. It would incentivize gaming the metrics.

### Can I autoscale fishnet in the cloud?

There is currently no ready-made solution, but
[an API for monitoring the job queue status](https://github.com/niklasf/fishnet/blob/master/doc/protocol.md#status)
is provided.

Protocol
--------

![Sequence diagram](https://raw.githubusercontent.com/niklasf/fishnet/master/doc/sequence-diagram.png)

See [protocol.md](https://github.com/niklasf/fishnet/blob/master/doc/protocol.md) for details.

License
-------

fishnet is licensed under the GPLv3+. See LICENSE.txt for the full
license text.
