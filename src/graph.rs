use plotters::prelude::*;
use crate::preprocessing::Passenger;

pub fn create_scatter_plot(
    passengers: &[Passenger],
    output_file: &str,
    x_label: &str,
    y_label: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(output_file, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_max = passengers
        .iter()
        .map(|p| p.pclass)
        .fold(f64::MIN, |a, b| a.max(b));
    let y_max = passengers
        .iter()
        .map(|p| p.age)
        .fold(f64::MIN, |a, b| a.max(b));

    let mut chart = ChartBuilder::on(&root)
        .caption("Scatter plot", ("Arial", 40).into_font())
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..(x_max + 1.0), 0f64..(y_max + 10.0))?;

    chart
        .configure_mesh()
        .x_desc(x_label)
        .y_desc(y_label)
        .draw()?;

    for passenger in passengers {
        let survived = if passenger.survived == 1.0 { "Survived" } else { "Not Survived" };
        let color = if passenger.survived == 1.0 { &GREEN } else { &RED };
        chart.draw_series(PointSeries::of_element(
            vec![(passenger.pclass, passenger.age)],
            5,
            ShapeStyle::from(color).filled(),
            &|coord, size, style| {
                return EmptyElement::at(coord) + Circle::new((0, 0), size, style);
            },
        ))?;
    }

    Ok(())
}
