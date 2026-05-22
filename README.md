# Event Certificate Smart Contract

## Description

This project is a Soroban smart contract built on the Stellar network designed to securely allocate, manage, and verify digital certificates for event participants. By leveraging blockchain technology, this contract ensures that credentials issued by event organizers are immutable, publicly verifiable, and protected against duplication.

## Features

Based on the `EventCertificateContract` implementation, the project currently supports:

* **Secure Initialization:** Set up the contract with a designated Admin (organizer) and a specific Event Name. This function is locked after the first use to prevent tampering.
* **Admin-Controlled Issuance:** Only the verified Admin address can authorize and issue certificates to attendees, ensuring complete control over credential distribution.
* **Duplicate Prevention:** The contract automatically checks persistent storage to ensure an attendee cannot receive more than one certificate for the same event.
* **Public Verification:** Anyone can query the contract using the `has_certificate` function to instantly verify if a specific address holds a valid certificate.
* **Event Identity:** Easily retrieve the associated event name directly from the blockchain state.

## Contract

**Live Testnet Contract:** [View on Stellar Expert](https://stellar.expert/explorer/testnet/contract/CAVDISEUCDGVDQW3KZWECR4HP3BLMIX54GSEX2TNTZOWCRQBO6SO4Q7D)

### Contract Screenshot

## Project Vision

The goal of this project is to provide a transparent, trustless, and decentralized method for educational institutions, community builders, and event organizers to issue credentials. By moving away from easily forgeable paper or standard digital certificates, we aim to make credential verification seamless and permanent.

## Future Scope

* **Batch Issuance:** Optimize the `issue` function to accept a list of addresses, allowing organizers to distribute certificates to hundreds of attendees in a single transaction.
* **Multi-Event Support:** Upgrade the contract architecture to manage multiple events and discrete certificate pools under a single deployed instance.
* **Rich Metadata:** Introduce functionality to append specific metadata to each certificate (e.g., issue date, achievement level, participant name).
* **Frontend Integration:** Build a decentralized application (dApp) interface for users to easily view their earned certificates and for admins to manage issuances without using the CLI.

## Profile

**Phan Quốc Bảo (Black Jack)**

* **Role:** Information Technology Student (HCMUS)
* **Technical Skills:** C++, Data Structures & Algorithms, Embedded Systems (ESP32, IoT), and PCB Design.
* **Hardware & Engineering:** SolidWorks, Custom 3D Printing (Voron/CoreXY), and Robotics hardware integration.