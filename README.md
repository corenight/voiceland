# Voiceland

Real time communication platform over QUIC written in Rust.

## What is Voiceland?

Voiceland is a fast, simple and reliable real time communication platform that relies on QUIC for transport layer. Is still in an early development, so there's a lot of things to do.

The actual goal is to create a simple (really simple) server that's able to communicate _n_ connections that can share audio and video with Opus and AV1.

## Development status

Actually server can share messages from a client to the rest of the connections without problems, and the client has a messy and basic interface to send and receive raw audio from Cpal (please ignore the client code, is a disaster but is just a weird test).

Sounds like a Coca-Cola can.

_Gátomo - GNU Affero General Public License v.3.0 License_
