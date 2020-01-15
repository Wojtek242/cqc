CQC
===

[![Latest version](https://img.shields.io/crates/v/cqc.svg)](https://crates.io/crates/cqc)
[![Documentation](https://docs.rs/cqc/badge.svg)](https://docs.rs/cqc)
![License](https://img.shields.io/crates/l/cqc.svg)

A sans-io Rust implementation of the [CQC
interface](https://softwarequtech.github.io/CQC-Python/interface.html).

The Classical-Quantum Combiner (CQC) interface is used to program quantum
networking nodes to create, transmit, and manipulate qubits.

The CQC interface will be used to interact with the Dutch demonstration
network, currently under development at QuTech in the Netherlands. At present,
the CQC interface is supported only by the quantum network simulator
[Simulaqron](http://www.simulaqron.org/).

- [Documentation](https://docs.rs/cqc)
- [SimulaQron](http://www.simulaqron.org/)
- [SimulaQron Manual](https://softwarequtech.github.io/SimulaQron/html/GettingStarted.html)

## Principle of Operation

This library provides two functions:

1) Build valid CQC packets.

2) Encode/decode to/from binary format.  It is left to the user to decide how
best to fit I/O in their framework.

### Building Packets

This crate offers two ways of building packets

1) Manually - one can manually build packets using the header definitions and
documentation provided in the `hdr` module.

2) Using the `builder` module - the builder module provides a simple API for
generating CQC packets.  It should be used in conjunction with the CQC
interface documentation in the `hdr` module.

### Encoding/decoding packets

All headers in the `hdr` module implement `serde`'s `Serialize` and
`Deserialize` traits which mean they can be directly used as input to
`bincode`.  The `Encoder` and `Decoder` impls provide an example.

The `builder` module returns a `Request` struct which implements `Serialize`
which can be used with `bincode`.

The library provides a `Response` struct which implements `Deserialize` and can
be used to deserialize any response from the SimulaQron server.

## CQC in action

The following example will create a qubit on one node and send it to another
node.  Before running the example below start up the SimulaQron nodes with
`$NETSIM/run/startAll.sh --nrnodes 2`.

```rust
extern crate bincode;
extern crate cqc;

use cqc::builder;
use cqc::hdr;
use std::net;

fn main() {
    // Initialise local node `localhost:8803`.
    let hostname = String::from("localhost");
    let local_port: u16 = 8803;

    // Set up remote node `127.0.0.1:8804`.
    let remote_host: u32 = u32::from(net::Ipv4Addr::new(127, 0, 0, 1));
    let remote_port: u16 = 8804;

    // Initialise application state with ID 10.
    let app_id: u16 = 10;
    let builder = builder::Builder::new(app_id);
    let mut coder = bincode::config();
    coder.big_endian();

    // Create, and send a qubit from `localhost:8803` to `localhost:8804`.
    {
        // Open connection to local node.
        let stream = net::TcpStream::connect((hostname.as_str(), local_port))
            .expect("Connect failed");

        // Create the qubit.
        let request = builder.cmd_new(0, hdr::CmdOpt::empty());
        coder.serialize_into(&stream, &request).expect(
            "Sending failed",
        );

        // Wait for confirmation of creation.
        let response: cqc::Response = coder.deserialize_from(&stream).expect("Receive failed");

        // Read the created qubit ID.
        let note = response.notify.get_notify_hdr();
        let qubit_id = note.qubit_id;

        // Send the qubit to the remote node.
        let request = builder.cmd_send(
            qubit_id,
            *hdr::CmdOpt::empty().set_notify(),
            builder::RemoteId {
                remote_app_id: app_id,
                remote_node: remote_host,
                remote_port: remote_port,
            },
        );
        coder.serialize_into(&stream, &request).expect(
            "Sending failed",
        );

        // Wait for confirmation.
        let response: cqc::Response = coder.deserialize_from(&stream).expect("Receive failed");
        assert!(response.cqc_hdr.msg_type.is_done(), "Unexpected response");
    }

    // Receive the qubit on the remote node, `localhost:8804`.
    {
        // Open connection to local node.
        let stream = net::TcpStream::connect((hostname.as_str(), remote_port))
            .expect("Connect failed");

        // Send a request to receive a qubit.
        let request = builder.cmd_recv(0, hdr::CmdOpt::empty());
        coder.serialize_into(&stream, &request).expect(
            "Sending failed",
        );

        // Receive a response.
        let response: cqc::Response = coder.deserialize_from(&stream).expect("Receive failed");
        assert!(response.cqc_hdr.msg_type.is_recv(), "Unexpected response");
        let note = response.notify.get_notify_hdr();
        let qubit_id = note.qubit_id;

        println!("Received qubit ID: {}", qubit_id);
    }
}
```

## Design goals

The following goals drive the design of the `cqc` crate:

- The user should be able to create any valid packet

  This goal is achieved by having correct struct definitions for the different
  CQC headers.

- It should be difficult, though preferably impossible, to create invalid
  packets

  The second goal is achieved by using Rust's typing system as much as
  possible, especially enums for fields with only a small set of possible
  values.  Furthermore a `builder` module is provided which guarantees correct
  CQC packets.

- Decoding should raise errors if unrecognised values are detected

  This is achieved through a combination of type definitions and
  deserialization implementations.

- No assumption about the user's run-time should be made

  The library is sans-io and only provides a very plain encoder and decoder as
  an example.  The intention is that the user builds packets using the `cqc`
  library, but I/O is their responsibility.  The `Serialize` and `Deserialize`
  traits are implemented so that the user can simply use `bincode` for
  encode/decode.

## Limitations

- Factory and Sequence Headers are not currently fully supported.
- Encode/decode is implemented for client-side operations.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
cqc = "0.5"
```

and this to your source file:

```rust
extern crate cqc;
```
