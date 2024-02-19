use std::env;
use std::error::Error;
use web3::contract::Contract;
use web3::transports::Http;
use web3::{ethabi, Web3};

pub struct Web3Client {
    pub web3: Web3<Http>,
    pub contract: Contract<Http>,
}

impl Web3Client {
    pub fn new(contract_address: &str) -> Result<Self, Box<dyn Error>> {
        let http = Http::new(&env::var("ETH_NODE_URL")?)?;
        let web3 = Web3::new(http);

        let contract_abi_path = env::var("CONTRACT_ABI_PATH")?;
        let contract_abi_file = std::fs::File::open(contract_abi_path)?;
        let contract_abi: ethabi::Contract = serde_json::from_reader(contract_abi_file)?;

        let contract = Contract::new(web3.eth(), contract_address.parse()?, contract_abi);

        Ok(Web3Client { web3, contract })
    }
}
