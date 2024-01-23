async function fetchMempoolTxs() {
    fetch('http://127.0.0.1:8000/get_mempool_json', {
        method: 'GET',
        credentials: 'include' // Importante para enviar y recibir cookies
    })
    .then(response => response.text())
    .then(txs => {
        //console.log(txs);
        //document.getElementById('apiResult').innerHTML = JSON.stringify(txs);
    })
    .catch(error => console.error('Error:', error));
}

// Llamada inicial y luego cada 5 segundos
// fetchMempoolTxs();
setInterval(fetchMempoolTxs, 5000);


