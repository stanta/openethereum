/* use crate::{
    transactions::{AddResult, Transactions},
}; */
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

/**  What to do
 (look https://github.com/stanta/dispos/raw/main/DisPOS.jpg )
Step 1
When forming an outgoing transaction to an external chain, the transaction is sent to the mempool of the “local” chain node with the addition of the addressable chain-id.

Step 2
If a node, when forming a block, encounters a transaction in the mempool to an external address (not with its own local chain-id), then it looks in the routing table, where each chain-id is associated with the addresses of the starting nodes of this chain. The node sends the transaction to the mempool of any node from this list, while adding “its” chain-id to the sender address of the transaction, as it happens in SMTP.

Step 3
The receiving node, when it detects a transaction from an external blockchain in its mempool, before including it in the block, similarly accesses the routing table and asks the sending node for the proof of the Merkle tree for the sent transaction.

Step 4
If the proof is received (that is, the transaction is confirmed by the sending node), the receiving node proceeds to verify the content of the transaction. We can say that the receiving node acts as a “light client” of the sender blockchain: “If a light client wants to determine the status of a transaction, it can simply request a Merkle proof showing that a particular transaction is in one of the Merkle trees, the root of which is in the header block for the main chain.” [5]

Step 5
Further verification of the transaction can be set at the level of smart contracts. It can be checked whether there are enough staked resources on the correspondent account to debit this transaction. Additional checks on the reputation of the blockchain that sent this transaction and others, at the choice of the receiving party, can be performed. Next, the execution of msg.data is analyzed if they were transferred to the smart contract on the receiving side.

Step 6
If all checks are passed, the transaction is included in the receiving blockchain.
*/

/* pub struct TXRoutes {
    pub prefix: transaction,
    pub endpoint: Vec<u32>,
}

 */
/* pub fn tx_route_hook (transaction: _income_tx ) -> TXRoutes {

} */

pub fn load_routes (input_file_name: String) -> HashMap<String, String> {
    
    let mut tx_route: HashMap<String, String> = HashMap::new();
    let input_file = File::open(input_file_name);
    let reader = BufReader::new(input_file);
    for line in reader.lines() {
        let s = line.unwrap().to_string();
        let tok:(str, str) = s.split(&d).collect();
        tx_route.insert(tok.0, tok.1);

    }
    format ("{:?}", tx_route)
}

pub fn main() {
    let mut tx_routes =  load_routes ("routes.csv");
 //   tx_routes.insert("0".to_string(), "https://mainnet.infura.io/v3/e48719b96ea6487b974b72a871e5aa48");
    // println!("Hello, world!");
}
