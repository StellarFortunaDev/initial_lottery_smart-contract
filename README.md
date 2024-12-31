# Stellar Fortuna Lottery Smart Contract

A decentralized lottery system built on Stellar's Soroban smart contract platform.

## Description

This smart contract implements a basic lottery system where:
- Players can join the lottery
- Only the owner can draw a winner
- Random winner selection is handled securely
- Prevents duplicate entries and maintains lottery state

## Prerequisites

- Rust (latest stable version)
- Soroban CLI
- Stellar Development Environment

## Installation
git clone https://github.com/StellarFortunaDev/initial_lottery_smart-contract.git

cd initial_lottery_smart-contract

## Install dependencies
cargo build

## Building the Contract
cargo build --target wasm32-unknown-unknown --release

soroban contract optimize --wasm target/wasm32-unknown-unknown/release/lottery.wasm 

## Deployment
soroban contract deploy \
--wasm target/wasm32-unknown-unknown/release/lottery.wasm \
--source YOUR_SECRET_KEY \
--network testnet

##Initialize the contract (after deployment you'll get a contract ID)
soroban contract invoke \
--id CONTRACT_ID \
--source YOUR_SECRET_KEY \
--network testnet \
-- init \
--owner YOUR_PUBLIC_KEY

##Join the lottery (for players):
soroban contract invoke \
--id CONTRACT_ID \
--source PLAYER_SECRET_KEY \
--network testnet \
-- join \
--player PLAYER_PUBLIC_KEY

##Check players:
soroban contract invoke \
--id CONTRACT_ID \
--source YOUR_SECRET_KEY \
--network testnet \
-- get_players

##Draw the winner (only owner can do this):
soroban contract invoke \
--id CONTRACT_ID \
--source OWNER_SECRET_KEY \
--network testnet \
-- draw \
--owner OWNER_PUBLIC_KEY

Contributing

Fork the repository \
Create your feature branch (git checkout -b feature/amazing-feature) \
Commit your changes (git commit -m 'Add some amazing feature') \
Push to the branch (git push origin feature/amazing-feature) \
Open a Pull Request \

License
This project is licensed under the MIT License - see the LICENSE file for details
Contact

Email: contact@stellarfortuna.com
Telegram: @stellarfortunasocial

