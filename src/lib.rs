use plotters::{
    chart::{ChartBuilder, ChartContext},
    coord::{Shift, types::RangedCoordf64},
    prelude::{Cartesian2d, DrawingArea, IntoDrawingArea},
    series::{LineSeries, PointSeries},
    style::{BLACK, FontDesc, WHITE},
};
use plotters_canvas::CanvasBackend;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use lol_alloc::{FreeListAllocator, LockedAllocator};

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: LockedAllocator<FreeListAllocator> =
    LockedAllocator::new(FreeListAllocator::new());

/// Type alias for the result of a drawing function.
pub type DrawResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Type used on the JS side to convert screen coordinates to chart
/// coordinates.
#[wasm_bindgen]
pub struct Chart {
    convert: Box<dyn Fn((i32, i32)) -> Option<(f64, f64)>>,
    viewport: Rect,
    drawing_area: DrawingArea<CanvasBackend, Shift>,
    chart_type: ChartType,
    vector1: Option<Point>,
    vector2: Option<Point>,
}

/// Result of screen to chart coordinates conversion.
#[wasm_bindgen]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Point {
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    pub fn init(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// (x, y) points to the bottom left corner in chart coords
#[derive(Copy, Clone)]
#[wasm_bindgen]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[wasm_bindgen]
pub enum Operation {
    Translation,
    Rotation,
    Scale,
}

#[wasm_bindgen]
pub enum ChartType {
    Complex,
}

#[wasm_bindgen]
impl Chart {
    pub fn new(canvas_id: &str) -> Result<Chart, JsValue> {
        let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
        let root = backend.into_drawing_area();
        let viewport = Rect {
            x: -100.0,
            y: -100.0,
            width: 200.0,
            height: 200.0,
        };
        let convert =
            Self::generate_chart_complex(viewport, root.clone()).map_err(|err| err.to_string())?;
        Ok(Chart {
            viewport: viewport.clone(),
            drawing_area: root,
            convert,
            chart_type: ChartType::Complex,
            vector1: None,
            vector2: None,
        })
    }

    pub fn get_viewport(&self) -> Rect {
        self.viewport
    }

    pub fn set_chart_type(&mut self, chart_type: ChartType) -> () {
        self.chart_type = chart_type;
    }

    pub fn translate(&mut self, translation: Point) -> () {
        self.viewport.x += translation.x;
        self.viewport.y += translation.y;
    }

    pub fn scale(&mut self, scale: Point) -> () {
        self.viewport.width *= scale.x;
        self.viewport.height *= scale.y;
    }

    pub fn update(&mut self) -> Result<(), JsValue> {
        self.convert = match self.chart_type {
            ChartType::Complex => {
                Self::generate_chart_complex(self.viewport, self.drawing_area.clone())
                    .map_err(|err| err.to_string())?
            }
        };
        self.drawing_area.present().map_err(|err| err.to_string())?;
        Ok(())
    }

    /// This function can be used to convert screen coordinates to
    /// chart coordinates.
    pub fn coord(&self, x: i32, y: i32) -> Option<Point> {
        (self.convert)((x, y)).map(|(x, y)| Point { x, y })
    }

    fn generate_2d_chart<'a>(
        viewport: Rect,
        drawing_area: DrawingArea<CanvasBackend, Shift>,
    ) -> DrawResult<ChartContext<'a, CanvasBackend, Cartesian2d<RangedCoordf64, RangedCoordf64>>>
    {
        let font: FontDesc = ("sans-serif", 20.0).into();
        drawing_area.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&drawing_area)
            .margin(30u32)
            .caption(format!("complex numbers"), font)
            .x_label_area_size(30u32)
            .y_label_area_size(30u32)
            .build_cartesian_2d(
                viewport.x..viewport.x + viewport.width,
                viewport.y..viewport.y + viewport.height,
            )?;

        chart.configure_mesh().x_labels(3).y_labels(3).draw()?;
        return Ok(chart);
    }

    fn generate_chart_complex(
        viewport: Rect,
        drawing_area: DrawingArea<CanvasBackend, Shift>,
    ) -> DrawResult<Box<impl Fn((i32, i32)) -> Option<(f64, f64)>>> {
        let mut chart = Self::generate_2d_chart(viewport, drawing_area)?;

        let resolution = 1000;
        let interval_shift = (viewport.width) / (resolution as f64);

        chart.draw_series(LineSeries::new(
            (0..=resolution)
                .map(|x| interval_shift * (x as f64) + viewport.x)
                .map(|x| (x, x.powf(2.0)))
                .filter(|(x, y)| *y < viewport.y + viewport.height && *y > viewport.y),
            &BLACK,
        ))?;
        Ok(Box::new(chart.into_coord_trans()))
    }
}
