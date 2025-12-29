use eframe::egui;
use egui_plot::{Line, Plot, PlotPoint, PlotPoints};

#[derive(Default)]
pub struct WaveformPlot {
    pub(crate) points: Vec<PlotPoint>,
}

impl WaveformPlot {
    pub fn show_plot(&self, ui: &mut egui::Ui) -> egui::Response {
        Plot::new("Waveform")
            .show(ui, |plot_ui| {
                plot_ui.line(Line::new("waveform", PlotPoints::Borrowed(&self.points)));
            })
            .response
    }

    pub fn set_data(&mut self, samples: &[f32]) {
        self.points.clear();

        for (i, &v) in samples.iter().enumerate() {
            self.points.push(PlotPoint::new(i as f64, v as f64));
        }
    }
}
