use crate::service::FullClient;
use node_template_runtime::opaque::Block;
use sc_transaction_pool;
use sp_blockchain::HeaderMetadata;
use tokio::time::Duration;
// use sc_transaction_pool_api::TransactionPool;
use std::sync::Arc;
use sp_core::Bytes;
use codec::{Decode, Encode};

use sc_transaction_pool_api::
	TransactionSource;

pub struct ExtraThread<TransactionPool>
where
	TransactionPool: sc_transaction_pool_api::TransactionPool + 'static,
{
	pub transaction_pool: Arc<TransactionPool>,
}

pub async fn run<TransactionPool>(config: ExtraThread<TransactionPool>)
where
	TransactionPool: sc_transaction_pool_api::TransactionPool + 'static,
{
	// submit an extrinsic
    let ext: Bytes = Bytes::from(vec![0;10]);

	let xt = match Decode::decode(&mut &ext[..]) {
        Ok(xt) => xt,
        Err(err) => return,
    };
    // let best_block_hash = self.client.info().best_hash;
    config.transaction_pool
        .submit_one(&sp_runtime::generic::BlockId::Number(0_u32.into()), TransactionSource::Local, xt)
        .await;
		
	loop {
		tokio::time::sleep(Duration::from_secs(10)).await;
		log::info!("ExtraThread::run");
	}
}
