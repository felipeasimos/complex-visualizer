use crate::DrawResult;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;

pub fn draw(canvas: HtmlCanvasElement) -> DrawResult<()> {
    let area = CanvasBackend::with_canvas_object(canvas)
        .unwrap()
        .into_drawing_area();
    area.fill(&BLACK)?;

    let x_axis = (-3.0..3.0).step(0.1);
    let z_axis = (-3.0..3.0).step(0.1);

    let mut chart =
        ChartBuilder::on(&area).build_cartesian_3d(x_axis.clone(), -3.0..3.0, z_axis.clone())?;
    chart.with_projection(|mut pb| {
        pb.yaw = 0.1;
        pb.pitch = 0.2;
        pb.scale = 0.8;
        pb.into_matrix()
    });
    chart.configure_axes().draw()?;

    chart.draw_series(
        SurfaceSeries::xoz(x_axis.values(), z_axis.values(), |x: f64, z: f64| {
            (x * x + z * z).cos()
        })
        .style(&BLUE.mix(0.2)),
    )?;

    chart.draw_series(LineSeries::new(
        (-100..100)
            .map(|y| y as f64 / 40.0)
            .map(|y| ((y * 10.0).sin(), y, (y * 10.0).cos())),
        &BLACK,
    ))?;

    Ok(())
}
