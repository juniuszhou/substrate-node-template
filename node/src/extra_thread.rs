use crate::service::FullClient;
use node_template_runtime::{opaque::Block, Hash, UncheckedExtrinsic, RuntimeCall, Balance, SignedExtra, Runtime, Extrinsic, Address, Signature, };
use sc_service::TransactionPool as TTransactionPool;

pub use sp_runtime::OpaqueExtrinsic;
use sp_runtime::traits::Block as BlockT;
use sc_transaction_pool;
use sp_blockchain::HeaderMetadata;
use tokio::time::Duration;
// use sc_transaction_pool_api::TransactionPool;
use std::sync::Arc;
use sp_core::Bytes;
use codec::{Decode, Encode};
use sp_runtime::generic::Era;
// use pallet_transaction_payment::pallet_asset_conversion_tx_payment::ChargeTransactionPayment;
use sc_transaction_pool_api::
	TransactionSource;
// use sc_transaction_pool::uxt;
use sp_keyring::{AccountKeyring, Ed25519Keyring, Sr25519Keyring};

pub struct ExtraThread<TransactionPool>
where
	TransactionPool: sc_transaction_pool_api::TransactionPool + 'static,
	TransactionPool: sc_service::TransactionPool,
	TransactionPool::Block: BlockT,
	sp_runtime::generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>: sp_runtime::traits::Extrinsic,
{
	pub transaction_pool: Arc<TransactionPool>,
}

pub async fn run<TransactionPool>(config: ExtraThread<TransactionPool>)
where
	TransactionPool: sc_transaction_pool_api::TransactionPool + 'static,
	{
	// submit an extrinsic
    let ext: Bytes = Bytes::from(vec![0;10]);
	let mut nonce = 0_u32;

	// let keyring = sp_keyring::Sr25519Keyring::Alice;
	// keyring.sign();
	// let runtime_call = node_template_runtime::BalancesCall::transfer;

	let function = RuntimeCall::System(frame_system::Call::remark { remark: vec![0; 1] });

	// let xt = match Decode::decode(&mut &ext[..]) {
    //     Ok(xt) => xt,
    //     Err(err) => return,
	// 	// 0x30270bca5c4a5959837a60742f37f5300d33c78c9286dce19bb75438af037e9d

    // };

	let extra: SignedExtra = (
		frame_system::CheckNonZeroSender::new(),
		frame_system::CheckSpecVersion::new(),
		frame_system::CheckTxVersion::new(),
		frame_system::CheckGenesis::new(),
		frame_system::CheckEra::from(Era::mortal(256, 0)),
		frame_system::CheckNonce::from(nonce),
		frame_system::CheckWeight::new(),
		// pallet_transaction_payment::ChargeTransactionPayment{tip:0, asset_id: None},
		);
	let payload = (
		RuntimeCall::System(frame_system::Call::remark { remark: vec![0; 1] }),
		extra.clone(),
		// spec_version,
		// tx_version,
		1,
		1,
		// genesis_hash,
		// genesis_hash,
		[0; 32],
		[0; 32],
	);

	let keyring = sp_keyring::Sr25519Keyring::Alice;

	
	let signature = payload
		.using_encoded(|b| {
			if b.len() > 256 {
				keyring.sign(&sp_io::hashing::blake2_256(b))
			} else {
				keyring.sign(b)
			}
		})
		.into();

	let address = keyring.to_account_id().into();
	let extrinsic: sp_runtime::generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra> = Extrinsic::new_signed(function, address, signature, extra);

	// let expected = <<TransactionPool as sc_service::TransactionPool>::Block as BlockT>::Extrinsic::new_signed(
	// 	function,
	// 	address,
	// 	signature,
	// 	extra,
	// );

	// let xt = sign(nonce, 100, 100, [0; 32]);
    // let best_block_hash = self.client.info().best_hash;
    config.transaction_pool
        .submit_one(&sp_runtime::generic::BlockId::Number(0_u32.into()), TransactionSource::Local, extrinsic)
        .await;

	// config.transaction_pool.submit_and_watch(&sp_runtime::generic::BlockId::Number(0_u32.into()), TransactionSource::Local, xt.encode().into()).await;
		
	loop {
		tokio::time::sleep(Duration::from_secs(10)).await;
		log::info!("ExtraThread::run");
	}
}

// #[derive(Encode, Debug, Clone, Eq, PartialEq)]
// pub struct ChargeAssetTxPayment {
//     #[codec(compact)]
//     tip: u128,
//     asset_id: Option<u32>,
// }

// pub fn signed_extra(nonce: u32) -> SignedExtra {
// 	(
// 		frame_system::CheckNonZeroSender::new(),
// 		frame_system::CheckSpecVersion::new(),
// 		frame_system::CheckTxVersion::new(),
// 		frame_system::CheckGenesis::new(),
// 		frame_system::CheckEra::from(Era::mortal(256, 0)),
// 		frame_system::CheckNonce::from(nonce),
// 		frame_system::CheckWeight::new(),
// 		ChargeAssetTxPayment{tip:0, asset_id: None},
// 	)
// }

// fn sign(
// 	nonce: u32,
// 	spec_version: u32,
// 	tx_version: u32,
// 	genesis_hash: [u8; 32],
// ) -> UncheckedExtrinsic {
// 	let extra: SignedExtra = (
// 		frame_system::CheckNonZeroSender::new(),
// 		frame_system::CheckSpecVersion::new(),
// 		frame_system::CheckTxVersion::new(),
// 		frame_system::CheckGenesis::new(),
// 		frame_system::CheckEra::from(Era::mortal(256, 0)),
// 		frame_system::CheckNonce::from(nonce),
// 		frame_system::CheckWeight::new(),
// 		// pallet_transaction_payment::ChargeTransactionPayment{tip:0, asset_id: None},
// 		);
// 	let payload = (
// 		RuntimeCall::System(frame_system::Call::remark { remark: vec![0; 1] }),
// 		extra.clone(),
// 		spec_version,
// 		tx_version,
// 		genesis_hash,
// 		genesis_hash,
// 	);

// 	let keyring = sp_keyring::Sr25519Keyring::Alice;

	
// 	let signature = payload
// 		.using_encoded(|b| {
// 			if b.len() > 256 {
// 				keyring.sign(&sp_io::hashing::blake2_256(b))
// 			} else {
// 				keyring.sign(b)
// 			}
// 		})
// 		.into();
// 	UncheckedExtrinsic {
// 		signature: Some((sp_runtime::MultiAddress::Id(keyring.public().into()), signature, extra)),
// 		function: payload.0,
// 	}


			
// }