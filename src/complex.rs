use crate::DrawResult;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;

pub fn draw(canvas_id: String, zoom: f64) -> DrawResult<impl Fn((i32, i32)) -> Option<(f64, f64)>> {
    let backend = CanvasBackend::new(canvas_id.as_str()).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 20.0).into();

    root.fill(&WHITE)?;

    let resolution = 1000;
    let limits = resolution as f64 / (2.0 * zoom);

    let mut chart = ChartBuilder::on(&root)
        .margin(30u32)
        .caption(format!("complex numbers"), font)
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d(-limits - 1f64..limits + 1f64, -limits - 1f64..limits + 1f64)?;

    chart.configure_mesh().x_labels(3).y_labels(3).draw()?;

    let interval_shift = (limits + limits) / (resolution as f64);

    chart.draw_series(LineSeries::new(
        (0..=resolution)
            .map(|x| interval_shift * (x as f64) - limits)
            .map(|x| (x, x.powf(2.0)))
            .filter(|(x, y)| *y < limits && *y > -limits),
        &BLACK,
    ))?;

    root.present()?;
    Ok(chart.into_coord_trans())
}
