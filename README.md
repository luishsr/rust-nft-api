`# Rust NFT API

## Purpose

Rust NFT API is a simple RESTful API built with Rust that enables creating, retrieving, and listing Non-Fungible Tokens (NFTs) on the Ethereum blockchain, along with managing their off-chain metadata.

## Requirements

To use Rust NFT API, you'll need the following:

1. An Ethereum node for testing (e.g., [Ganache](https://www.trufflesuite.com/ganache))
2. [IPFS Desktop](https://github.com/ipfs/ipfs-desktop) for local IPFS node simulation
3. [Remix IDE](https://remix.ethereum.org/) or another tool for deploying the Smart Contract

## Setup

Follow these steps to set up and run Rust NFT API:

### 1. Start Ganache

Launch Ganache to create a local Ethereum blockchain instance.

### 2. Deploy the Smart Contract

Use Remix IDE or your preferred tool to deploy the Smart Contract using the `MyNFT.sol` file located in the `contract` folder.

### 3. Start the IPFS Node

Open IPFS Desktop or your preferred IPFS client to start an IPFS node.

### 4. Configure Environment Variables

Edit the `.env` file in the project root folder and set the following environment variables:

```env
ETH_NODE_URL=http://localhost:8545
CONTRACT_ABI_PATH=./MyNFT.json
MOCK_PRIVATE_KEY=69440d76b64f2418574043fbd79f3d4f56c293290c3056fc18b62b12013db7e7 # This is a fake key for testing

TEST_OWNER_ADDRESS=<Your Token Owner account address from Ganache>
TEST_TOKEN_ID=1 # Keep it as 1 by default
TEST_CONTRACT_ADDRESS=<Your deployed Smart Contract address> `

### 5\. Build the Project

Run the following command to build the project:

bashCopy code

`cargo build`

### 6\. Run Tests

Ensure your setup is correct by running:

bashCopy code

`cargo test`

If the tests fail, double-check your setup based on the previous steps.

### 7\. Run the Application

Once the tests pass, you can run the application by providing the deployed smart contract address as a command-line argument (the same address specified in the `TEST_CONTRACT_ADDRESS` environment variable). This allows for easy switching of smart contracts without needing to rebuild.

### 8\. Interact with the API

After starting the application, you can interact with the API by navigating to:

bashCopy code

`http://localhost:3010/swagger-ui`

Alternatively, you can use your preferred tool to call the API endpoints.

Contributing
------------

Contributions are welcome! Please feel free to submit pull requests or open issues to discuss proposed changes or enhancements.

License
-------

This project is licensed under the MIT License - see the [LICENSE](https://chat.openai.com/c/LICENSE) file for details.
