use super::super::config::Networks;
use cardano::block;
use cardano::util::{hex, try_from_slice::TryFromSlice};
use cardano_storage::tag;
use std::sync::Arc;

use iron;
use iron::status;
use iron::{IronResult, Request, Response};

use router;
use router::Router;

use std::convert::TryFrom;
use hex::encode;
use pallas::{
    codec::utils::Nullable,
    network::{
        facades::PeerClient,
        miniprotocols::{Point, MAINNET_MAGIC},
    },
};
use pallas::ledger::traverse::{MultiEraBlock, MultiEraTx};
use minicbor::Encoder;

use super::common;

pub struct Handler {
    networks: Arc<Networks>,
}
impl Handler {
    pub fn new(networks: Arc<Networks>) -> Self {
        Handler { networks: networks }
    }
    pub fn route(self, router: &mut Router) -> &mut Router {
        router.get(":blockid", self, "block_data")
    }
}

impl iron::Handler for Handler {
    async fn handle(&self, req: &mut Request) -> IronResult<Response> {
        println!("get block data len: {:?}", &block.txs().len());
        let params = req.extensions.get::<router::Router>().unwrap();

        let ref block_hash = params.find("blockid").unwrap();
        // println!("block_hash: ", &block_hash);
         get_block_data(block_hash).await;

        Ok(Response::with((status::Ok, "hihi")))
    }
}

async fn get_block_data(hash: &&str) -> impl Future<Output=()> {
  // mainnet
  let mut peer = PeerClient::connect("relays-new.cardano-mainnet.iohk.io:3001", MAINNET_MAGIC)
  .await
  .unwrap();

  // no tx
  let point: Point = Point::Specific(
    112266064,
    hex::decode("853621ee8c3c4bf76bc48677b807f017ff24b47cd9a98cd7f2112939468534d2").unwrap(),
  );

  let cbor = peer.blockfetch().fetch_single(point).await.unwrap();
    let block = MultiEraBlock::decode(&cbor).expect("invalid cbor");

    println!(
        "Number: {}, Slot: {}, Block Hash: {}, Block body hash: {}",
        block.number(),
        block.slot(),
        block.hash(),
        block
            .header()
            .as_babbage()
            .unwrap()
            .header_body
            .block_body_hash
    );
    println!("Txs len: {:?}", &block.txs().len());

    let b_header = block.header();
    println!("Block header cbor: {:?}", hex::encode(b_header.cbor()));

    // let mut buffer = [0u8; 2000000];
    // let mut encoder: Encoder<&mut [u8]> = Encoder::new(&mut buffer[..]);
    // let tx_len = block.txs().len() as usize;
    // let tx_len64 = u64::try_from(tx_len).unwrap();
    // encoder.array(tx_len64).unwrap();

    // for tx in &block.txs() {
    //     match tx {
    //         MultiEraTx::Babbage(x) => {
    //             let transaction_body_cbor = x.transaction_body.raw_cbor();
    //             let transaction_witness_set_cbor = x.transaction_witness_set.raw_cbor();
    //             let auxiliary_data_cbor = match &x.auxiliary_data {
    //                 Nullable::Some(aux) => aux.raw_cbor(),
    //                 _ => &[],
    //             };
    //             let _ = encoder
    //             .array(3)
    //             .unwrap()
    //             .str(encode(transaction_body_cbor).as_str())
    //             .unwrap()
    //             .str(encode(transaction_witness_set_cbor).as_str())
    //             .unwrap()
    //             .str(encode(auxiliary_data_cbor).as_str())
    //             .unwrap();
    //             // .end();
    //         }
    //         _ => println!("noooooo"),
    //     }
    // }
    // let _ = encoder.end().unwrap();
    // let data = hex::encode(&buffer);
    // let index_of = match data.rfind("ff0000000000000000000000000000000000000000") {
    //     Some(data_index) => data_index,
    //     None => data.len()
    // };
    // println!("Txs cbor: {:?}", &data[..index_of]);

}
