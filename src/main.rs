// Importación de macros y dependencias  externas
#[macro_use] extern crate rocket;
use rocket::fs::FileServer;
use rocket::fs::relative;
// use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use std::sync::{Arc, Mutex};
use std::thread;
// use bitcoincore_rpc::Client;
use std::time::Duration;
use std::collections::HashSet;

// Importación de módulos locales
mod mempool_data;
mod mempool_processor;
mod conex;
mod apis;

// Definición de constantes
const SLEEP_TIME: u64 = 5;

// Función principal para lanzar el servidor Rocket
#[launch]
fn rocket() -> _ {

    // Creando estructuras de datos para almacenar los datos de la mempool
    let get_raw_mempool = Arc::new(Mutex::new(mempool_data::RawMempool::new()));
    let get_raw_mempool_clone: Arc<Mutex<mempool_data::RawMempool>> = Arc::clone(&get_raw_mempool);

    let get_mempool_var_info = Arc::new(Mutex::new(mempool_data::MempoolVarInfo::new(String::new(),String::new(),String::new())));
    let get_mempool_var_info_clone: Arc<Mutex<mempool_data::MempoolVarInfo>> = Arc::clone(&get_mempool_var_info);

    let get_weight_tx = Arc::new(Mutex::new(mempool_data::WeightTX::new()));
    let get_weight_tx_clone: Arc<Mutex<mempool_data::WeightTX>> = Arc::clone(&get_weight_tx);

    let get_range_weights = Arc::new(Mutex::new(mempool_data::RangeWeights::new()));
    let get_range_weights_clone: Arc<Mutex<mempool_data::RangeWeights>> = Arc::clone(&get_range_weights);


    // Creando un hilo para actualizar datos de la mempool periódicamente
    thread::spawn(move || {
        // Conexión con el nodo Bitcoin Core
        let client = conex::conex();

        // Variable para almacenar txs_mempool_old  
        let mut txs_mempool_old: Vec<String> = Vec::new();

        // Variable para almacenar last_block_processed  old
        let mut last_block_processed_old = String::new();


        loop {
        
            // Iniciamos tiempo que va a durar el procesamiento de las transacciones nuevas
            let start = std::time::Instant::now();

            
            // TXS DE LA MEMPOOL
            // Get txs de la mempool actual
            let txs_mempool_new = mempool_processor::get_txs_mempool_new(&client);

            println!("Txs totales: {:?}", txs_mempool_new.len());

            // Actualizamos el numero de txs totales en la struct MempoolVarInfo
            get_mempool_var_info_clone.lock().unwrap().txs_totales = 
                       txs_mempool_new.len().to_string();
            
            // Calculamos la diferencia entre txs_mempool_new y txs_mempool_old 
            // y se almacena en txs_mempool_new_not_old
            // Primero convertimos txs_mempool_old a HashSet para poder usar el método contains
            let txs_mempool_old_set: HashSet<_> = txs_mempool_old.iter().cloned().collect();
            // Calculamos la diferencia entre txs_mempool_new y txs_mempool_old
            let txs_mempool_new_not_old: Vec<_> = txs_mempool_new
                .iter()
                .filter(|tx| !txs_mempool_old_set.contains(*tx))
                .cloned()
                .collect();

            // println!("Txs nuevas: {:?}", txs_mempool_new_not_old);

            // Añadimos txs_mempool_new_not_old a la struct RawMempool
            mempool_processor::add_txs_mempool_new_not_old(txs_mempool_new_not_old, 
                                                           get_raw_mempool_clone.clone(),
                                                           &client);

            // Actualizamos txs_mempool_old a txs_mempool_new
            txs_mempool_old = txs_mempool_new;


            // ULTIMO BLOQUE MINADO
            // Get el ultimo bloque minado
            let last_block_processed_new = mempool_processor::get_last_block( &client, get_mempool_var_info_clone.clone());
            // Si el ultimo bloque minado es distinto al ultimo bloque minado anterior
            // last_block_processed_new != last_block_processed_old
            // entonces obtenemos las txs del ultimo bloque minado y estas txs
            // las eliminamos de la struct RawMempool
            if last_block_processed_new != last_block_processed_old {

                println!("Nuevo bloque minado: {}", last_block_processed_new);
                
                // Get txs del ultimo bloque minado
                let txs_last_block_result = 
                            mempool_processor::get_txs_last_block(&client);

                let txs_last_block: Vec<String> = match txs_last_block_result {
                    Ok(txs) => txs,
                    Err(e) => {
                        println!("Error: {:?}", e);
                        Vec::new() // Return an empty vector in case of error
                    }
                };
                
                // Eliminamos txs_last_block de la struct RawMempool
                mempool_processor::delete_txs_last_block(txs_last_block, get_raw_mempool_clone.clone());
                
                // Actualizamos last_block_processed_old
                last_block_processed_old = last_block_processed_new;
                
            }


            // PESO DE LAS TXS
            // Get el peso de las txs de la mempool (RawMempool)
            // y los ordena de menor a mayor peso
            mempool_processor::get_weight_tx( get_weight_tx_clone.clone(), 
                                              get_raw_mempool_clone.clone());

            // Calculamos los rangos de pesos 
            mempool_processor::get_range_weights(get_range_weights.clone(), 
                                                 get_weight_tx_clone.clone());




            // Imprime el numero de transacciones que hay en la mempool
            println!("Txs con ascen/descen: {}", get_raw_mempool_clone.lock().unwrap().data.len());

            // Actualizamos el numero de txs totales en la struct MempoolVarInfo
            get_mempool_var_info_clone.lock().unwrap().txs_ascen_descen = 
                 get_raw_mempool_clone.lock().unwrap().data.len().to_string();
            
            // Calculo del tiempo transcurrido
            let duration = start.elapsed();
            println!("Tiempo transcurrido: {:?}\n", duration);
            

            // Esperar 5 segundos antes de volver a procesar las transacciones nuevas
            thread::sleep(Duration::from_secs(SLEEP_TIME));
        }

    });   

    // Configurando el servidor Rocket con la ruta definida
    rocket::build()
        .mount("/", routes![apis::sketch_js, 
                            apis::script_js,
                            apis::get_mempool_var_info_json,
                            apis::get_weight_tx_json,
                            apis::get_range_weights_json,
                            apis::get_raw_mempool_json,
                            apis::get_txs_range_weights_json,
                            apis::get_ascen_descen_for_txid_json,
                            apis::get_index,])
        .manage(get_raw_mempool)
        .manage(get_mempool_var_info)
        .manage(get_weight_tx)
        .manage(get_range_weights_clone)
        .mount("/static", FileServer::from(relative!("static")))
        // .attach(cors())
}
