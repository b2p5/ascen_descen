use std::collections::HashMap;
use crate::HashSet;
use crate::Serialize;
use crate::Deserialize;

// Estructura para representar una transacción padre-hijos e hijos - nietos
pub struct SeparatedTxGraph {
    // Relaciones padre -> hijos
    pub parent_child_edges: HashMap<String, HashSet<String>>,
    // Relaciones hijo -> nietos
    pub child_grandchild_edges: HashMap<String, HashSet<String>>,
}
impl SeparatedTxGraph {

    // Constructor para crear un nuevo grafo de transacciones separado vacío
    pub fn new() -> SeparatedTxGraph {
        SeparatedTxGraph {
            parent_child_edges: HashMap::new(),
            child_grandchild_edges: HashMap::new(),
        }
    }

    // Función para agregar una relación padre-hijo entre dos transacciones
    pub fn add_parent_child_edges(&mut self, parent_id: String, child_id: String) {
        self.parent_child_edges.entry(parent_id).or_default().insert(child_id);
    }

    // Función para agregar una relación hijo-nieto entre dos transacciones
    pub fn add_child_grandchild_edges(&mut self, child_id: String, grandchild_id: String) {
        self.child_grandchild_edges.entry(child_id).or_default().insert(grandchild_id);
    }

    // Función para limpiar el grafo de transacciones,
    // eliminando aquellas que ya no están en la mempool
    pub fn clean_separated_tx_graph(&mut self, mempool_txs: &HashSet<String>) {

        self.parent_child_edges.retain(|tx_id, _| mempool_txs.contains(tx_id));
        self.child_grandchild_edges.retain(|tx_id, _| mempool_txs.contains(tx_id));
    }

}

// Estructura para representar los nuevas transacciones de la mempool
#[derive(Serialize, Deserialize)]
pub struct MempoolNews {
    pub txs: Vec<String>,
}
impl MempoolNews {
    // Crea una nueva instancia de MempoolNews
    pub fn new() -> MempoolNews {
        MempoolNews {
            txs: Vec::new(),
        }
    }

    // Inserta una nueva transacción en el mempool
    pub fn insert(&mut self, txid: String) {
        self.txs.push(txid);
    }

    // Elimina todas las transaccies del mempool_news
    pub fn delete(&mut self) {
        self.txs.clear();
    }

}

// Estructura para guardar el último Block de la blockchain
#[derive(Serialize, Deserialize)]
pub struct LastBlock {
    pub block: String,
}
impl LastBlock {
    // Crea una nueva instancia de LastBlock
    pub fn new() -> LastBlock {
        LastBlock {
            block: String::new(),
        }
    }

    // Inserta una nueva transacción en el mempool
    pub fn insert(&mut self, block: String) {
        self.block = block;
    }

    // Elimina todas las transaccies del mempool_news
    pub fn delete(&mut self) {
        self.block.clear();
    }

} 

// Estructura  de `getblockchaininfo`
#[derive(Deserialize)]
pub struct BlockchainInfo {
    pub bestblockhash: String,
}

// Estructura para almacenar la información de cada Tx de la mempool.
#[derive(Debug, Clone)]
// HashMap para mapear wtxid (clave) a MempoolEntry (valor).
pub struct MempoolData {
    pub entries: HashMap<String, Vec<String>>,
}
impl MempoolData {
    // Constructor para crear un nuevo Mempool vacío.
    pub fn new() -> MempoolData {
        MempoolData {
            entries: HashMap::new(),
        }
    }

    // Método para agregar una entrada al Mempool.
    pub fn add_entry(&mut self, wtxid: String, entry: Vec<String>) {
        self.entries.insert(wtxid, entry);
    }

    // Método para buscar una entrada por wtxid.
    pub fn get_entry(&self, wtxid: &str) -> Option<&Vec<String>> {
        self.entries.get(wtxid)
    }
}


