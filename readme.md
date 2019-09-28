# sim1h

A simulator/emulator for [lib3h](https://github.com/holochain/lib3h).

## Why?

rrDHT is agent centric by design.

This makes certain things difficult debug.

There is no "global store" to audit to see what has been published.

This is great for production but problematic for:

- Conductor devs verifying the crossover between wasm and network workflows
- Zome devs reviewing how data is published and recieved across nodes
- Hardware (e.g. holoport) devs wanting to test OS level concerns decoupled from DHT network health etc.

It's also nice to have at least one other implementation of `Lib3hProtocol`.  
One small step towards this being more "protocol" than "implementation detail".

## How?

This is a sandbox network implementation.

It implements the same `Lib3hProtocol` messages as `lib3h`.

It reuses the p2p logic for direct send/receive messaging from `lib3h`.

That's about it.

Everything published to "the network" gets dumped into a key/value database.

The CAS address is the key.  
Whatever the conductor passes us is the value, stored verbatim.

Devs can open up the database to inspect what happened "globally".

## What?

Currently wrapping dynamodb from AWS for the key/value store.

- has a cloud option to support nodes in different locations
- has a local option for local development/CI/testing
- has a 25GB free tier with no monthly fees
- it's pretty popular and does what you'd expect for basic key/value stuff

Calling this the "bezos bunker DHT" (bbDHT).

## Usage with holochain-rust

Currently supported on the [sim1h-integration branch](https://github.com/holochain/holochain-rust/tree/sim1h-integration). That branch supports a new `sim1h` network type, as well as a nix command to run a local dynamodb instance.

In your conductor config, use the following for the `network` config section:

```
[network]
type = 'sim1h'
dynamo_url = 'http://localhost:8000' # URL of running dynamodb instance
```

You can run a local dynamodb instance at port 8000 by entering a nix-shell and running:

    dynamodb

If you want to expose your local dynamodb instance over the internet, we suggest using a tunneling service like [ngrok](https://ngrok.com/) to map a public URL to your local port. Then, your friends can use that public URL as their `dynamo_url` instead of localhost.

That's it!
