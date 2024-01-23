use std::sync::Mutex;
use std::sync::Arc;
use bitcoincore_rpc::Client;
use bitcoincore_rpc::RpcApi;

use crate::HashSet;
use crate::SeparatedTxGraph;
use crate::MempoolData;
use crate::BlockchainInfo;
use crate::BitcoinRpcError;
use crate::Value;
use crate::SystemTime;
use crate::NUM_TX_PROCE;



// Función para obtener el grafo de transacciones
pub fn get_graph(mempool_txs: &HashSet<String>, client: &Client, 
    separated_graph: &Arc<Mutex<SeparatedTxGraph>>) {

    let mut separated_graph = separated_graph.lock().unwrap();
    let mut num_txs = 0;

    // Iterando sobre todas las transacciones de la mempool
    for hash_tx in mempool_txs {

        // Obtener los descendientes de la transacción actual (hijos)
        let descendants = get_mempool_descendants(client, hash_tx).unwrap_or_else(|_| vec![]);
        for desc_tx in descendants {
           // Procesar solamente las primeras NUM_TX_PROCE transacciones de la mempool
           num_txs += 1;
           if num_txs > NUM_TX_PROCE {
               break;
           }

           separated_graph.add_parent_child_edges(hash_tx.clone(), desc_tx.clone());

           // Obtener los descendientes de los descendientes (nietos)
           let desc_descendants = get_mempool_descendants(client, &desc_tx).unwrap_or_else(|_| vec![]);
           for desc_desc_tx in desc_descendants {
               separated_graph.add_child_grandchild_edges(desc_tx.clone(), desc_desc_tx.clone());
           }
        } 
    }

    // Iterar separated_graph.child_grandchild_edges  para eliminar de parent_child_edges 
    // los padres que están en child_grandchild_edges como hijos
    for (child_id, _grandchildren) in separated_graph.child_grandchild_edges.clone() {
        // Si child_id está en separated_graph.parent_child_edges 
        // eliminar child_id de separated_graph.parent_child_edges
        if separated_graph.parent_child_edges.contains_key(&child_id) {
           separated_graph.parent_child_edges.remove(&child_id);
        }

    }

}

pub fn get_mempool_descendants(client: &Client, txid: &str) -> bitcoincore_rpc::Result<Vec<String>> {
    match client.call("getmempooldescendants", &[Value::String(txid.to_string())]){
        Ok(descendants) => Ok(descendants),
        Err(e) => Err(e)
    }
}

pub fn get_raw_mempool(client: &Client) -> Result<HashSet<String>, BitcoinRpcError> {
    match client.call::<Vec<String>>("getrawmempool", &[serde_json::Value::Bool(false)]) {
        Ok(mempool_txids) => {
           let txids: HashSet<String> = mempool_txids.into_iter().collect();
           Ok(txids)
        },
        Err(e) => Err(e)
    }
}

pub fn get_blockchain_info(client: &Client) -> Result<BlockchainInfo, BitcoinRpcError> {
    match client.call::<BlockchainInfo>("getblockchaininfo", &[]) {
        Ok(blockchain_info) => Ok(blockchain_info),
        Err(e) => Err(e)
    }
}

pub fn get_mempool_entry(client: &Client, txid: &str) ->  Result<Vec<String>, BitcoinRpcError> {
    match client.call::<Value>("getmempoolentry", &[Value::String(txid.to_string())]) {
        Ok(mempool_entry) => {
           let vsize = mempool_entry["vsize"].as_u64().unwrap() as usize;
           let weight = mempool_entry["weight"].as_u64().unwrap() as usize;
           let _time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(mempool_entry["time"].as_u64().unwrap());
           // devuelve Ok con un vector de tres elementos
           Ok(vec![vsize.to_string(), weight.to_string()])
        
        },
        Err(e) => Err(e)
    }
}

pub fn get_datos_tx_mempool(mempool_txs: &HashSet<String>, client: &Client, mempool_data_clone: &Arc<Mutex<MempoolData>>) {
    let mut mempool_data = match mempool_data_clone.lock() {
        Ok(data) => data,
        Err(e) => {
           eprintln!("Error al bloquear MempoolData: {:?}", e);
           return;
        }
    };

    for txid in mempool_txs {
        match get_mempool_entry(client, txid) {
           Ok(mempool_entry) => {
               mempool_data.add_entry(txid.clone(), mempool_entry);
           },
           Err(e) => {
               eprintln!("Error al obtener información de la transacción {}: {:?}", txid, e);
               // Considera si quieres continuar con el siguiente txid o salir del bucle
           }
        }
    }
}
