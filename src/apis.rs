// use bitcoincore_rpc::bitcoin::hashes::hash160::Hash;
use rocket::{ response::{self, Responder, Response}, http::ContentType};
use std::path::Path;
use rocket::fs::NamedFile;
use rocket::State;
use crate::Arc;
use crate::Mutex;
use std::collections::HashMap;
use crate::mempool_data::MempoolDat;

use std::string::String;

use bitcoincore_rpc::RpcApi;

use crate::mempool_data::RawMempool;
use crate::mempool_data::MempoolVarInfo;
use crate::mempool_data::WeightTX;
use crate::mempool_data::RangeWeights;
use crate::mempool_data::AscenDescenForTxid;

use crate::conex;

#[get("/index")]
pub fn get_index() -> HtmlContent {

    let html_output = format!(
        "<!DOCTYPE html>
         <html>
             
             <head>
                 <script src='/static/p5.min.js'></script>
             </head>
             <body>
                <script align='center' src='/sketch_js'></script>
             </body>
             
         </html>" 
     );
 
     HtmlContent(html_output)
}

// API - 3 para mostrar descen y ascen de un tx
#[get("/get_ascen_descen_for_txid_json/<txid>")]
pub fn get_ascen_descen_for_txid_json ( txid: String) -> JsonResponse {

    let txid = txid;
    let txid_2 = txid.clone();

    // Conexión con el nodo Bitcoin Core
    let client = conex::conex();

    // Inicializamos la estructura de datos
    let mut ascen_descen_for_txid = AscenDescenForTxid::new();
    ascen_descen_for_txid.delete();
    
    // Obtenemos las txs descendientes de txid
    let descen_for_txid: serde_json::Value = client.call("getmempooldescendants", &[serde_json::Value::String(txid.to_string())]).unwrap();
    
    // Iteramos descen_for_txid para obtener las txs descendientes de txid
    let mut vec_descen_for_txid =Vec::new();
    for txid_descen in descen_for_txid.as_array().unwrap() {
        // Añade txid_descen a vec_descen_for_txid
        vec_descen_for_txid.push(txid_descen.as_str().unwrap().to_string());
        
    }
    ascen_descen_for_txid.descen.insert(txid, vec_descen_for_txid);

    // Obtenemos las txs ascendientes de txid
    let ascen_for_txid: serde_json::Value = client.call("getmempoolancestors", &[serde_json::Value::String(txid_2.to_string())]).unwrap();
    let mut vec_ascen_for_txid =Vec::new();
    for txid_ascen in ascen_for_txid.as_array().unwrap() {
        // Añade txid_ascen a vec_ascen_for_txid
        vec_ascen_for_txid.push(txid_ascen.as_str().unwrap().to_string());
    }
    ascen_descen_for_txid.ascen.insert(txid_2, vec_ascen_for_txid);


    let ascen_descen_for_txid_string = serde_json::to_string(&ascen_descen_for_txid).unwrap();
    JsonResponse(ascen_descen_for_txid_string)

}



// API  - 2 para mostrar los txs de un rango de pesos
#[get("/get_txs_range_weights_json/<weight1>/<weight2>")]
pub fn get_txs_range_weights_json( weight1: u64, weight2: u64, 
                                   get_weight_tx: &State<Arc<Mutex<WeightTX>>>,
                                   get_raw_mempool: &State<Arc<Mutex<RawMempool>>>) -> JsonResponse {

    let get_weight_tx = get_weight_tx.lock().unwrap();
    let get_raw_mempool = get_raw_mempool.lock().unwrap();

    let mut txs: HashMap<String, MempoolDat> = HashMap::new();

    for (weight, txsid) in &get_weight_tx.weight_tx {
        if weight >= &weight1 && weight <= &weight2 {
            for txid in txsid {
                // Busca el txid en get_raw_mempool.data
                // Si lo encuentra añade data: HashMap<String, MempoolDat> a txs
                let _ = get_raw_mempool.data.iter().find(|&(id, _data)| id == txid).map(|(id, data)| txs.insert(id.clone(), data.clone()));
            }
        }
    }

    let stringified_json = serde_json::to_string(&txs).unwrap();
    JsonResponse(stringified_json)
}


// API - 1 para mostrar los datos de RangeWeights
#[get("/get_range_weights_json")]
pub fn get_range_weights_json( get_range_weights: &State<Arc<Mutex<RangeWeights>>> ) -> JsonResponse {

    let get_range_weights = get_range_weights.lock().unwrap();

    let stringified_json = serde_json::to_string(&get_range_weights.rang_data).unwrap();
    JsonResponse(stringified_json)
}


// API - 0 con información general de la mempool
#[get("/get_mempool_var_info_json")]
pub fn get_mempool_var_info_json(  mempool_var_info: &State<Arc<Mutex<MempoolVarInfo>>> ) -> JsonResponse {

    let mempool_var_info: std::sync::MutexGuard<'_, MempoolVarInfo> = mempool_var_info.lock().unwrap();
  
    // Generar JSON con last_block y txt_totales
    let stringified_json = serde_json::to_string(&*mempool_var_info).unwrap();

    JsonResponse(stringified_json)
}



/////////////////////////////////////////////////////////////////////////////////
// APIS Auxiliares
/////////////////////////////////////////////////////////////////////////////////



// API para mostrar los datos de WeightTX en formato JSON
#[get("/get_weight_tx_json")]
pub fn get_weight_tx_json(  get_weight_tx: &State<Arc<Mutex<WeightTX>>> ) -> JsonResponse {

    let get_weight_tx = get_weight_tx.lock().unwrap();
    // println!("llegan: {} ", get_weight_tx.weight_tx.len() );

    let stringified_json = serde_json::to_string(&get_weight_tx.weight_tx).unwrap();
    JsonResponse(stringified_json)
}


// API para mostrar los datos de RawMempool.data  en formato JSON
#[get("/get_raw_mempool_json")]
pub fn get_raw_mempool_json(  get_raw_mempool: &State<Arc<Mutex<RawMempool>>> ) -> JsonResponse {

    let get_raw_mempool = get_raw_mempool.lock().unwrap();
    // println!("llegan: {} ", get_raw_mempool.data.len() );

    let stringified_json = serde_json::to_string(&get_raw_mempool.data).unwrap();
    JsonResponse(stringified_json)
}





// Estructura para manejar contenido HTML como respuesta
pub struct HtmlContent(String);
// Implementación del trait Responder para HtmlContent, permitiendo su uso como respuesta HTTP
impl<'r> Responder<'r, 'static> for HtmlContent {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> response::Result<'static> {
        Response::build()
            .header(ContentType::HTML)
            .sized_body(self.0.len(), std::io::Cursor::new(self.0))
            .ok()
    }
}

// Estructura para manejar la respuesta JSON
pub struct JsonResponse(String);
impl<'r> Responder<'r, 'static> for JsonResponse {
    fn respond_to(self, _: &'r rocket::Request) -> response::Result<'static> {
        Response::build()
            .header(ContentType::JSON)
            .sized_body(self.0.len(), std::io::Cursor::new(self.0))
            .ok()
    }
}

#[get("/script_js")]
pub async fn script_js() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/script.js")).await.ok()
}
#[get("/sketch_js")]
pub async fn sketch_js() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/sketch.js")).await.ok()
}
