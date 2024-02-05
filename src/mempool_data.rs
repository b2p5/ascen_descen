use std::collections::HashMap;
// use std::collections::HashSet;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AscenDescenForTxid {
    pub descen: HashMap<String , Vec<String>>,
    pub ascen: HashMap<String , Vec<String>>,
}
impl AscenDescenForTxid {
    pub fn new() -> AscenDescenForTxid {
        AscenDescenForTxid {
            descen: HashMap::new(),
            ascen: HashMap::new(),
        }
    }

    pub fn delete (&mut self) {
        self.descen.clear();
        self.ascen.clear();
    }
}


#[derive(Serialize, Deserialize)]
// Estructura de datos para almacenar los datos de las Txs de la mempool
pub struct RawMempool {
    pub data: HashMap<String, MempoolDat>,
}
impl RawMempool {
    pub fn new() -> RawMempool {
        RawMempool {
            data: HashMap::new(),
        }
    }
    pub fn txid_exists(&self, txid: &str) -> bool {
        self.data.contains_key(txid)
    }
    // pub fn get_mempooldat(&self, txid: &str) -> MempoolDat {
    //     self.data.get(txid).unwrap().clone()
    // }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MempoolDat {
    pub vsize: u64,
    pub weight: u64,
    pub time: u64,
    pub fees_base: f64,
    pub num_ascen: u64,
    pub num_descen: u64,
} 
impl MempoolDat {
    pub fn new( vsize: u64, weight: u64, time: u64, fees_base: f64,
                num_ascen: u64, num_descen: u64) -> MempoolDat {
        MempoolDat {
            vsize,
            weight,
            time,
            fees_base,
            num_ascen,
            num_descen,
        }
    }
}

// Estructura de datos para almacenar  datos varios de la mempool
pub struct MempoolVarInfo {
    pub last_block: String,
}
impl MempoolVarInfo {
    pub fn new(last_block: String) -> MempoolVarInfo {
        MempoolVarInfo {
            last_block,
        }
    }

}

// Estructura de datos para almacenar weights y txs de la mempool
pub struct WeightTX {
    pub weight_tx:HashMap<u64, Vec<String>>,
}
impl WeightTX {
    pub fn new() -> WeightTX {
        WeightTX {
            weight_tx: HashMap::new(),
        }
    }

    pub fn erase_weight_tx(&mut self) {
        self.weight_tx.clear();
    }

    pub fn sort_by_weight(&mut self) {
        let weight_tx = self.weight_tx.clone();
        let mut weight_tx_sorted = HashMap::new();
        let mut weights: Vec<u64> = weight_tx.keys().cloned().collect();
        weights.sort();
        weights.reverse();
        for weight in weights {
            weight_tx_sorted.insert(weight, weight_tx.get(&weight).unwrap().clone());
        }
        self.weight_tx = weight_tx_sorted;
    }

}

// Estructura de datos para almacenar rangos de pesos y txs de la mempool
pub struct RangeWeights  {
    pub rang_data:HashMap<u64, u64>,
}
impl RangeWeights  {
    pub fn new() -> RangeWeights  {
        RangeWeights  {
            rang_data: HashMap::new(),
        }
    }

    pub fn erase_range_data(&mut self) {
        self.rang_data.clear();
    }

    pub fn add_range_data(&mut self, weight: u64, num: u64) {
        self.rang_data.entry(weight).or_insert(num);
    }

}