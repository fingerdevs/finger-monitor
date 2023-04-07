use plotters::{backend::RGBPixel, prelude::*};

use crate::{
    models::{Charts, FingerChart},
    ui::FingerState,
};

pub fn generate_charts(state: &FingerState) -> Result<Charts, Box<dyn std::error::Error>> {
    Ok(Charts {
        memory_chart: generate_memory_chart(state)?,
        cpu_chart: generate_memory_chart(state)?,
    })
}

fn generate_memory_chart(_state: &FingerState) -> Result<FingerChart, Box<dyn std::error::Error>> {
    let width = 640;
    let height = 480;
    let mut buffer: Vec<u8> = vec![0; width as usize * height as usize * 3];
    {
        let root = BitMapBackend::with_buffer(&mut buffer, (width, height)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption("y=x^2", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;

        chart.configure_mesh().draw()?;

        chart
            .draw_series(LineSeries::new(
                (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                &RED,
            ))?
            .label("y = x^2")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;
        root.present()?;
    }
    let chart = FingerChart {
        raw: buffer,
        height: height as usize,
        width: width as usize,
    };
    Ok(chart)
}
