use ccm_runtime::{
	AccountId, AuraConfig, BalancesConfig, EVMConfig, EthereumConfig, CouncilConfig,GenesisConfig, GrandpaConfig,
	Signature, SudoConfig, SystemConfig, WASM_BINARY,
};
use sc_service::ChainType;
use serde_json::json;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public, H160, U256,crypto::UncheckedInto};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::{collections::BTreeMap, str::FromStr};
use std::convert::{TryInto, TryFrom};
use hex_literal::hex;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

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

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
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
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				AccountId::from_str("0x6Da573EEc80f63c98b88cED15D32CA270787FB5a")
					.unwrap(),
				// Pre-funded accounts
				vec![
					AccountId::from_str("0x69aA70Bd59d22FF13cB1FE90b66EBf474b4a5D32")
						.unwrap(),
					AccountId::from_str("0x8c6ca6d78EC4bDb6ce3b58B4BB788323263b7eFD")
						.unwrap(),
					AccountId::from_str("0x910D0F22f29bA12ff15E7A2aB6590944E9c2588D")
						.unwrap(),
					AccountId::from_str("0x4d4869b6d69915addA0ff8e6B839b2196b3CEF29")
						.unwrap(),
					AccountId::from_str("0x256CD031Bfa21542D6Ea1cA6bfdC987ABC49005d")
						.unwrap(),
					AccountId::from_str("0x5f807d760E50fFedb492bcAA3220115d97D95AC6")
						.unwrap(),
					AccountId::from_str("0xc1432Db8842742937E8b27475C9a220e63C2cCaD")
						.unwrap(),
					AccountId::from_str("0x9f883b12fd0692714c2f28be6c40d3afdb9081d3")
						.unwrap(),
					AccountId::from_str("0x6da573eec80f63c98b88ced15d32ca270787fb5a")
						.unwrap(),
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
				vec![(hex!["8e63609d78e8b07eff0ad673ece04798df19443ec74fdf7f6157190d0806643d"]
						  .unchecked_into(),
					  hex!["8a4d5bfc684ae1b1e48c4cb3f6b8ad663f7b3434513edf91e5243a0b1a279536"]
						  .unchecked_into()),
					 ( hex!["e4924c61e4764e8824da0cb3da6a35f789a19b6e8f5fb918aa502f63bc35fe0d"]
						   .unchecked_into(),
					   hex!["e94b5c662fe1a20f2f41b02b5a90066b5633fce1ac6e5058cb4d2d52446dc8dc"]
						   .unchecked_into())],
				// Sudo account
				AccountId::from_str("0x6Da573EEc80f63c98b88cED15D32CA270787FB5a")
					.unwrap(),
				// Pre-funded accounts
				vec![
					AccountId::from_str("0x69aA70Bd59d22FF13cB1FE90b66EBf474b4a5D32")
						.unwrap(),
					AccountId::from_str("0x8c6ca6d78EC4bDb6ce3b58B4BB788323263b7eFD")
						.unwrap(),
					AccountId::from_str("0x910D0F22f29bA12ff15E7A2aB6590944E9c2588D")
						.unwrap(),
					AccountId::from_str("0x4d4869b6d69915addA0ff8e6B839b2196b3CEF29")
						.unwrap(),
					AccountId::from_str("0x256CD031Bfa21542D6Ea1cA6bfdC987ABC49005d")
						.unwrap(),
					AccountId::from_str("0x5f807d760E50fFedb492bcAA3220115d97D95AC6")
						.unwrap(),
					AccountId::from_str("0xc1432Db8842742937E8b27475C9a220e63C2cCaD")
						.unwrap(),
					AccountId::from_str("0x9f883b12fd0692714c2f28be6c40d3afdb9081d3")
						.unwrap(),
					AccountId::from_str("0x6da573eec80f63c98b88ced15d32ca270787fb5a")
						.unwrap(),
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
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
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
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities
				.iter()
				.map(|x| (x.1.clone(), 1))
				.collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: root_key,
		},
		council: CouncilConfig::default(),
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
		base_fee: Default::default(),
	}
}
