contract;

use std::logging::log;
use std::constants::ZERO_B256;

abi MyContract {
    fn test_function() -> bool;
}

pub struct RegistrationExtendedEvent {
    duration: u64,
    name: str[8],
    new_expiry: u64,
}

const ASSET_ID = ~ContractId::from(ASSET);

impl MyContract for Contract {
    fn test_function() -> bool {
        log(RegistrationExtendedEvent {
            duration: 5,
            name: "SwaySway",
            new_expiry: 5,
        });
        true
    }
}
