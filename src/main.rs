// Importación de macros y dependencias necesarias para Rocket
#[macro_use] extern crate rocket;
use rocket::fs::FileServer;
use rocket::fs::relative;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use serde_json::Value;
use std::time::SystemTime;

// Importación de dependencias y módulos específicos de la aplicación
use bitcoincore_rpc::{Auth,Client, Error as BitcoinRpcError};
use serde::{Deserialize, Serialize};
use std::collections:: HashSet;

use crate::mempool_data::SeparatedTxGraph;
use crate::mempool_data::MempoolNews;
use crate::mempool_data::MempoolData;
use crate::mempool_data::LastBlock;
use crate::mempool_data::BlockchainInfo;

use crate::mempool_processor::get_raw_mempool;
use crate::mempool_processor::get_blockchain_info;
use crate::mempool_processor::get_datos_tx_mempool;
use crate::mempool_processor::get_graph;

// Importaciones del módulo API
mod api;
mod mempool_data;
mod mempool_processor;

const SLEEP_TIME: u64 = 7;
const NUM_TX_PROCE: u64 = 50000;
const USER:&str = "userX";
const PWS:&str  = "wsx";


// Función principal para lanzar el servidor Rocket
#[launch]
fn rocket() -> _ {
  
    // Inicializando el grafo de transacciones y su versión compartida entre hilos
    let separated_graph = Arc::new(Mutex::new(SeparatedTxGraph::new()));
    let separated_graph_clone = Arc::clone(&separated_graph);
    let mempool_news = Arc::new(Mutex::new(MempoolNews::new()));
    let mempool_news_clone = Arc::clone(&mempool_news);
    
    // Inicializando la estructura para almacenar la información de las transacciones
    // y su versión compartida entre hilos
    let mempool_data = Arc::new(Mutex::new(MempoolData::new()));
    let mempool_data_clone: Arc<Mutex<MempoolData>> = Arc::clone(&mempool_data);

    // Inicializando la estructura para almacenar el ultimo bloque de la blockchain
    let mut last_block_struct = LastBlock::new();


    // Creando un hilo para actualizar el grafo periódicamente
    thread::spawn(move || {

        // Iniciar contador de tiempo
        let start = std::time::Instant::now();

        // Conexión con el nodo Bitcoin Core
        let rpc_url  = "http://localhost:8332";
        let rpc_auth = Auth::UserPass(USER.to_string(), PWS.to_string());
        let client = Client::new(rpc_url, rpc_auth).expect("Error to connect Bitcoin Core");

        // Parte primera: procesar todas las transacciones de la mempool
        let mut mempool_txs = get_raw_mempool(&client).expect("Error to get mempool transactions");
        
        println!("\nESPERE, PROCESANDO TODA LA MEMPOOL (este proceso puede tardar unos minutos)\n");
        println!("=> Txs mempool: {}", mempool_txs.len());
        
        get_graph(&mempool_txs, &client, &separated_graph_clone);

        // Get mempool_entry de las transacciones de mempool y 
        // almacenar (vsize, weight, time) en mempool_data
        get_datos_tx_mempool (&mempool_txs,&client,  &mempool_data_clone);


        // Calcular tiempo transcurrido en minutos y segundos desde start
        let duration = start.elapsed();
        let minutes = duration.as_secs() / 60;
        let seconds = duration.as_secs() % 60;
        let miliseconds = duration.subsec_millis();
        let velocity = mempool_txs.len() as f64 / (seconds as f64 + miliseconds as f64 / 1000.0);
        let velocity = format!("{:.1}", velocity);


        println!("OK MEMPOOL: {}m {}s velocidad: {} Txs/s ", minutes, seconds, velocity);
        println!("YA PUEDES HACER PETICIONES VIA WEB.\n\n");

        // Bucle infinito para procesar las transacciones nuevas que llegan a la mempool
        loop {

            // Iniciar contador de tiempo
            let start = std::time::Instant::now();

            // Obteniendo las transacciones nuevas de la mempool 
            // (mempool_news_txs - mempool_txs)
            let mempool_now = get_raw_mempool(&client).expect("Error al obtener transacciones del mempool");
            let mut mempool_new_txs = HashSet::new();

            mempool_news_clone.lock().unwrap().delete();
   
            for hash_tx in mempool_now.clone() {
                if !mempool_txs.contains(&hash_tx) {
                    mempool_news_clone.lock().unwrap().insert(hash_tx.clone());
                    mempool_new_txs.insert(hash_tx);
                }
            }
            
            mempool_txs = mempool_now;

            // Procesar las transacciones nuevas de la mempool para actualizar el grafo
            get_graph(&mempool_new_txs, &client, &separated_graph_clone);

            // Eliminamos del grafo las transacciones que ya no están en la mempool
            separated_graph_clone.lock().unwrap().clean_separated_tx_graph(&mempool_txs);

            println!("=> Txs padre - hijos: {}", separated_graph_clone.lock().unwrap().parent_child_edges.len());
            println!("=> Txs hijo - nietos: {}", separated_graph_clone.lock().unwrap().child_grandchild_edges.len());

            // Get mempool_entry de las transacciones nuevas de mempool y 
            // almacenar (vsize, weight, time) en mempool_data
            get_datos_tx_mempool (&mempool_new_txs,&client,  &mempool_data_clone);


            // Get el id del ultimo bloque y almacenar en LastBlock
            let blockchain_info = get_blockchain_info(&client).expect("Error al obtener información del blockchain");

            let last_block = blockchain_info.bestblockhash;
            let last_block = last_block.to_string();
            // Almcenar last_block en la struct LastBlock
            last_block_struct.delete();
            last_block_struct.insert(last_block);


            // Calcular tiempo transcurrido en minutos y segundos desde start
            let duration = start.elapsed();
            let seconds = duration.as_secs() % 60;
            let miliseconds = duration.subsec_millis();
            let velocity = mempool_new_txs.len() as f64 / (seconds as f64 + miliseconds as f64 / 1000.0);
            // Formatea la velocidad a 1 decimal
            let velocity = format!("{:.1}", velocity);
            println!("Procesadas {} Txs nuevas: {}s {}ms velocidad: {} Txs/s \n", mempool_new_txs.len(), seconds, miliseconds, velocity);



            // Esperar 5 segundos antes de volver a procesar las transacciones nuevas
            thread::sleep(Duration::from_secs(SLEEP_TIME));
        }
    });

    // Configurando el servidor Rocket con la ruta definida
    rocket::build()
        .manage(separated_graph)
        .manage(mempool_data)
        .manage(mempool_news)
        .mount("/", routes![api::get_descen_html, api::get_descen_json, 
                            api::get_tx_mempool_data, api::get_index, 
                            api::get_mempool_datos, api::get_mempool_json, 
                            api::script_js, api::sketch_js, 
                            api::generate_token_endpoint])
        .mount("/static", FileServer::from(relative!("static")))
        

}

