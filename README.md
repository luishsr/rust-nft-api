Rust NFT API
============

Purpose
-------

Rust NFT API is a simple RESTful API developed in Rust that offers functionalities for creating, retrieving, and listing Non-Fungible Tokens (NFTs) on the Ethereum blockchain, along with managing their off-chain metadata.

Requirements
------------

Before setting up the Rust NFT API, ensure you have the following components ready:

1.  Ethereum Node: An Ethereum test node, such as Ganache, for local blockchain simulation.
2.  IPFS: IPFS Desktop or similar for off-chain data storage, to test locally.
3.  Smart Contract Deployment Tool: Remix IDE or another tool for deploying the Smart Contract to the Ethereum blockchain.

Setup
-----

Follow these steps to set up and run the Rust NFT API:

### 1\. Start Ganache

Launch Ganache to initiate a local Ethereum blockchain node.

### 2\. Deploy the Smart Contract

Using Remix IDE or your preferred tool, deploy the `MyNFT.sol` smart contract located in the `contract` folder of this project.

### 3\. Start the IPFS Node

Run your IPFS node to handle off-chain metadata storage. If using IPFS Desktop, simply open the application.

### 4\. Configure Environment Variables

Edit the `.env` file in the project's root directory to set the necessary environment variables:

envCopy code

`ETH_NODE_URL=http://localhost:8545
CONTRACT_ABI_PATH=./MyNFT.json
MOCK_PRIVATE_KEY=69440d76b64f2418574043fbd79f3d4f56c293290c3056fc18b62b12013db7e7 # This is a placeholder for testing

TEST_OWNER_ADDRESS=<Your Token Owner account address from Ganache>
TEST_TOKEN_ID=1 # Default to 1
TEST_CONTRACT_ADDRESS=<Your deployed Smart Contract address>`

### 5\. Build the Project

Compile the project using Cargo:

bashCopy code

`cargo build`

### 6\. Run Tests

Execute the project's test suite to verify the setup:

bashCopy code

`cargo test`

If any tests fail, revisit the previous setup steps to troubleshoot.

### 7\. Run the Application

Start the Rust NFT API server by providing the deployed smart contract's address as a command-line argument (matching the `TEST_CONTRACT_ADDRESS` in `.env`):

bashCopy code

`cargo run -- <Deployed_Smart_Contract_Address>`

### 8\. Interact with the API

Access the Swagger UI at <http://localhost:3010/swagger-ui> to interact with the API through a graphical interface or use your preferred tool to call the API endpoints directly.

* * * * *

Additional Notes
----------------

-   Ensure Ganache and the IPFS node are running before starting the Rust NFT API server.
-   The `MOCK_PRIVATE_KEY` provided in the `.env` example is for demonstration purposes only. Never use real private keys in your development environment or commit them to version control.
-   For any issues or contributions, feel free to open an issue or a pull request in the repository.
