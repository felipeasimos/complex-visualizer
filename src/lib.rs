use std::mem::swap;

use plotters::{
    chart::{ChartBuilder, ChartContext},
    coord::{Shift, types::RangedCoordf64},
    prelude::{Cartesian2d, Circle, DrawingArea, EmptyElement, IntoDrawingArea},
    series::{LineSeries, PointSeries},
    style::{BLACK, FontDesc, GREEN, RED, WHITE},
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
    pub chart_type: ChartType,
    pub vector1: Point,
    pub vector2: Point,
}

/// Result of screen to chart coordinates conversion.
#[derive(Copy, Clone)]
#[wasm_bindgen]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl From<Point> for (f64, f64) {
    fn from(value: Point) -> Self {
        (value.x, value.y)
    }
}

#[wasm_bindgen]
impl Point {
    pub fn init(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn translate(&self, other: Point) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    pub fn rotate(&self, angle: f64) -> Self {
        Self {
            x: self.x * angle.cos() - self.y * angle.sin(),
            y: self.x * angle.sin() + self.y * angle.cos(),
        }
    }
    pub fn scale(&self, scale: f64) -> Self {
        Self {
            x: self.x * scale,
            y: self.y * scale,
        }
    }
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0.0, y: 0.0 }
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

#[derive(Copy, Clone, Debug)]
#[wasm_bindgen]
pub enum ChartType {
    ComplexTranslate,
    ComplexRotate,
    ComplexScale,
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
        let convert = Self::generate_chart(
            viewport,
            root.clone(),
            Point::default(),
            Point::default(),
            ChartType::ComplexTranslate,
        )
        .map_err(|err| err.to_string())?;
        Ok(Chart {
            viewport: viewport.clone(),
            drawing_area: root,
            convert,
            chart_type: ChartType::ComplexTranslate,
            vector1: Point::default(),
            vector2: Point::default(),
        })
    }

    pub fn get_viewport(&self) -> Rect {
        self.viewport
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
        self.convert = Self::generate_chart(
            self.viewport,
            self.drawing_area.clone(),
            self.vector1.clone(),
            self.vector2.clone(),
            self.chart_type,
        )
        .map_err(|err| err.to_string())?;
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

    fn generate_chart(
        viewport: Rect,
        drawing_area: DrawingArea<CanvasBackend, Shift>,
        vector1: Point,
        vector2: Point,
        chart_type: ChartType,
    ) -> DrawResult<Box<impl Fn((i32, i32)) -> Option<(f64, f64)>>> {
        let mut chart = Self::generate_2d_chart(viewport, drawing_area)?;

        chart.draw_series(PointSeries::of_element(
            vec![vector1.into()],
            5,
            &RED,
            &|c, s, st| {
                return Circle::new(c, s, st.filled()); // you could customize here
            },
        ))?;

        let result = match chart_type {
            ChartType::ComplexTranslate => vector1.translate(vector2).into(),
            ChartType::ComplexRotate => vector1.rotate(vector2.x).into(),
            ChartType::ComplexScale => vector1.scale(vector2.x).into(),
        };
        chart.draw_series(PointSeries::of_element(
            vec![result],
            5,
            &GREEN,
            &|c, s, st| {
                return Circle::new(c, s, st.filled()); // you could customize here
            },
        ))?;
        Ok(Box::new(chart.into_coord_trans()))
    }
}
