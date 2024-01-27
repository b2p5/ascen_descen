// Importación de macros y dependencias  externas
#[macro_use] extern crate rocket;
use rocket::fs::FileServer;
use rocket::fs::relative;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::time::Instant;

extern crate serde_json;

use crate::mempool_data::GetRawMempoolTrue;
use crate::mempool_data::GetRawMempoolFalse;
use crate::mempool_data::MempoolVarInfo;
use crate::mempool_data::GetMempoolEntryTx;


// Importaciones del módulo API
mod apis;
mod mempool_data;
mod mempool_processor;
mod conex;

const SLEEP_TIME: u64 = 5;

// Función principal para lanzar el servidor Rocket
#[launch]
fn rocket() -> _ {
  
    // Creando estructuras de datos para almacenar los datos de la mempool
    let get_raw_mempool_true = Arc::new(Mutex::new(GetRawMempoolTrue::new()));
    let get_raw_mempool_true_clone: Arc<Mutex<GetRawMempoolTrue>> = Arc::clone(&get_raw_mempool_true);

    let get_raw_mempool_false = Arc::new(Mutex::new(GetRawMempoolFalse::new()));
    let get_raw_mempool_false_clone: Arc<Mutex<GetRawMempoolFalse>> = Arc::clone(&get_raw_mempool_false);

    let get_mempool_entry_tx = Arc::new(Mutex::new(GetMempoolEntryTx::new()));
    let mempool_entry_tx_clone: Arc<Mutex<GetMempoolEntryTx>> = Arc::clone(&get_mempool_entry_tx);

    let range_weights_txs_mempool = Arc::new(Mutex::new(mempool_data::RangeWeightsTxsMempool::new()));
    let range_weights_txs_mempool_clone: Arc<Mutex<mempool_data::RangeWeightsTxsMempool>> = Arc::clone(&range_weights_txs_mempool);

    let mempool_var_info = Arc::new(Mutex::new(MempoolVarInfo::new("".to_string())));
    let mempool_var_info_clone: Arc<Mutex<MempoolVarInfo>> = Arc::clone(&mempool_var_info);


    // Creando un hilo para actualizar datos de la mempool periódicamente
    thread::spawn(move || {
        // Conexión con el nodo Bitcoin Core
        let client = conex::conex();

        // Get el ultimo bloque minado
        mempool_processor::get_last_block( &mempool_var_info_clone, &client);

        // Get raw_mempool false para obtener Txs de la mempool
        mempool_processor::get_raw_mempool_false( &get_raw_mempool_false_clone, &client);

        // Get datos de una transacción de la mempool con getmempoolentry txid
        let txid = "b172c27571f50ed888db58532b34ff2b66b083cd4134cbb1c7b625f7affef584".to_string();
        mempool_processor::get_mempool_entry_tx( &mempool_entry_tx_clone, &txid, &client);

        loop {

            // Get todas las Txs de la mempool con getrawmempool true
            mempool_processor::get_raw_mempool_true( &get_raw_mempool_true_clone, &client);

            // Itera raw_mempool.entries y actualiza range_weights_txs_mempool.ranges
            mempool_processor::get_range_weights_txs_mempool(&get_raw_mempool_true_clone, Arc::clone(&range_weights_txs_mempool_clone));

            // Esperar 5 segundos antes de volver a procesar las transacciones nuevas
            thread::sleep(Duration::from_secs(SLEEP_TIME));

        }

    });

    // Configurando el servidor Rocket con la ruta definida
    rocket::build()
        .manage(get_raw_mempool_true)
        .manage(get_raw_mempool_false)
        .manage(get_mempool_entry_tx)
        .manage(range_weights_txs_mempool)
        .manage(mempool_var_info)
        .attach(make_cors())
        .mount("/", routes![apis::get_raw_mempool_true_json,
                            apis::get_raw_mempool_false_json,
                            apis::get_mempool_var_info_json,
                            apis::get_range_weights_txs_mempool_json,
                            apis::get_index, 
                            apis::script_js, 
                            apis::sketch_js, 
                           ])
        .mount("/static", FileServer::from(relative!("static")))

}


// Funciones auxiliares
fn make_cors() -> Cors {
    // let allowed_origins = 
    //     AllowedOrigins::some_exact(&["http://127.0.0.1:8000/get_mempool_datos/2"]);

    CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![rocket::http::Method::Get].into_iter().map(From::from).collect(),
        allowed_headers:  AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Content-Type",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Error al configurar CORS")
}

