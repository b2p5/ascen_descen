
let datosExternos = [];

let balls       = [];
let numBalls    = 0;
let spring      = 0.005;
let gravity     = 0.19;
let friction    = -0.1;
let diam        = 10;

let y_canva     = 650;
let y_min       = 1000;
let y_incre     = 100;

let ultimo_tx   = 0;
let ultimo_block   = 0;



async function setup() {
  
  createCanvas(720, y_canva);

  // Get toda la mempool
  await obtenerDatosExternos( "1" );
  let num_balls = datosExternos.length;

  y_canva += int(num_balls/1000)*y_incre;

  createCanvas(720, y_canva);

  console.log('num_balls: ' + num_balls);

  // Get zona para representar la mempool
  let x_zone_min = 0;
  let x_zone_max = width;
  let y_zone_min = 2*(y_canva/3);
  let y_zone_max = y_canva;

  let bolasPorFila = y_zone_max / diam;
  let separacion_x = 1;
  let separacion_y = 1;
  

  // Colocar num_balls bolas entre x_zone_min y x_zone_max y entre y_zone_min y y_zone_max
  // con una distancia mínima entre ellas de diam
  let x = 0;
  let y = y_zone_max - diam ;
  let k = 0;
  for (let i = 1; i <= num_balls; i++) {

    x = x_zone_min +  diam * (k++)  + separacion_x;

    balls = ges_balls(x, y, diam, i, balls, false);

    if (x > x_zone_max - diam) {
      y -= (diam + separacion_y);
      k = 0;
    }

  }

  setTimeout(function() {
    obtenerDatosExternos( "2" );
    new_balls = datosExternos.length;

    obtenerDatosExternos( "3" );
    ultimo_tx == datosExternos[0]

  }, 5000);


  fill(255, 204);
  frameRate(30);
  //noLoop();

}

async function draw() {
  background(0);

  // Si todas las bolas están paradas, se crean nuevas
  let stoped = true;
  balls.forEach(ball => {
    if(!ball.stoped) {
        stoped = false;
        //return;
    }
  });
  
  if (stoped) {
    // Get el último  tx de la mempool
    await obtenerDatosExternos( "3" );
    if (ultimo_tx == datosExternos[0]) {
      // esperaBloqueante(2000, balls)
      // console.log('2 segundos');

      // frameRate(30);

      return;
    }

    // frameRate(30);
     
    ultimo_tx = datosExternos[0];
    // Get incremento de txs de la mempool
    await obtenerDatosExternos( "2" );
    // Número de nuevas txs recibidas
    let new_balls = datosExternos.length;
    console.log('new_balls: ' + new_balls);
    // Número de txs en en balls
    let n_balls = balls.length;
    for (let i = n_balls; i < n_balls + new_balls; i++) {

      balls = ges_balls(random(width), random(height/8), diam, i, balls, false);
      numBalls++;

    }
      
  }


  // Formatea numBalls con punto de miles
  numBalls = balls.length-1;
  numBalls_format = numBalls.toLocaleString('de-DE');
  //new_balls_format = new_balls.toLocaleString('de-DE');
  let texto = 'Número Txs de la mempool con descendientes: ' + numBalls_format +
              ' - ( nuevas Tx: ' + '0' + ' )';
  text(texto, 5, 10);
  // Cursor dentro de objeto
  // cursorDentroObj(balls);
  // balls.forEach(ball => {
  //   ball.collide();
  //   ball.move();
  //   ball.display();
  // });
  balls.forEach(ball => {
    ball.collide();
    ball.move();
    ball.display();
  });

}

function ges_balls(x, y, diam, i, balls, stoped) {
  balls[i] = new Ball(
    x,
    y,
    diam,
    i,
    balls,
    stoped
  );

  return balls;
}

function esperaBloqueante(milisegundos, balls) {
  const inicio = new Date().getTime();
  let ahora = inicio;
  while (ahora - inicio < milisegundos) {
    balls.forEach(ball => {
      ball.display();
    });
    ahora = new Date().getTime();
  }
}

class Ball {
  constructor(xin, yin, din, idin, oin, stoped) {
    this.x = xin;
    this.y = yin;
    this.vx = 0;
    this.vy = 0;
    this.diameter = din;
    this.id = idin;
    this.others = oin;
    this.stoped = stoped;
  }

  collide() {
    for (let i = 1 ; i <= numBalls; i++) {

        if (!this.stoped) { 

          let dx = this.others[i].x - this.x;
          let dy = this.others[i].y - this.y;
          let distance = sqrt(dx * dx + dy * dy);
          let minDist = this.others[i].diameter / 2 + this.diameter / 2;
          if (distance < minDist+10)  {
          
              if (this.others[i].stoped){
                  this.stoped = true;

                  if (this.y < y_min) y_min = this.y;

                  return;          
              }

              // let angle = atan2(dy, dx);
              // let targetX = this.x + cos(angle) * minDist;
              // let targetY = this.y + sin(angle) * minDist;
              // let ax = (targetX - this.others[i].x) * spring;
              // let ay = (targetY - this.others[i].y) * spring;
              // this.vx -= ax;
              // this.vy -= ay;
              // this.others[i].vx += ax;
              // this.others[i].vy += ay;
            
          }
        }
    }
  }

  move() {
     
    if(!this.stoped) {

        this.vy += gravity;
        this.x += this.vx;
        this.y += this.vy;

        if (this.x + this.diameter / 2 > width) {
          this.x = width - this.diameter / 2;
          this.vx *= friction;
        } else if (this.x - this.diameter / 2 < 0) {
          this.x = this.diameter / 2;
          this.vx *= friction;
        }
        if (this.y + this.diameter / 2 > height) {
          this.y = height - this.diameter / 2;
          this.vy *= friction;

          this.stoped = true;

        } else if (this.y - this.diameter / 2 < 0) {

            this.y = this.diameter / 2;
            this.vy *= friction;
        }
    }
  }

  display() {
    ellipse(this.x, this.y, this.diameter, this.diameter);
    // Imprime el id de la bola
    //fill(255);
    //text(this.id, this.x, this.y);
    
  }

}

/////////////////////////////////////////////////////////////////////////////
//Funciones
/////////////////////////////////////////////////////////////////////////////
function cursorDentroObj(miCursorDentroObjetos){ 
  
    for(let i=1; i <= miCursorDentroObjetos.length-1; i++) {  
    
        if ( estaDentroObjeto(  mouseX, mouseY, 
                                miCursorDentroObjetos[i].x, 
                                miCursorDentroObjetos[i].y, 
                                (miCursorDentroObjetos[i].diameter)/2) ){

            // Get value from stroke color select


            stroke(255 ,127, 0);
            //fill(255 ,127, 0);
            ellipse(  miCursorDentroObjetos[i].x, 
                      miCursorDentroObjetos[i].y, 
                      ((miCursorDentroObjetos[i].diameter)/2) + 20,
                      ((miCursorDentroObjetos[i].diameter)/2) + 20 );

            stroke(127 ,127, 127);
            //fill(127 ,127, 127);
            
            return;

        }//fin if ( estaDentroObjeto

    }//fin for(let i=0; i < miCursorDentroObjetos.arrObjetos.length; 

}//fin cursorDentroObj  


function estaDentroObjeto ( x1, y1, x2, y2 , radio) {

  return  (Math.sqrt((x1-x2)**2 + (y1-y2)**2) < (radio/2) ) ;

}//fin de function posicionado


async function obtenerDatosExternos( tipo ) {

    await fetch('http://127.0.0.1:8000/get_mempool_datos/'+ tipo ) // Reemplaza con la URL de tu fuente de datos
      .then(response => response.json())
      .then(data => {
        datosExternos = data; // Actualiza la variable con los nuevos datos
        
        //console.log('datosExternos: ' + datosExternos);
      })
      .catch(error => {
        console.error('Error al obtener los datos:', error);
      });
  
}

