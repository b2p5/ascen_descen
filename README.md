
# Monitoreo de los Ascen y Descen de la Mempool Bitcoin.

Este proyecto utiliza el framework Rocket en Rust para crear un servidor web que monitorea y procesa datos en tiempo real de la mempool de Bitcoin. Proporciona una API REST para acceder a información variada sobre las transacciones en la mempool, como transacciones totales, pesos de transacciones, y más. Por último, muestra los resultados utilizando la librería gráfica de P5.js

## Características

- Monitoreo en tiempo real de la mempool de Bitcoin.
- API REST para acceder a información de la mempool.
- Visualización de datos con P5.js para mostrar información en forma gráfica.
- Gestión de conexiones a un nodo de Bitcoin Core para obtener datos directamente de la blockchain.

## Estructura del Proyecto

- `main.rs`: Archivo principal que configura y lanza el servidor Rocket.
- `mempool_data.rs`, `mempool_processor.rs`, `conex.rs`, `apis.rs`: Módulos que manejan la lógica de procesamiento de datos, conexión con el nodo de Bitcoin Core, y definición de endpoints de la API.
- `Cargo.toml`: Define las dependencias del proyecto.
- `sketch.js`: Script de P5.js para visualización de datos en el frontend.
- Directorios `static` y `assets`: Contienen archivos estáticos y recursos para el frontend.

## Requisitos Previos

- Rust y Cargo instalados.
- Acceso a un nodo de Bitcoin Core con el RPC habilitado.
- [Opcional] Configuración de CORS si se requiere acceso desde dominios cruzados.

## Instalación

1. Clona este repositorio en tu sistema local.
2. Navega al directorio del proyecto.
3. Ejecuta `cargo build` para compilar el proyecto.

## Configuración

Configura el acceso al nodo de Bitcoin Core en `conex.rs` con tus credenciales RPC:

```rust
// Ejemplo de configuración
fn conex() -> Client {
    Client::new("http://direccion_del_nodo:puerto", Auth::UserPass("usuario".to_string(), "contraseña".to_string())).unwrap()
}
```

## Ejecución

Para iniciar el servidor, ejecuta:

```bash
cargo run
```

El servidor estará disponible en `http://localhost:8000`.

## Endpoints de la API

La API proporciona los siguientes endpoints para acceder a los datos de la mempool:

- `/api/index`: Muestra la información de la mempool.
- `/api/mempool`: Información general de la mempool.
- `/api/weight_tx`: Peso de las transacciones.
- `/api/range_weights`: Rangos de peso de las transacciones.
- `/api/ascen_descen_for_txid`: Ascen/Descen para una transacción.
- `/api/mempool_var_info`: Información varia sobre la mempool.

## Contribuir

Si deseas contribuir al proyecto, considera hacer fork y enviar tus pull requests. Asegúrate de seguir las guías de estilo de Rust y de documentar adecuadamente tus cambios.

