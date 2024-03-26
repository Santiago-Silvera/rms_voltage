#![allow(warnings)]
use core::f32;
use std::{error::Error, io, process};

use plotters::{backend::BitMapBackend, drawing::IntoDrawingArea, prelude::*};

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

fn cheat_rms() -> Result<(), Box<dyn Error>> {
    // Creamos un lector.
    let mut rdr = csv::Reader::from_reader(io::stdin());

    // Aqui creamos el header, que sus elementos deben coincidir con los elementos del struct.
    // TODO: assert_eq!(,); entre header y struct atrributes
    let header = rdr.headers()?.clone();

    // debug info
    println!("{:?}", header);

    let mut max_volt: f32 = 0.0;

    // Aqui iteramos sobre cada fila
    for record in rdr.records() {
        // Descomponemos la fila en el struct Row
        let row: Row = record?.deserialize(Some(&header))?;
        if row.Voltage.parse::<f32>()? > max_volt {
            max_volt = row.Voltage.parse::<f32>()?;
        }
    }
    let result = max_volt / (f32::sqrt(2.0));
    println!("{:?}", result);

    // devolvemos ok a main
    Ok(())
}

fn graph_data() -> Result<(), Box<dyn Error>> {
    //Create the canvas
    let root = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();
    let _ = root.fill(&WHITE);
    let root = root.margin(30, 5, 50, 30);

    // Make the chart
    let mut chart = ChartBuilder::on(&root)
        .caption("Data", ("sans-serif", 40).into_font())
        .x_label_area_size(100)
        .y_label_area_size(1)
        .build_cartesian_2d(-2E-2f32..2E-2f32, -20f32..20f32)?;

    // Configure the chart
    chart
        .configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    let mut rdr = csv::Reader::from_reader(io::stdin());
    let header = rdr.headers()?.clone();

    let mut graphing_vec = Vec::new();
    for record in rdr.records() {
        // Descomponemos la fila en el struct Row
        let row: Row = record?.deserialize(Some(&header))?;

        // add the point
        let gp = (row.Time.parse::<f32>()?, row.Voltage.parse::<f32>()?);
        graphing_vec.push(gp);
    }
    // make the graph based on all the recorded ponints.
    chart.draw_series(PointSeries::of_element(
        graphing_vec,
        1,
        &RED,
        &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()); // At this point, the new pixel coordinate is established
        },
    ))?;
    root.present()?;
    Ok(())
}

// Esta es la funcion mas importante. No hay que prestar mucha atencion al tipo de valor devolvido.
fn find_true_rms() -> Result<(), Box<dyn Error>> {
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
    if let Err(err) = cheat_rms() {
        println!("error running example: {}", err);
        process::exit(1);
    }

    // mostramos el tiempo tardado
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
