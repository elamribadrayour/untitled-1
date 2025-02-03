use anyhow::Result;
use plotters::prelude::*;

pub struct Plot;

impl Plot {
    pub fn fitness_step(
        fitnesses: &[f32],
        epochs: usize,
        smoothing_level: usize,
        data_dir: &str,
    ) -> Result<()> {
        let filename = format!("{}/fitness-smooth-{}.png", data_dir, smoothing_level);
        let root = BitMapBackend::new(&filename, (1024, 768)).into_drawing_area();

        root.fill(&WHITE)?;

        let mut smoothed_fitnesses = Vec::new();
        for i in 0..epochs as i32 {
            let mut sum = 0.0;
            for j in 0..smoothing_level as i32 {
                if (i - j) < 0 {
                    break;
                }
                sum += fitnesses[(i - j) as usize];
            }
            smoothed_fitnesses.push(sum / smoothing_level as f32);
        }

        let mut max_fitness = *smoothed_fitnesses
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        let mut min_fitness = *smoothed_fitnesses
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        max_fitness += max_fitness * 0.1;
        min_fitness -= min_fitness * 0.1;

        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .caption("Fitnesses", ("sans-serif", 40))
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Right, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(0..epochs, min_fitness..max_fitness)?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .x_labels(30)
            .max_light_lines(4)
            .draw()?;

        chart.draw_series(LineSeries::new(
            (0..epochs).map(|i| (i, smoothed_fitnesses[i])),
            &BLUE,
        ))?;

        Ok(())
    }

    pub fn fitness(fitnesses: &[f32], epochs: usize, data_dir: &str) -> Result<()> {
        let plots_dir = format!("{}/plots", data_dir);
        std::fs::create_dir_all(&plots_dir)?;
        let smoothing_levels = 1..=10;
        log::info!(
            "plotting fitnesses with smoothing levels: {:?}",
            smoothing_levels
        );
        smoothing_levels.for_each(|smoothing_level| {
            Plot::fitness_step(fitnesses, epochs, smoothing_level, &plots_dir).unwrap()
        });
        Ok(())
    }
}
