

use ccm_runtime::{AccountId, AuraConfig, BalancesConfig,
                  CouncilConfig, EVMConfig, EthereumConfig, ElectionsConfig,
                  FarmConfig, DemocracyConfig, TechnicalCommitteeConfig, GenesisConfig,
                  GrandpaConfig,
                  Signature, SudoConfig, SystemConfig,
                  ValidatorSetConfig,
                  SessionConfig,
                  opaque::SessionKeys,
                  EthereumSigner,
                  WASM_BINARY, Balance, DOLLARS};
// FarmConfig,
use hex_literal::hex;
use penguin_farm::{PenguinConfig, PenguinStatus};
use sc_service::ChainType;
use serde_json::json;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::UncheckedInto, Pair, Public, H160, U256,OpaquePeerId};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::{collections::BTreeMap, str::FromStr};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// compose a session-key objects.
fn session_keys(
    aura: AuraId,
    grandpa: GrandpaId,
) -> SessionKeys {
    SessionKeys { aura, grandpa }
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura GrandpaId authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
    (
        get_from_seed::<AuraId>(s),
        get_from_seed::<GrandpaId>(s)
    )
}

pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![
                    (
                        AccountId::from_str("0x6Da573EEc80f63c98b88cED15D32CA270787FB5a").unwrap(),
                        get_from_seed::<AuraId>("Alice"),
                        get_from_seed::<GrandpaId>("Alice"),
                    ),
                ],
                //vec![authority_keys_from_seed("Alice")],
                // Sudo account
                AccountId::from_str("0x6Da573EEc80f63c98b88cED15D32CA270787FB5a").unwrap(),
                // Pre-funded accounts
                vec![
                    AccountId::from_str("0x6Da573EEc80f63c98b88cED15D32CA270787FB5a").unwrap(),
                    AccountId::from_str("0x7cb821cB086B979a6eD429543859c1A9eFE0928c").unwrap(),
                    AccountId::from_str("0x9d9B0A61b4E0B60C9e91E87362C31C261eE22Eae").unwrap(),
                    AccountId::from_str("0x2c95eDE69Da04b2cdB1c7873c0e2ea8Fb121a596").unwrap(),
                    AccountId::from_str("0x3fcAA1E75397b4Ad18Be21036e6Cc28bC97F602A").unwrap(),
                    AccountId::from_str("0x5EF1068Db4e1e3CA3551a574aa1397d4dC4422bB").unwrap(),
                    AccountId::from_str("0xef11BbD0653255294eF8927b7fa15015d911Bba3").unwrap(),

                ],
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        Some("ccm"),
        // Properties
        Some(
            json!({
              "tokenDecimals": 18,
              "tokenSymbol": "CCM"
            })
            .as_object()
            .expect("Error")
            .clone(),
        ),
        // Extensions
        None,
    ))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    let (acc,au,gp): (AccountId, AuraId, GrandpaId) =(
        AccountId::from_str("0x6Da573EEc80f63c98b88cED15D32CA270787FB5a").unwrap(),
        hex!["708e2d070222049c6684ce608845d6c28168aa45946593d9a0ce69d3d6c9be56"]
            .unchecked_into(),
        hex!["8fcd4ac76751ca962f1fe92713c12710f4bdf9c23100baa0030b463aa6218fb8"]
            .unchecked_into(),
    );

    //let acc_58=bs58::encode(acc.as_ref()).into_string();
    //let auro_58=bs58::encode(au.as_ref()).into_string();
    //let gp_58=bs58::encode(gp.as_ref()).into_string();
    //log::error!("accid:{},auro:{},gran:{}",acc_58,auro_58,gp_58);

    //log::error!("account:{:?}, auraId:{:?}, gran:{:?}",bs58::encode(acc.as_ref()), au.as_ref(),gp.as_ref() );

    Ok(ChainSpec::from_genesis(
        // Name
        "ccm Testnet",
        // ID
        "ccm_testnet",
        ChainType::Custom(String::from("ccm")),
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![
                    (
                        AccountId::from_str("0x6Da573EEc80f63c98b88cED15D32CA270787FB5a").unwrap(),
                        hex!["708e2d070222049c6684ce608845d6c28168aa45946593d9a0ce69d3d6c9be56"]
                            .unchecked_into(),
                        hex!["8fcd4ac76751ca962f1fe92713c12710f4bdf9c23100baa0030b463aa6218fb8"]
                            .unchecked_into(),
                    ),
                    (
                        AccountId::from_str("0x7cb821cB086B979a6eD429543859c1A9eFE0928c").unwrap(),
                        hex!["72b8edbe194dfa3973bf16c2f67de0d369f706a50ce75598ce719a23709ec761"]
                            .unchecked_into(),
                        hex!["f3adf3464c7f62f7d5f546ad7d4ec878451b62e239fad3833feac6cf12559eff"]
                            .unchecked_into(),
                    ),
                ],
                // Sudo account
                AccountId::from_str("0x6Da573EEc80f63c98b88cED15D32CA270787FB5a").unwrap(),
                // Pre-funded accounts
                vec![
                    AccountId::from_str("0x6da573eec80f63c98b88ced15d32ca270787fb5a").unwrap(),
                    AccountId::from_str("0x2c95eDE69Da04b2cdB1c7873c0e2ea8Fb121a596").unwrap(),
                    AccountId::from_str("0x3fcAA1E75397b4Ad18Be21036e6Cc28bC97F602A").unwrap(),
                    AccountId::from_str("0x5EF1068Db4e1e3CA3551a574aa1397d4dC4422bB").unwrap(),
                    AccountId::from_str("0x9d9B0A61b4E0B60C9e91E87362C31C261eE22Eae").unwrap(),
                    AccountId::from_str("0x7cb821cB086B979a6eD429543859c1A9eFE0928c").unwrap(),
                    AccountId::from_str("0x5f807d760E50fFedb492bcAA3220115d97D95AC6").unwrap(),
                    AccountId::from_str("0xc1432Db8842742937E8b27475C9a220e63C2cCaD").unwrap(),
                    AccountId::from_str("0x9f883b12fd0692714c2f28be6c40d3afdb9081d3").unwrap(),

                ],
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        Some("ccm"),
        // Properties
        Some(
            json!({
              "tokenDecimals": 18,
              "tokenSymbol": "CCM"
            })
            .as_object()
            .expect("Error")
            .clone(),
        ),
        // Extensions
        None,
    ))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AccountId, AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    _enable_println: bool,
) -> GenesisConfig {
	let  num_endowed_accounts=endowed_accounts.len();
	const STASH: Balance = 100 * DOLLARS;
    GenesisConfig {
        system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
            changes_trie_config: Default::default(),
        },
        balances: BalancesConfig {
            // Configure endowed accounts with initial balance of 1 << 60.
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 70))
                .collect(),
        },
        validator_set: ValidatorSetConfig {
            validators: initial_authorities.iter().map(|x| x.0.clone()).collect::<Vec<_>>(),
        },
        session: SessionConfig {
            keys: initial_authorities.iter().map(|x| {
                (x.0.clone(), x.0.clone(), session_keys(x.1.clone(), x.2.clone()))
            }).collect::<Vec<_>>(),
        },
		elections: ElectionsConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.map(|member| (member, STASH))
				.collect(),
		},
        aura: AuraConfig {
            authorities: vec![],
        },
        grandpa: GrandpaConfig {
            authorities: vec![],
        },
        sudo: SudoConfig {
            // Assign network admin rights.
            key: root_key,
        },
		technical_committee: TechnicalCommitteeConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		},
		democracy: DemocracyConfig::default(),
        council: CouncilConfig::default() ,
        treasury: Default::default(),
        evm: EVMConfig {
            accounts: {
                let mut map = BTreeMap::new();
                map.insert(
                    // H160 address of Alice dev account
                    // Derived from SS58 (42 prefix) address
                    // SS58: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
                    // hex: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
                    // Using the full hex key, truncating to the first 20 bytes (the first 40 hex chars)
                    H160::from_str("d43593c715fdd31c61141abd04a99fd6822c8558")
                        .expect("internal H160 is valid; qed"),
                    pallet_evm::GenesisAccount {
                        balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
                            .expect("internal U256 is valid; qed"),
                        code: Default::default(),
                        nonce: Default::default(),
                        storage: Default::default(),
                    },
                );
                map.insert(
                    // H160 address of CI test runner account
                    H160::from_str("6be02d1d3665660d22ff9624b7be0551ee1ac91b")
                        .expect("internal H160 is valid; qed"),
                    pallet_evm::GenesisAccount {
                        balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
                            .expect("internal U256 is valid; qed"),
                        code: Default::default(),
                        nonce: Default::default(),
                        storage: Default::default(),
                    },
                );
                map
            },
        },
        ethereum: EthereumConfig {},
        dynamic_fee: Default::default(),
        farm: FarmConfig {},
        /*node_authorization: NodeAuthorizationConfig {
            nodes: vec![
                (
                    OpaquePeerId(bs58::decode("12D3KooWFsLQAmNCpxgeeXY9mH9DgqgchutwmSL2wqeaAhhbG1R4").into_vec().unwrap()),
                    root_key.clone()
                ),
                (
                    OpaquePeerId(bs58::decode("12D3KooWB6bcZYvH4iRih9HndMGsSYdVMA1xjN1gJyq6ifYxprYN").into_vec().unwrap()),
                    endowed_accounts[0].clone()
                ),
            ],
        },*/
    }
}


#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn hello_world_is_ok() {
        let (acc,au,gp): (AccountId, AuraId, GrandpaId) =(
            AccountId::from_str("0x6Da573EEc80f63c98b88cED15D32CA270787FB5a").unwrap(),
            hex!["708e2d070222049c6684ce608845d6c28168aa45946593d9a0ce69d3d6c9be56"]
                .unchecked_into(),
            hex!["8fcd4ac76751ca962f1fe92713c12710f4bdf9c23100baa0030b463aa6218fb8"]
                .unchecked_into(),
        );

        let acc_58=bs58::encode(acc.as_ref()).into_string();
        let auro_58=bs58::encode(au.as_ref()).into_string();
        let gp_58=bs58::encode(gp.as_ref()).into_string();
        println!("accid:{},auro:{},gran:{}",acc_58,auro_58,gp_58)
    }



}