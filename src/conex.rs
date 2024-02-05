// Importación de macros y dependencias externas
use bitcoincore_rpc::{Auth,Client};

const USER:&str = "userX";
const PWS:&str  = "wsx";

// Conexión con el nodo Bitcoin Core
pub fn conex() -> Client {
    let rpc_url  = "http://localhost:8332";
    let rpc_auth = Auth::UserPass(USER.to_string(), PWS.to_string());
    let client = Client::new(rpc_url, rpc_auth).expect("Error to connect Bitcoin Core");
    return client;
}
     