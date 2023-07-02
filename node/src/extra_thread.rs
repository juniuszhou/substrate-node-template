use crate::service::FullClient;
use node_template_runtime::opaque::Block;
use sc_transaction_pool;
use sp_blockchain::HeaderMetadata;
use tokio::time::Duration;
// use sc_transaction_pool_api::TransactionPool;
use std::sync::Arc;

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

	// config.transaction_pool.submit_one(
		
	// 	Default::default(),
	// 	Default::default(),
    //     Default::default(),
	// );

	loop {
		tokio::time::sleep(Duration::from_secs(10)).await;
		log::info!("ExtraThread::run");
	}
}
