use fuels::prelude::*;

// Load abi from json
abigen!(MyContract, "out/debug/log-bug-abi.json");

pub async fn get_contract_instance() -> (MyContract, ContractId, WalletUnlocked, WalletUnlocked) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(2),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
    )
    .await;
    let wallet = wallets.pop().unwrap();
    let wallet2 = wallets.pop().unwrap();

    let id = Contract::deploy(
        "./out/debug/log-bug.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/log-bug-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    let instance = MyContract::new(id.to_string(), wallet.clone());

    (instance, id.into(), wallet, wallet2)
}

#[tokio::test]
async fn log() {
    let (instance, _, _wallet, _) = get_contract_instance().await;

    let response = instance.methods().test_function().call().await.unwrap();
    let log = instance
        .logs_with_type::<RegistrationExtendedEvent>(&response.receipts)
        .unwrap();

    assert_eq!(
        log,
        vec![RegistrationExtendedEvent {
            duration: 5,
            name: SizedAsciiString::<8>::new(String::from("SwaySway")).unwrap(),
            new_expiry: 5,
        }]
    )
}
