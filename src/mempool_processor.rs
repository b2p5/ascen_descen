use bitcoincore_rpc::Client;
use bitcoincore_rpc::RpcApi;
use serde_json::Value;
use std::sync::{Arc, Mutex};
use bitcoincore_rpc::Error as RpcError;
use std::io;

use crate::mempool_data::RawMempool;
use crate::mempool_data::MempoolDat;
use crate::mempool_data::MempoolVarInfo;
use crate::mempool_data::WeightTX;
use crate::mempool_data::RangeWeights;

const NUM_TXS_X_RANGE: u64 = 200;


pub fn get_range_weights( get_range_weights_clone: Arc<Mutex<RangeWeights>>,  
                          get_weight_tx_clone: Arc<Mutex<WeightTX>>  ) {

    let get_weight_tx = 
        get_weight_tx_clone.lock().unwrap();

    let mut get_range_weights = 
            get_range_weights_clone.lock().unwrap();


    get_range_weights.erase_range_data();


    // Iterar sobre get_weight_tx.weight_tx para obtener los rangos de pesos que se
    // graban en get_range_weights.rang_data
    let mut conta: u64 = 1;
    // let mut last_weight: &u64 = &0;
    let mut max_weight:u64 = 0;
    for (weight, txsid) in &get_weight_tx.weight_tx {
        if weight > &max_weight {
            max_weight = weight.clone();
        }
        // Iteramos el Vector txsid para obtener los txid
        for _txid in txsid {
            conta += 1;
        }

        if conta > NUM_TXS_X_RANGE {
            get_range_weights.add_range_data(weight.clone(), conta);
            conta = 1;
        }

    }

    // Calculamos el último rango de pesos
    get_range_weights.add_range_data(max_weight, conta);

}


pub fn get_weight_tx( get_weight_tx_clone: Arc<Mutex<WeightTX>>,  
                      get_raw_mempool_clone: Arc<Mutex<RawMempool>>  ) {

    let get_raw_mempool = 
        get_raw_mempool_clone.lock().unwrap();

    let mut get_weight_tx = 
            get_weight_tx_clone.lock().unwrap();

    get_weight_tx.erase_weight_tx();

    // let mut conta = 0;
    for (txid, mempool_dat) in &get_raw_mempool.data {
        // Añade a get_weight_tx.weight_tx el peso y el txid
        get_weight_tx.weight_tx.entry(mempool_dat.weight).or_insert(Vec::new()).push(txid.to_string());

        // conta += 1;
    }
    // println!("Weights - Txs procesadas: {}", conta);

    // Ordena los pesos de menor a mayor
    get_weight_tx.sort_by_weight();

}


pub fn delete_txs_last_block( txs_last_block: Vec<String>, 
                              get_raw_mempool_clone: Arc<Mutex<RawMempool>> ){

    let mut get_raw_mempool = 
            get_raw_mempool_clone.lock().unwrap();

    let mut conta = 0;
    for txid in txs_last_block {
        if get_raw_mempool.txid_exists(&txid) {
            get_raw_mempool.data.remove(&txid);
            conta += 1;
        }
        // get_raw_mempool.data.remove(&txid);
    }
    println!("Txs eliminadas: {}", conta);
}


pub fn get_txs_last_block(client: &Client) -> Result<Vec<String>, bitcoincore_rpc::Error> {

    let last_block_hash: String = client.call("getbestblockhash", &[])?;

    // Utilizar el hash para obtener los detalles del bloque, incluidas las transacciones
    let block: Value = client.call("getblock", &[last_block_hash.into()])?;

    // Intenta extraer las transacciones del bloque
    let txs = if let Some(tx_array) = block["tx"].as_array() {
        tx_array.iter()
                 .map(|tx| tx.as_str().unwrap_or_default().to_string())
                 .collect()
    } else {
        // Devuelve un error si no se encuentra el array de transacciones
        return Err(RpcError::from(io::Error::new(io::ErrorKind::Other, "No tx array found in block")));
    };

    Ok(txs)
  
}



pub fn get_last_block(client: &Client, 
                      get_mempool_var_info_clone:Arc<Mutex<MempoolVarInfo>>) -> String {

    let mut get_mempool_var_info = 
            get_mempool_var_info_clone.lock().unwrap();

    let last_block: String = client.call("getbestblockhash", &[]).unwrap();

    get_mempool_var_info.last_block = last_block.clone();

    last_block

}

pub fn add_txs_mempool_new_not_old( txs_mempool_new_not_old: Vec<String>, 
                                    get_raw_mempool_clone: Arc<Mutex<RawMempool>> , 
                                    client: &Client) {

    let mut get_raw_mempool = 
            get_raw_mempool_clone.lock().unwrap();

    let mut conta = 0;
    for txid in txs_mempool_new_not_old {
        
        // Get (size, weight, fee, fee_rate, time, height, ascen y descen) of txid from mempool
        // Si no tiene ascen o descen saltar a la siguiente iteración
        // En caso contrario obtener los valores vsize, weight, fee, fee_rate, time, height, num_ascen y num_descen
        let my_mempool_dat = get_descendant_count(&client, &txid).unwrap();
        if my_mempool_dat.num_descen > 1 {
            get_raw_mempool.data.insert(txid, my_mempool_dat);
            conta += 1;
        }
    }
    println!("Txs nuevas: {}", conta);
}

pub fn get_descendant_count(client: &Client, txid: &str) -> Result<MempoolDat, bitcoincore_rpc::Error> {
    let mut my_mempool_dat = MempoolDat::new(0, 0, 0, 0.0, 0, 0);
    
    match client.call::<Value>("getmempoolentry", &[txid.into()]) {
        Ok(entry) => {
            // Intenta extraer el número de descendientes y devuelve el valor si es exitoso.
            let descendant_count = entry.get("descendantcount").and_then(|n| n.as_u64()).unwrap_or(0);
            if descendant_count > 1 {
                let vsize = entry.get("vsize").and_then(|n| n.as_u64()).unwrap_or(0);
                let weight = entry.get("weight").and_then(|n| n.as_u64()).unwrap_or(0);
                let time = entry.get("time").and_then(|n| n.as_u64()).unwrap_or(0);
                let fees_base = entry.get("fees").and_then(|fees| fees.get("base")).and_then(|n| n.as_f64()).unwrap_or(0.0);
                let num_ascen = entry.get("ancestorcount").and_then(|n| n.as_u64()).unwrap_or(0);
                let num_descen = entry.get("descendantcount").and_then(|n| n.as_u64()).unwrap_or(0);
                
                my_mempool_dat = MempoolDat::new(vsize, weight, time, fees_base, num_ascen, num_descen);
            }
            Ok(my_mempool_dat)
        },
        Err(_) => {
            // En caso de cualquier error, devuelve 0.
            Ok(my_mempool_dat)
        },
    }
}



pub fn get_txs_mempool_new(client: &Client) -> Vec<String> {

    let raw_mempool: Vec<String> = client.call("getrawmempool", &[serde_json::Value::Bool(false)]).unwrap();

    // Toma los 2000 primeros elementos de raw_mempool
    // let txs_mempool_new: Vec<String> = raw_mempool.iter().take(2000).cloned().collect();

    let txs_mempool_new = raw_mempool;

    txs_mempool_new
}


