mod utils;

use borsh::*;
use namada::ledger::masp::ShieldedUtils;
use namada::types::token::Transfer;
use namada::types::transaction::TxResult;
use serde;
use serde::ser::Serialize;
use wasm_bindgen::prelude::JsValue;
use wasm_bindgen::prelude::*;

struct TransactionInBlock {
    pub gas_used: String,
    pub initialized_accounts: Vec<String>,
}

struct WebShieldedUtils;
impl ShieldedUtils for WebShieldedUtils {
    fn client(&self) -> Self::C {
        // Murisi will change this to be called something like
        // get_tx_search_result
        // will get the parameters for this call and return its data
        // https://docs.tendermint.com/v0.34/rpc/#/Info/tx_search
        !unimplemented!("this will change so that we just return data returned tx_search")
    }

    fn local_tx_prover(&self) -> LocalTxProver {
        // this will just construct the prover from the static param files
        // check out what this does
        !unimplemented!("returns a prover that was constructed with the parameter files")
    }

    fn save(&self, ctx: &namada::ledger::masp::ShieldedContext<Self>) -> std::io::Result<()> {
        // this does not do anything in the web, on desktop this will persist the context as an optimization
        // to prevent having to refetch the shielded notes
        !unimplemented!("this func does not do anything for now")
    }

    fn load(self) -> std::io::Result<namada::ledger::masp::ShieldedContext<Self>> {
        // this will return all the notes, the current web implementation passed them in that high level func
        !unimplemented!("this func returns the shielded notes that were fetched in js for now")
    }

    async fn query_conversion(
        &self,
        asset_type: AssetType,
    ) -> Option<(
        namada::types::address::Address,
        namada::types::storage::Epoch,
        masp_primitives::transaction::components::Amount,
        MerklePath<Node>,
    )> {
        // this just decoded the data that is being fetched from the storage path
        !unimplemented!("this will have to query the endpoint for a specific path, but it does not have to do anything except return the data")
    }

    async fn query_epoch(&self) -> namada::types::storage::Epoch {
        // query epoc, this is trivial we already do that
        !unimplemented!("returns the epoch, use the existing code for this")
    }

    async fn query_storage_value<T: Send>(&self, key: &namada::types::storage::Key) -> Option<T>
    where
        T: BorshDeserialize,
    {
        // check out what this does
        !unimplemented!("??")
    }

    async fn query_results(&self) -> Vec<namada::types::storage::BlockResults> {
        // check out what this does
        !unimplemented!("??")
    }
}

#[wasm_bindgen]
pub fn decode_transactions(transactions: &[u8]) -> JsValue {
    let tx_result_result = TxResult::try_from_slice(transactions);

    if let Ok(tx_result) = tx_result_result {
        return serde_wasm_bindgen::to_value(&tx_result.gas_used).unwrap();
    } else {
        let error_result =
            serde_wasm_bindgen::to_value("error @ namada-utils/src/lib.rs:decode_transaction");
        return error_result.unwrap();
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
