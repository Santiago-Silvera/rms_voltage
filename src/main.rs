#![allow(warnings)]
use std::{error::Error, io, process};

// El struct Row nos permite separar cada fila del .csv en distintos elementos
// para trabajar con valores atomicos.
#[derive(serde::Deserialize, Debug)]
struct Row {
    // el unico elemento que realmente necesitamos es Voltage pero para que el
    // header y el struct coincidan debemos colocar todos los elementos como corresponde
    Time: String,
    Voltage: String,
    PeakDetect: String,
    Time2: String,
    Math: String,
}

// Esta es la funcion mas importante. No hay que prestar mucha atencion al tipo de valor devolvido.
fn read_csv() -> Result<(), Box<dyn Error>> {
    // Creamos un lector.
    let mut rdr = csv::Reader::from_reader(io::stdin());

    // Aqui creamos el header, que sus elementos deben coincidir con los elementos del struct.
    // TODO: assert_eq!(,); entre header y struct atrributes
    let header = rdr.headers()?.clone();

    // debug info
    println!("{:?}", header);

    // counter de n y de la sumatoria
    // nosotros sabemos que n = 7 millones pero por las dudas es mejor hacerlo asi
    let mut n = 0.0;
    let mut v_sum: f32 = 0.0;

    // Aqui iteramos sobre cada fila
    for record in rdr.records() {
        // Descomponemos la fila en el struct Row
        let row: Row = record?.deserialize(Some(&header))?;

        v_sum += f32::powf(row.Voltage.parse::<f32>().unwrap(), 2.0);
        n += 1.0;

        // Debug info. WARNING: no descomentar si se va a utilizar el archivo grande, el tiempo de
        // ejecucion aumenta significativamente.

        // println!(
        //     "Time: {:?} \n\tVoltage: {:?}",
        //     row.Time.parse::<f32>().unwrap(),
        //     row.Voltage.parse::<f32>().unwrap()
        // );
    }
    let result = f32::sqrt((1.0 / n) * v_sum);
    println!("{:?}", result);

    // devolvemos ok a main
    Ok(())
}

fn main() {
    println!("Starting program");
    use std::time::Instant;
    let now = Instant::now();

    // ejecutamos nuestra funcion, si algo sale mal, este codigo lo atrapa y cierra el proceso.
    if let Err(err) = read_csv() {
        println!("error running example: {}", err);
        process::exit(1);
    }

    // mostramos el tiempo tardado
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
