
use crate::mempool_data::GetRawMempoolTrue;
use crate::mempool_data::GetRawMempoolFalse;
use crate::mempool_data::MempoolEntry;
use crate::mempool_data::RangeWeightsTxsMempool;
use crate::mempool_data::MempoolVarInfo;
use crate::mempool_data::GetMempoolEntryTx;
use crate::Instant;

use bitcoincore_rpc::RpcApi;
use bitcoincore_rpc::Client;
use serde_json::Value;
use std::sync::{Arc, Mutex};

// const NUM_WEIGHTS: u64 = 10000;
const NUM_RANGOS: u64 = 10;


pub fn get_range_weights_txs_mempool(get_raw_mempool_true:  &Arc<Mutex<GetRawMempoolTrue>>, 
                                     range_weights_txs_mempool_clone: Arc<Mutex<RangeWeightsTxsMempool>>)   {
    
    let get_raw_mempool_true = get_raw_mempool_true.lock().unwrap();
    let mut range_weights_txs_mempool = range_weights_txs_mempool_clone.lock().unwrap();
    
    //Get max_weight
    let mut max_weight: u64 = 0;
    let mut min_weight: u64 = 100000000;
    for (_wtxid, txinfo) in get_raw_mempool_true.entries.iter() {
        let weight = txinfo.weight;
        if weight > max_weight {
            max_weight = weight;
        }    
        if weight < min_weight {
            min_weight = weight;
        }
    }
    
    let range_widht = (max_weight - min_weight) / NUM_RANGOS;
    // Generamos los rangos de pesos
    for i in 1..NUM_RANGOS+1 {
        let range_key = format!("{}", i * range_widht);
        range_weights_txs_mempool.ranges.insert(range_key, 0);
    }
    
    // Iteramos sobre get_raw_mempool_true.entries, get weight y obtenemos range_index y range_key
    for (_wtxid, txinfo) in get_raw_mempool_true.entries.iter() {
        let weight = txinfo.weight;
        for (range_key, num_txs) in range_weights_txs_mempool.ranges.iter_mut() {
            let range_key = range_key.parse::<u64>().unwrap();
            if weight <= range_key {
                *num_txs += 1;
                break;
            }
        }

       
    }


} 


pub fn get_mempool_entry_tx(get_mempool_entry_tx:  &Arc<Mutex<GetMempoolEntryTx>>, txid: &String, client: &Client)   {
    
    let mut get_mempool_entry_tx = get_mempool_entry_tx.lock().unwrap();
    let mempool_entry: Value = client.call("getmempoolentry", &[Value::String(txid.to_string())]).unwrap();

    if let Value::Object(entry_data) = mempool_entry {
        let vsize = entry_data.get("vsize").and_then(Value::as_u64).unwrap();
        let weight = entry_data.get("weight").and_then(Value::as_u64).unwrap();
        let time = entry_data.get("time").and_then(Value::as_u64).unwrap();

        let mempool_entry = MempoolEntry::new(vsize, weight, time);
        get_mempool_entry_tx.add_entry(txid.to_string(), mempool_entry);
    }

    println!("get_mempool_entry_tx: {:?}", get_mempool_entry_tx.entries);

}


pub fn get_raw_mempool_true(get_raw_mempool_true:  &Arc<Mutex<GetRawMempoolTrue>>, client: &Client)   {
    
    let mut start = std::time::Instant::now();

    let mut get_raw_mempool_true = get_raw_mempool_true.lock().unwrap();
    let mempool_info: Value = client.call("getrawmempool", &[Value::Bool(true)]).unwrap();
    
    let duration = start.elapsed();
    println!("Tiempo de getrawmempool: {:?}", duration);

    start = std::time::Instant::now();

    if let Value::Object(entries) = mempool_info {

        // let mut conta = 0 ;
        for (wtxid, entry) in entries {

            // Si wtxid ya existe en get_raw_mempool_true.entries, no se añade
            if get_raw_mempool_true.entries.contains_key(&wtxid) {
                continue;
            }
             
            if let Value::Object(entry_data) = entry {
                let vsize = entry_data.get("vsize").and_then(Value::as_u64).unwrap();
                let weight = entry_data.get("weight").and_then(Value::as_u64).unwrap();
                let time = entry_data.get("time").and_then(Value::as_u64).unwrap();

                let mempool_entry = MempoolEntry::new(vsize, weight, time);
                get_raw_mempool_true.add_entry(wtxid, mempool_entry);
                //conta += 1;


            }
        }

        // println!("=> Txs añadidos a get_raw_mempool_true: {}", conta);
        let duration = start.elapsed();
        println!("Tiempo proceso getrawmempool: {:?}\n", duration);

        let num_txs = get_raw_mempool_true.entries.len();
        calcular_velocidad_transacciones(start, num_txs);

    }

}


pub fn get_raw_mempool_false(get_raw_mempool_false:  &Arc<Mutex<GetRawMempoolFalse>>, client: &Client)   {
    
    let mut get_raw_mempool_false = get_raw_mempool_false.lock().unwrap();
    let mempool_info: Value = client.call("getrawmempool", &[Value::Bool(false)]).unwrap();

    if let Value::Array(entries) = mempool_info {

        for entry in entries {

            get_raw_mempool_false.add_entry( entry.as_str().unwrap().to_string());

        }
    }

}


pub fn get_last_block( mempool_var_info_clone: &Arc<Mutex<MempoolVarInfo>> , client: &Client)   {
    
    let mut mempool_var_info = mempool_var_info_clone.lock().unwrap();
    let block_hash: Value = client.call("getbestblockhash", &[]).unwrap();

    if let Value::String(last_block) = block_hash {
        mempool_var_info.set_last_block(last_block);
    }
    
    
    //println!("block_hash: {}", block_hash);

}



// Funcion auxiliar
// La función calculará la velocidad de procesamiento de las transacciones
fn calcular_velocidad_transacciones(start: Instant, num_txs: usize) {
    let duration = start.elapsed();
    let seconds = duration.as_secs() % 60;
    let miliseconds = duration.subsec_millis();
    let velocity = num_txs as f64 / (seconds as f64 + miliseconds as f64 / 1000.0);
    let velocity = format!("{:.1}", velocity);

    println!("Procesadas {} Txs en {}s {}ms velocidad: {} Txs/s \n",num_txs, seconds, miliseconds, velocity);
}

