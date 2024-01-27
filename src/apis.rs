use rocket::{ response::{self, Responder, Response}, http::ContentType};
use std::path::Path;
use rocket::fs::NamedFile;
use rocket::State;
// use std::collections::HashMap;
use crate::Arc;
use crate::Mutex;

use crate::mempool_data::RangeWeightsTxsMempool;
use crate::mempool_data::GetRawMempoolTrue;
use crate::mempool_data::GetRawMempoolFalse;
use crate::mempool_data::MempoolVarInfo;


// API para mostrar los datos de GetMempoolEntryTx para una txid introducida por parametro 
//en formato JSON
// #[get("/get_mempool_entry_tx_json/<txid>")]
// pub fn get_mempool_entry_tx_json(  get_mempool_entry_tx: &State<Arc<Mutex<GetMempoolEntryTx>>>, txid: String ) -> JsonResponse {

//     let get_mempool_entry_tx = get_mempool_entry_tx.lock().unwrap();
//     println!("llegan: {} ", get_mempool_entry_tx.txid );

//     let stringified_json = serde_json::to_string(&get_mempool_entry_tx.txid).unwrap();
//     JsonResponse(stringified_json)
// }



// API para mostrar los datos de MempoolVarInfo en formato JSON
#[get("/get_mempool_var_info_json")]
pub fn get_mempool_var_info_json(  mempool_var_info: &State<Arc<Mutex<MempoolVarInfo>>> ) -> JsonResponse {

    let mempool_var_info = mempool_var_info.lock().unwrap();
    println!("llegan: {} ", mempool_var_info.last_block );

    let stringified_json = serde_json::to_string(&mempool_var_info.last_block).unwrap();
    JsonResponse(stringified_json)
}


#[get("/get_range_weights_txs_mempool_json")]
pub fn get_range_weights_txs_mempool_json(  range_weights_txs_mempool: &State<Arc<Mutex<RangeWeightsTxsMempool>>> ) -> JsonResponse {

    let range_weights_txs_mempool = range_weights_txs_mempool.lock().unwrap();
    println!("llegan: {} ", range_weights_txs_mempool.ranges.len() );

    let stringified_json = serde_json::to_string(&range_weights_txs_mempool.ranges).unwrap();
    JsonResponse(stringified_json)
}

// API para mostrar los datos de GetRawMempoolTrue en formato JSON
#[get("/get_raw_mempool_true_json")]
pub fn get_raw_mempool_true_json(  get_raw_mempool_true: &State<Arc<Mutex<GetRawMempoolTrue>>> ) -> JsonResponse {

    let get_raw_mempool_true = get_raw_mempool_true.lock().unwrap();
    println!("llegan: {} ", get_raw_mempool_true.entries.len() );

    let stringified_json = serde_json::to_string(&get_raw_mempool_true.entries).unwrap();
    JsonResponse(stringified_json)
}

// API para mostrar los datos de GetRawMempoolFalse en formato JSON
#[get("/get_raw_mempool_false_json")]
pub fn get_raw_mempool_false_json(  get_raw_mempool_false: &State<Arc<Mutex<GetRawMempoolFalse>>> ) -> JsonResponse {

    let get_raw_mempool_false = get_raw_mempool_false.lock().unwrap();
    println!("llegan: {} ", get_raw_mempool_false.entries.len() );

    let stringified_json = serde_json::to_string(&get_raw_mempool_false.entries).unwrap();
    JsonResponse(stringified_json)
}





#[get("/index")]
pub fn get_index() -> HtmlContent {

    let html_output = format!(
        "<!DOCTYPE html>
         <html>
             
             <head>
                 <script src='/static/p5.min.js'></script>
             </head>
             <body>
                <script src='/sketch_js'></script>
             </body>
             
         </html>" 
     );
 
     HtmlContent(html_output)
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
