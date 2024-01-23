use rocket::{State, response::{self, Responder, Response}, http::ContentType};
use rocket::http::{Cookie, CookieJar};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::path::Path;
use rocket::fs::NamedFile;
use jsonwebtoken::Header;
use jsonwebtoken::EncodingKey;
use uuid::Uuid;
use jsonwebtoken::encode;

use crate::MempoolData;
use crate::SeparatedTxGraph;
use crate::LastBlock;
use crate::MempoolNews;


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

// Estructura para generar cookies
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}


// Ruta del servidor web para obtener las transacciones descendientes en formato HTML
#[get("/get_tx_mempool_data")]
pub fn get_tx_mempool_data(  mempool_data: &State<Arc<Mutex<MempoolData>>> ) -> HtmlContent {
    
    let mempool_data = mempool_data.lock().unwrap();
    

    // Generando contenido HTML con las transacciones
    let mut transactions = String::new();

    // transactions.push_str("<!DOCTYPE html>");
    transactions.push_str("<h1>Datos de las Txs de la Mempool</h1>");

    for (txid, datos) in mempool_data.entries.iter() {
        transactions.push_str(&format!("<p>{} - {}</p>", txid, datos[0]));
    }

    // Transacciones separadas padre - padres
    transactions.push_str("<style> .tx-padre { color: black; } .tx-hijo { color: green; } .tx-nieto { color: blue; } </style>");

    // Empaquetando el contenido HTML como una respuesta
    let html_output = format!("<html><body>{}</body></html>", transactions);
    HtmlContent(html_output)

}

#[get("/get_mempool_json")]
pub fn get_mempool_json(cookies: &CookieJar<'_>) -> HtmlContent {

    // Verificar si ya existe un token y que transacciones hay que enviar, toas o incremental
    if let Some(_cookie) = cookies.get("user_token") {

        let now_time = std::time::SystemTime::now();
        // Pasar now_time a u64
        let now_time_number = now_time.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

        // Obtener el tiempo transcurrido desde la última petición
        let now_time_cookie = cookies.get("now_time").unwrap().value().parse::<u64>().unwrap();
        let elapstime = now_time_number - now_time_cookie;
        
        // Si el tiempo transcurrido es mayor que 20 segundos, 
        // el tipo es 1 y se envían todas las transacciones
        if elapstime > 20 {
            cookies.add(Cookie::new("tipo_tx", "1"));
        } else {
            cookies.add(Cookie::new("tipo_tx", "2"));
        }

        // Formatear la hora actual en formato UNIX
        let now_time_string: String = now_time.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs().to_string();
        cookies.add(Cookie::new("now_time", now_time_string ));

    } else {
        // Generar y almacenar un nuevo token si no existe
        let token = generate_token(); 
        println!("Token generado: {}", token);
        cookies.add(Cookie::new("user_token", token));
        cookies.add(Cookie::new("tipo_tx", "0" ));
        cookies.add(Cookie::new("now_time", "0" ));
    }

    // Obtener las transacciones de la mempool. Implementa tu lógica aquí.
    let txs = get_mempool_txs(); // Esta función debe ser implementada por ti

    txs

    
}

fn get_mempool_txs() -> HtmlContent {
    
    let html_output = format!(
        "<!DOCTYPE html>
         <html>
             
             <head>
                 <script src='/static/p5.min.js'></script>
                 <script src='script_js'></script>
             </head>
             <body>
                <script src='/sketch_js'></script>
             </body>
             
         </html>" 
     );
 
     HtmlContent(html_output)


}

#[get("/get_mempool_datos/<tipo>")]
pub fn get_mempool_datos( separated_graph: &State<Arc<Mutex<SeparatedTxGraph>>>,
                      mempool_news_clone:  &State<Arc<Mutex<MempoolNews>>>,
                      tipo: u16) -> String {

    let mut txs: Vec<String> = vec![];
    
    if tipo == 1 {
        // Iteramos sobre las transacciones padre separated_graph.parent_child_edges
        // generando un elemento txs por cada transacción padre
        let separated_graph = separated_graph.lock().unwrap();
        for (parent_id, _children) in separated_graph.parent_child_edges.iter() {
            txs.push(parent_id.parse::<String>().unwrap());
         }

    } else if tipo == 2 {
        // Iteramos sobre las transacciones de mempool_news llevandolas a txs
        let mempool_news_clone = mempool_news_clone.lock().unwrap();
        for tx in mempool_news_clone.txs.iter() {
            txs.push(tx.clone());
        }

    } else if tipo == 3 {
        // Get primer elemento de mempool_news_clone.txs
        let mempool_news_clone = mempool_news_clone.lock().unwrap();
        let tx = mempool_news_clone.txs.first().unwrap();
        txs.push(tx.clone());

    } else if tipo == 4 { 
        // Get el id del ultimo bloque
        let last_block_struct = LastBlock::new();
        let last_block = last_block_struct.block.clone();
        txs.push(last_block);  

    }   
    

    // Convertir el vector txs a JSON
    let json_txs = json!(txs).to_string();

    json_txs

}

#[get("/generate_token")]
pub fn generate_token_endpoint(cookies: &CookieJar<'_>) -> String {
    let token = generate_token();
    cookies.add(Cookie::new("user_key", token.clone()));
    token
}
fn generate_token() -> String {
    let my_uuid = Uuid::new_v4().to_string();
    let my_claims = Claims {
        sub: my_uuid,
        company: "b2p5".into(),
        exp: 10000000000,
    };
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap();

    token
}

#[get("/script_js")]
pub async fn script_js() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/script.js")).await.ok()
}
#[get("/sketch_js")]
pub async fn sketch_js() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/sketch.js")).await.ok()
}

#[get("/index")]
pub fn get_index() -> HtmlContent {

    // Llamar a la funccion descen_json para obtener el string stringified_json
    let mut stringified_json: Vec<String> = vec![];
    //Añade a stringified_json tres transacciones
    stringified_json.push("1".to_string());
    stringified_json.push("2".to_string());
    stringified_json.push("3".to_string());
    // Convierte stringified_json a String
    let stringified_json = serde_json::to_string(&json!({"descendientes": stringified_json})).unwrap();
 
   
    let html_output = format!(
        "<!DOCTYPE html>
         <html>
             
             <head>
                 <script src='/static/p5.min.js'></script>
                 <script src='script_js'></script>
             </head>
             <body>
                <script>let datosDescendientes = {};</script>
                <script src='/sketch_js'></script>
                <div id='apiResult'>Cargando...</div>
             </body>
             
         </html>" , 
        stringified_json
     );
 
     HtmlContent(html_output)
}

// Transacciones descendientes en formato HTML
#[get("/get_descen_html")]
pub fn get_descen_html( separated_graph: &State<Arc<Mutex<SeparatedTxGraph>>>,
                    mempool_data: &State<Arc<Mutex<MempoolData>>> ) -> HtmlContent {
    
    let separated_graph = separated_graph.lock().unwrap();
    let mempool_data = mempool_data.lock().unwrap();

    let mut conta_padre ;
    let mut conta_hijo ;
    let mut conta_nieto ;
    let mut conta_todo = 0;

    let mut vsize: String;
    let mut weight: String;

    let mut tx_mempool_data: Option<&Vec<String>>;

    // Generando contenido HTML con las transacciones
    let mut transactions = String::new();

    // transactions.push_str("<!DOCTYPE html>");
    transactions.push_str("<h1>Txs de la Mempool</h1>");

    // Transacciones separadas padre - padres
    let conta_tot_padre  = separated_graph.parent_child_edges.len() ;
    transactions.push_str(&format!("<h3> Total transacciones padre: {} </h3>", conta_tot_padre));

    transactions.push_str("<style> .tx-padre { color: black; } .tx-hijo { color: green; } .tx-nieto { color: blue; } </style>");

    // Iteramos sobre las transacciones padre separated_graph.parent_child_edges
    conta_padre = 1;
    for (parent_id, children) in separated_graph.parent_child_edges.iter() {
        
        tx_mempool_data = mempool_data.get_entry(parent_id);
        
        vsize = match tx_mempool_data {
            Some(vec) => vec[0].clone(),
            None => String::from("Default Value"), // Replace "Default Value" with a suitable default
        };
        weight = match tx_mempool_data {
            Some(vec) => vec[1].clone(),
            None => String::from("Default Value"), // Replace "Default Value" with a suitable default
        };
        
        transactions.push_str(&format!("<p class='tx-padre'> {}:Tx padre: {:?}  ( vsize: {} - weight: {} )</p>",conta_padre, parent_id, vsize, weight));
        conta_todo += 1;
        conta_padre += 1;

        // Iteramos sobre las transacciones hijo children
        conta_hijo = 1;
        for child_id in children {

            tx_mempool_data = mempool_data.get_entry(child_id);
        
            vsize = match tx_mempool_data {
                Some(vec) => vec[0].clone(),
                None => String::from("Default Value"), // Replace "Default Value" with a suitable default
            };
            weight = match tx_mempool_data {
                Some(vec) => vec[1].clone(),
                None => String::from("Default Value"), // Replace "Default Value" with a suitable default
            };

            transactions.push_str(&format!("<p class='tx-hijo'>&nbsp;&nbsp; {}:Tx hijo: {:?} ( vsize: {} - weight: {} )</p>",conta_hijo, child_id, vsize, weight));
            conta_todo += 1;
            conta_hijo += 1;

            // Iteramos sobre las transacciones nieto separated_graph.child_grandchild_edges
            if let Some(grandchildrens) = separated_graph.child_grandchild_edges.get(child_id) {
                conta_nieto = 1;
                for grandchildren in grandchildrens {

                    tx_mempool_data = mempool_data.get_entry(grandchildren);
        
                    vsize = match tx_mempool_data {
                        Some(vec) => vec[0].clone(),
                        None => String::from("Default Value"), // Replace "Default Value" with a suitable default
                    };
                    weight = match tx_mempool_data {
                        Some(vec) => vec[1].clone(),
                        None => String::from("Default Value"), // Replace "Default Value" with a suitable default
                    };

                    transactions.push_str(&format!("<p class='tx-nieto'>&nbsp;&nbsp;&nbsp;&nbsp; {}:Tx nieto: {:?} ( vsize: {} - weight: {} )</p>",conta_nieto, grandchildren, vsize, weight));
                    conta_todo += 1;
                    conta_nieto += 1;
                }
            }
        }
    }
    
    transactions.push_str(&format!("\n<p > Total líneas listado  {} : </p>", conta_todo));

    // Empaquetando el contenido HTML como una respuesta
    let html_output = format!("<html><body>{}</body></html>", transactions);
    HtmlContent(html_output)

}

// Transacciones descendientes en formato HTML
#[get("/get_descen_json")]
pub fn get_descen_json( separated_graph: &State<Arc<Mutex<SeparatedTxGraph>>>,
                    mempool_data: &State<Arc<Mutex<MempoolData>>>  ) -> Result<JsonResponse, rocket::response::Debug<serde_json::Error>>  {
    
    let separated_graph = separated_graph.lock().unwrap();
    let mempool_data = mempool_data.lock().unwrap();

    let mut vsize_p: String;
    let mut weight_p: String;
    let mut vsize_h: String;
    let mut weight_h: String;
    let mut vsize_n: String;
    let mut weight_n: String;

    let mut tx_mempool_data: Option<&Vec<String>>;

    let mut transactions = Vec::new();
    let mut parent_object: serde_json::Value ;


    // Iteramos sobre las transacciones padre separated_graph.parent_child_edges
    for (parent_id, children) in separated_graph.parent_child_edges.iter() {

        tx_mempool_data = mempool_data.get_entry(parent_id);
        
        vsize_p = match tx_mempool_data {
            Some(vec) => vec[0].clone(),
            None => String::from("Default Value"), // Replace "Default Value" with a suitable default
        };
        weight_p = match tx_mempool_data {
            Some(vec) => vec[1].clone(),
            None => String::from("Default Value"), // Replace "Default Value" with a suitable default
        };

        parent_object = json!({});

        // Iteramos sobre las transacciones hijo children
        for child_id in children {

            tx_mempool_data = mempool_data.get_entry(child_id);
        
            vsize_h = match tx_mempool_data {
                Some(vec) => vec[0].clone(),
                None => String::from("Default Value"), // Replace "Default Value" with a suitable default
            };
            weight_h = match tx_mempool_data {
                Some(vec) => vec[1].clone(),
                None => String::from("Default Value"), // Replace "Default Value" with a suitable default
            };

            // Iteramos sobre las transacciones nieto separated_graph.child_grandchild_edges
            if let Some(grandchildrens) = separated_graph.child_grandchild_edges.get(child_id) {
                
                //for grandchildren in grandchildrens {
                for grandchild_id in grandchildrens.iter(){

                    tx_mempool_data = mempool_data.get_entry(grandchild_id);
        
                    vsize_n = match tx_mempool_data {
                        Some(vec) => vec[0].clone(),
                        None => String::from("Default Value"), // Replace "Default Value" with a suitable default
                    };
                    weight_n = match tx_mempool_data {
                        Some(vec) => vec[1].clone(),
                        None => String::from("Default Value"), // Replace "Default Value" with a suitable default
                    };

                    parent_object = json!({
                        "0": [parent_id, vsize_p, weight_p],
                        "1": [child_id, vsize_h, weight_h],
                        "2": [grandchild_id, vsize_n, weight_n]
                    });

                }

            } else {

                    parent_object = json!({
                        "0": [parent_id, vsize_p, weight_p],
                        "1": [child_id, vsize_h, weight_h],
                    });     

            }

        }

        transactions.push(parent_object);

    }
    
    let stringified_json = serde_json::to_string(&json!({"transacciones": transactions}))?;
    Ok(JsonResponse(stringified_json))

}

