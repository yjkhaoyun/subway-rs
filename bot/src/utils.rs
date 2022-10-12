
use ethers::prelude::*;
use eyre::Result;

abigen!(uniswapV2Pair, "src/abi/IUniswapV2Pair.json");

/// Required Environment Variables
pub static ENVIRONMENT_VARIABLES: Vec<&'static str> = vec![
    "RPC_URL",
    "RPC_URL_WSS",
    "PRIVATE_KEY",
    "FLASHBOTS_AUTH_KEY",
    "SANDWICH_CONTRACT",
];

/// Read environment variables
pub fn read_env_vars() -> Result<Vec<(String, String)>> {
    let mut env_vars = Vec::new();
    for key in ENVIRONMENT_VARIABLES {
        // Read environment variable
        let value = std::env::var(key).map_err(|_| eyre::eyre!("Required environment variable \"{}\" not set", key))?;
        env_vars.push((key.to_string(), value));
    }
    Ok(env_vars)
}

/// Returns the Uniswap V2 Pair Contract Address
pub fn get_univ2_address() -> Result<Address> {
    Address::from_str("0x7a250d5630b4cf539739df2c5dacb4c659f2488d").map_err(|_| eyre::eyre!("Invalid address"))
}

/// Returns the configured Sandwich Contract Address
pub fn get_sandwich_contract_address() -> Result<Address> {
    let addr = std::env::var("SANDWICH_CONTRACT").map_err(|_| eyre::eyre!("Required environment variable \"SANDWICH_CONTRACT\" not set"))?;
    Address::from_str(addr).map_err(|_| eyre::eyre!("Invalid address \"{}\"", addr))
}

/// Returns the WETH Contract Address
pub fn get_weth_address() -> Result<Address> {
    Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2").map_err(|_| eyre::eyre!("Invalid address"))
}

/// Returns the usdc Contract Address
pub fn get_usdc_address() -> Result<Address> {
    Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").map_err(|_| eyre::eyre!("Invalid address"))
}

/// Return a Provider for the given URL
pub fn get_http_provider() -> Result<Provider<Http>> {
    let url = std::env::var("RPC_URL").map_err(|_| eyre::eyre!("Required environment variable \"RPC_URL\" not set"))?;
    Provider::<Http>::try_from(url).map_err(|_| eyre::eyre!("Invalid RPC URL"))
}

/// Return a Provider for the given Websocket URL
pub fn get_ws_provider() -> Result<Provider<Ws>> {
    let url = std::env::var("RPC_URL_WSS").map_err(|_| eyre::eyre!("Required environment variable \"RPC_URL_WSS\" not set"))?;
    Provider::<Ws>::try_from(url).map_err(|_| eyre::eyre!("Invalid RPC URL"))
}

/// Construct the searcher wallet
pub fn get_searcher_wallet() -> Result<LocalWallet> {
    let private_key = std::env::var("PRIVATE_KEY").map_err(|_| eyre::eyre!("Required environment variable \"PRIVATE_KEY\" not set"))?;
    private_key.parse::<LocalWallet>()
}


/// Construct the Uniswap V2 Pair Contract
pub fn get_univ2_contract() -> Result<uniswapV2Pair::IUniswapV2Pair<Provider<Http>>> {
    let provider = get_http_provider()?;
    let contract = uniswapV2Pair::IUniswapV2Pair::new(get_univ2_address()?, provider);
    Ok(contract)
}

