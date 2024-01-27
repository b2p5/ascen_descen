use std::collections::HashMap;
use std::collections::HashSet;
use rocket::serde::Serialize;
use rocket::serde::Deserialize;

#[derive(Deserialize, Serialize)]
pub struct RangeWeightsTxsMempool {
    // Crear un hashmap de rangos de pesos y numero de transacciones por rango
    pub ranges:HashMap<String, u64>,
    // pub ranges:HashMap<String, Vec<String>>,
}
impl RangeWeightsTxsMempool {
    // Crear un nuevo estado del mempool
    pub fn new() -> RangeWeightsTxsMempool {
        RangeWeightsTxsMempool {
            ranges: HashMap::new(),
        }
    }

}

pub struct MempoolVarInfo {
    // pub max_weight: u64,
    // pub min_weight: u64,
    pub last_block: String,
}
impl MempoolVarInfo {
    pub fn new(last_block: String) -> MempoolVarInfo {
        MempoolVarInfo {
            // max_weight,
            // min_weight,
            last_block,
        }
    }

    pub fn set_last_block(&mut self, last_block: String) {
        self.last_block = last_block;
    }

}

// Estructura para los valores que devuelve getrawmempool true y getmempoolentry
#[derive(Deserialize, Serialize)]
#[derive(Debug)]
pub struct MempoolEntry {
    pub vsize: u64,
    pub weight: u64,
    pub time: u64,
} 
impl MempoolEntry {
    pub fn new(vsize: u64, weight: u64, time: u64) -> MempoolEntry {
        MempoolEntry {
            vsize,
            weight,
            time,
        }
    }
}  


pub struct  GetMempoolEntryTx {
    pub entries: HashMap<String, MempoolEntry>,
}
impl GetMempoolEntryTx {
    pub fn new() -> GetMempoolEntryTx {
        GetMempoolEntryTx {
            entries: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, wtxid: String, entry: MempoolEntry) {
        self.entries.insert(wtxid, entry);
    }

}


// Estructura para getGetRawMempoolTruetrue.  
// Un HashMap de wtxid (clave) a MempoolEntry (valor).
pub struct GetRawMempoolTrue{
    pub entries: HashMap<String, MempoolEntry>,
}
impl GetRawMempoolTrue{
    pub fn new() -> GetRawMempoolTrue{
        GetRawMempoolTrue{
            entries: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, wtxid: String, entry: MempoolEntry) {
        self.entries.insert(wtxid, entry);
    }

}


// Estructura para almacenar el estado del mempool
pub struct GetRawMempoolFalse {
    pub entries: HashSet<String>,
}
impl GetRawMempoolFalse {
    // Crear un nuevo estado del mempool
    pub fn new() -> GetRawMempoolFalse {
        GetRawMempoolFalse {
            entries: HashSet::new(),
        }
    }

    pub fn add_entry(&mut self, entry: String) {
        self.entries.insert(entry);
    }


}

