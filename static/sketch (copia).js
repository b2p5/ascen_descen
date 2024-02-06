
let mempoolData_1 = null;
let mempoolData_2 = null;
let mempoolData_3 = null;

let range_weight1 = 0;
let range_weight2 = 0;
let txs_work = "";

let interval_1 = 0;
const TIME_INTERVAL = 5000;

//Inicializar vector para contener los rectangulos de rangos de pesos
let rects_range = [];
let rects_data = [];
let txs_data_descen = [];
let txs_data_ascen = [];


let switch_process = "rangos";

let radio_ascen_descen = 30;

let image_row_arrow = null;


function setup() {
  createCanvas(720, 650);
  //Trabaja en radianes
  //angleMode(

  image_row_arrow = loadImage('/static/image_row_arrow.png');
  
  // Llamar a la función getmempoolData_1 cada 5 segundos
  interval_1 = setInterval(getmempoolData_1, TIME_INTERVAL);
  getmempoolData_1();

  fill(255, 204);
  frameRate(3);

  // loop(0.5);

}

function draw() {
    background(0);

    if (mempoolData_3) {
      txs_data_descen = [];
      txs_data_ascen = [];

      fill(255);
      rect(0, 0, 720, 40);
      image (image_row_arrow, 0, 0, 40, 40);

      // Centro del canvas
      let x_0 = width / 2;
      let y_0 = height / 2;

      fill(255, 255, 0);
      text ('ASCEN ', 170, 150);
      fill(255, 0, 255);
      text ('DESCEN ', 470, 150);

      let descen = mempoolData_3.descen;
      
      Object.keys(descen).forEach(key => {
        num_descen = descen[key].length;
        let radianes_des = (1*PI/num_descen) 
        
        for (let i = 0; i < descen[key].length; i++) {
          let angulo = (radianes_des*i) - (PI/2);
          let x = x_0 + 100 + (100 * cos(angulo));
          let y = y_0 + (100 * sin(angulo));

          fill(255, 255, 255);
          stroke(126);
          line(x_0, y_0, x, y);
          
          txs_data_descen.push({ txid: key,
                          descen : descen[key][i],
                          x: x, y: y, r: radio_ascen_descen,
                        });
        }


      });

      let ascen = mempoolData_3.ascen;
      
      Object.keys(ascen).forEach(key => {
        num_ascen = ascen[key].length;
        let radianes_asc = (1*PI/num_ascen) ;
        
        for (let i = 0; i < ascen[key].length; i++) {
          let angulo = (radianes_asc*i) + (PI/2);
          let x = x_0 - 100 + (100 * cos(angulo));
          let y = y_0 + (100 * sin(angulo));

          fill(255, 255, 255);
          stroke(126);
          line(x_0, y_0, x, y);

          txs_data_ascen.push({ txid: key,
                          ascen : ascen[key][i],
                          x: x, y: y, r: radio_ascen_descen,
                        });
        }

      });

      // fill(255, 255, 255);
      // let txid_red = txs_data[0].txid;
      // text ('Tx: ' + txid_red, 10, 20);
      // for (let i = 0; i < txs_data.length; i++) {
        
      //   let x = 10;
      //   let y = 40 + i*20;
      //   if (txs_data[i].descen) {
      //     text ('   Descen: ' + txs_data[i].descen, x+50, y + 20);
      //   } else if (txs_data[i].ascen) {
      //     text ('   Ascen: ' + txs_data[i].ascen, x+50, y + 20 );
      //   }
      // }

      // Dibujar un círculo en el centro del canvas que es la Tx de trabajo

      // Dibujar un círculo en el centro del canvas que es la Tx de trabajo
      let work_tx = (txs_work.tx).substring(0, 10)+ '...' + (txs_work.tx).substring((txs_work.tx).length-10, (txs_work.tx).length);
      let r = 50;
      circle(x_0, y_0, r);
      // fill(255, 255, 255);
      // text ('Tx: ' + work_tx, x_0+20, y_0-10);
      if (isMouseInsideCircle(x_0, y_0, r)) {
        fill(127);
        stroke(126);
        rect(x_0+10, y_0-15, 200, 30, 10);
        fill(255, 255, 255);
        let redu_key = (txs_work.tx).substring(0, 10)+ '...' + (txs_work.tx).substring((txs_work.tx).length-10, (txs_work.tx).length);
        text ('Tx: ' + redu_key, x_0+20, y_0+4);
      } 

      // Dibujar los circulos de los descen
      radianes_des = (1*PI/num_descen) ;
      for (let i = 0; i < num_descen; i++) {
        let angulo = (radianes_des*i) - (PI/2);
        //console.log(angulo);
        let x = x_0 + 100 + (100 * cos(angulo));
        let y = y_0 + (100 * sin(angulo));
        let r = radio_ascen_descen;
        fill(255, 0, 255);
        circle(x, y, r);
        //text ('Tx: ' + txs_data[i].descen, x+20, y-10);
        if (isMouseInsideCircle(x, y, r)) {
          fill(127);
          stroke(126);
          rect(x+10, y-15, 200, 30, 10);
          fill(255, 255, 255);
          let redu_key = (txs_data_descen[i].descen).substring(0, 10)+ '...' + (txs_data_descen[i].descen).substring((txs_data_descen[i].descen).length-10, (txs_data_descen[i].descen).length);
          text ('Tx: ' + redu_key, x+20, y+4);
        } 
      }

      // Dibujar los circulos de los ascen
      radianes_asc = (1*PI/num_ascen) ;
      for (let i = 0; i < num_ascen; i++) {
        let angulo = (radianes_asc*i) + (PI/2);
        // console.log(angulo);
        let x = x_0 - 100 + (100 * cos(angulo));
        let y = y_0 + (100 * sin(angulo));
        let r = radio_ascen_descen;
        fill(255, 255, 0);
        circle(x, y, r);
        //text ('Tx: ' + txs_data[i].ascen, x+20, y-10);
        if (isMouseInsideCircle(x, y, r)) {
          fill(127);
          stroke(126);
          rect(x+10, y-15, 200, 30, 10);
          fill(255, 255, 255);
          let redu_key = (txs_data_ascen[i].ascen).substring(0, 10)+ '...' + (txs_data_ascen[i].ascen).substring((txs_data_ascen[i].ascen).length-10, (txs_data_ascen[i].ascen).length);
          text ('Tx: ' + redu_key, x+20, y+4);
        } 
      }

    /////////////////////////////////////////////////////////////////
    } else if (mempoolData_2) {
      rects_data = [];

      fill(255);
      rect(0, 0, 720, 40);
      image (image_row_arrow, 0, 0, 40, 40);

      let proyeccion =  (range_weight2 - range_weight1)/(height-40) ;
      Object.keys(mempoolData_2).forEach(key => {
        fill(255, 255, 255);
        let x = mempoolData_2[key].fees_base * 10000000;
        let y = ((mempoolData_2[key].weight)*proyeccion) + 40;
        let r = 10;
        circle(x, y, r);

        // Almacenar los datos de los circulos
        rects_data.push({x: x, y: y, r: r, 
                          tx : key,
                          fees_base: mempoolData_2[key].fees_base, 
                          weight: mempoolData_2[key].weight, 
                        });

        if (isMouseInsideCircle(x, y, r)) {
          fill(127);
          stroke(126);
          rect(x+10, y-25, 200, 50, 10);
          fill(255, 255, 255);
          let redu_key = key.substring(0, 10)+ '...' + key.substring(key.length-10, key.length);
          text ('Tx: ' + redu_key, x+20, y-10);
          text ('Fees: ' + mempoolData_2[key].fees_base, x+20, y+5);
          text ('Weight: ' + mempoolData_2[key].weight, x+20, y+20);
        } 

      });


    /////////////////////////////////////////////////////////////////
    } else if (mempoolData_1) {
        rects_range = [];
        let x = 0;
        let y = 0;
        let w = width;
        let weight_prev = 1;
        Object.keys(mempoolData_1).forEach(key => {
          let h = key/10;

          fill(127);
          rect(x, y, w, h);

          fill(255, 255, 255);
          text (mempoolData_1[key] + ' Txs.', x+110, y+20);
          text ('con pesos entre '+ weight_prev + ' y ' + key, x+170, y+20);

          // Almacenar los datos de los rectangulos
          rects_range.push({x: x, y: y, w: w, h: h,
                          weight_prev: weight_prev, weight: key, 
                          txs: mempoolData_1[key]});

          y += h+10;     
          weight_prev = int(key) + 1;     

        });
   
    }

}


function getmempoolData_1() {
  fetch('http://127.0.0.1:8000/get_range_weights_json/')
    .then(response => response.json())
    .then(data => {
      mempoolData_1 = data;
      mempoolData_2 = null;
      mempoolData_3 = null;
    })
    .catch(error => console.error('Error al obtener los datos:', error));
}

function getMempoolData_2(weight1, weight2) {
  let url = 'http://127.0.0.1:8000/get_txs_range_weights_json/'+weight1+'/'+weight2;
  fetch(url)
    .then(response => response.json())
    .then(data => {
      mempoolData_2= data;
      mempoolData_1 = null;
      mempoolData_3 = null;
    })
    .catch(error => console.error('Error al obtener los datos:', error));
}

function getMempoolData_3(tx) {
  let url = 'http://127.0.0.1:8000/get_ascen_descen_for_txid_json/'+tx;
  fetch(url)
    .then(response => response.json())
    .then(data => {
      mempoolData_3= data;
      mempoolData_1 = null;
      mempoolData_2 = null;
    })
    .catch(error => console.error('Error al obtener los datos:', error));
}


// Si dblclick el ratón, llamo a la función getMempoolData_2
window.addEventListener("click", function(e) {
  // console.log(switch_process);
  x = mouseX;
  y = mouseY;

  if (switch_process == "rangos") {
    let range = rects_range.find(rect => isMouseInsideRect(rect.x, rect.y, rect.w, rect.h));
    if (range) {
      clearInterval(interval_1);
      range_weight1 = range.weight_prev;
      range_weight2 = range.weight;
      switch_process = "txs" ;
      getMempoolData_2(range.weight_prev, range.weight);

    }
  
  } else if (switch_process == "txs") {
    let ascen_descen = isMouseInsideRect(0,0,40,40);
    if (ascen_descen) {
      range_weight1 = null;
      range_weight2 = null;
      txs_work = null;
      switch_process = "rangos" ;
      getmempoolData_1();
      interval_1 = setInterval(getmempoolData_1, TIME_INTERVAL);
    }

    let tx = rects_data.find(rect => isMouseInsideCircle(rect.x, rect.y, rect.r));
    if (tx) {
      clearInterval(interval_1);
      txs_work = tx;
      switch_process = "ascen_descen" ;
      getMempoolData_3(tx.tx);
    }
    
  } else if (switch_process == "ascen_descen") {
    let ascen_descen = isMouseInsideRect(0,0,40,40);
    if (ascen_descen) {
      range_weight1 = null;
      range_weight2 = null;
      txs_work = null;
      switch_process = "rangos" ;
      getmempoolData_1();
      interval_1 = setInterval(getmempoolData_1, TIME_INTERVAL);
    }
  } else {
    console.log("Error en el switch_process");
  }
  
}); 


// Funcion para determinar si el raton esta posicionado dentro de un circulo
function isMouseInsideCircle(x, y, r) {
  let d = dist(mouseX, mouseY, x, y);
  if (d < r) {
    return true;
  } else {
    return false;
  }
}

// Funcion para determinar si el Click del raton esta dentro de un rectangulo
function isMouseInsideRect(x, y, w, h) {
  if (mouseX > x && mouseX < x + w && mouseY > y && mouseY < y + h) {
    return true;
  } else {
    return false;
  }
}
