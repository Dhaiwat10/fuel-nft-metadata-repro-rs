use fuels::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let wallet = launch_provider_and_get_wallet().await?;

    let contract_id: Bech32ContractId = Contract::load_from(
        "./nft-contract/out/debug/nft-contract.bin",
        LoadConfiguration::default(),
    )?
    .deploy(&wallet, TxPolicies::default())
    .await?;

    abigen!(Contract(
        name = "MyContract",
        // Replace with your contract ABI.json path
        abi = "./nft-contract/out/debug/nft-contract-abi.json"
    ));

    println!("Contract deployed @ {contract_id}");

    let connected_contract_instance: MyContract<WalletUnlocked> = MyContract::new(contract_id, wallet);

    let tx1 = connected_contract_instance.methods().set_metadata(BASE_ASSET_ID, "0".to_string(), Metadata::String(("test").to_string())).call().await?;

    // print tx1 receipts
    println!("tx1: {:?}", tx1);

    let tx2 = connected_contract_instance.methods().metadata(BASE_ASSET_ID, "0".to_string()).simulate().await?;

    println!("tx2 receipts: {:?}", tx2);

    Ok(())
}
