# safe-relay-rs

This repository contains a Rust-based backend service that allows you to create Gnosis Safe instances using the CREATE2 opcode, as well as relaying transactions through the service.

## Table of Contents

- [Introduction](#introduction)
- [Prerequisites](#prerequisites)
- [Setup](#setup)

## Introduction

This project aims to simplify the process of creating Gnosis Safe instances and relaying transactions through a Rust backend service. By leveraging the CREATE2 opcode, it ensures deterministic deployment addresses for the Gnosis Safe contracts, providing a predictable and efficient way to manage digital assets on the Ethereum blockchain.

## Prerequisites

Before you begin, make sure you have the following installed:

- Rust programming language
- Cargo (Rust package manager)
- Node.js and npm (for additional tooling)

## Setup

To set up the project, follow these steps:

1. Clone the repository:

```bash
git clone https://github.com/safebox/safe-relay-rs.git
```

2. Install dependencies:
```bash
cargo build
```

3. Start the rust Server:
```bash
cargo run
```
