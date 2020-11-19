[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v2.0%20adopted-ff69b4.svg)](code_of_conduct.md)


# COVID-19 Self-reporting with Privacy
Privacy preserving voluntary COVID-19 self-reporting platform for contact tracing. 

## Overview & Motivation
Social contact tracing based on mobile phone data has been used to track and mitigate the spread of COVID-19[[1]](https://www.nature.com/articles/d41586-020-00740-y). However, this is a significant privacy risk, and sharing these data may disproportionately affect at-risk populations, who could be subject to discrimination and targeting. In certain countries, obtaining this data en masse is not legally viable. 

We propose a privacy-preserving, voluntary self-reporting system for sharing detailed location data amongst individuals and organizations. Users will be able to encrypt and share complete location history, and their current status (positive, negative, unknown). Users will be able to update their status if it changes. This system will compute on shared, aggregate data and return location-based social contact analytics. 

This has been tested with up to 4 million data points, representing location data for 1000+ people over a 14-day period.
See the [test report](docs/test_report.md) for more performance information.

## Core components

### Location History data (from Google Location Services via Google Takeout, or other services)

Any user who has Location Services active with Google is able to obtain a JSON format file of their location history. 
They are also able to edit this file manually to remove any unwanted or sensitive locations (i.e., a home address). 

Obtaining location history via Google is one way to get location data, but this project aims to be generalized. For this reason, 
the actual input data is merely a set of (latitude, longitude, time point). Any service which can obtain such data can transform it to a valid
format and use it as input data. 

### A Privacy-preserving, blockchain-based computation service

Private computation is a term for performing tasks on data that is never viewed in plaintext. Our system uses private computation to generate individual and global analytics. 

We implement this using distributed, private computation based on Secret Network. This allows us to have a robust, decentralized
infrastructure, and create the application logic on top, with separation of responsibilities between layers.

In this repository we implement a secret contract that allows for two core functionalities:

- Identify users who have been in close proximity with individuals who have tested positive
- Analyse and create clusters from user data, and output those results to a map without revealing original data to anyone

Future work may include other features, such as:

- Add noise to user locations, and then output that data to a map without revealing the original data to anyone, including application developers or server owners 

## System Architecture

The core of this repo is the blockchain infrastructure, and secret contract application
that performs the handling of data and business logic. We aim not to provide a full implementation 
including a UI, but rather a reference so that others may use this as an example, integrate, and build on 
this infrastructure. 

### Blockchain Infrastructure

The Secret Network is a decentralized network of computers (which we call "secret nodes") that use hardware-based and software-based privacy technologies to enable secure computation. On top of this network, developers can build Secret Apps - unstoppable, permissionless applications that can utilize encrypted data without ever exposing the data itself, even to the nodes in the network performing computations.

The advantage of using the Secret Network to not only handle encryption, and private computation is that we end up with a decentralized system, and a set of mature developer tools and documentation,
making getting easier.

### Secret Contracts

Secret Contracts are based on CosmWasm which is implemented on various Cosmos SDK blockchains. Secret Contracts allow a developer to 
create applications on the blockchain infrastructure without having to care about the underlying infrastructure, and use standardized tools for development, testing, and deployment.   
CosmWasm smart contracts are written in the Rust language. You can learn more about Secret Contracts [here](https://build.scrt.network/dev/secret-contracts.html) 

### UI with SecretJS

Since we are building on the Secret Network infrastructure, we can use [secretjs](https://www.npmjs.com/package/secretjs), written in Javascript
to easily interface with our Secret Contract. This allows the developer to focus on creating the user experience, 
rather than creating custom logic to interface with the privacy-preserving application

## LICENSE

The code in this repository is released under the [MIT License](contract/LICENSE).
