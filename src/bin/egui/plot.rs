use eframe::egui;
use eframe::egui::Response;
use egui_plot::{Legend, Line, Plot, PlotPoint, PlotPoints};

pub struct WaveformPlot {
    pub(crate) points: Vec<PlotPoint>,
}

impl Default for WaveformPlot {
    fn default() -> Self {
        Self { points: Vec::new() }
    }
}

impl WaveformPlot {
    pub fn show_plot(&self, ui: &mut egui::Ui) -> Response {
        Plot::new("Waveform")
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                plot_ui.line(Line::new("curve", PlotPoints::Borrowed(&self.points)).name("curve"));
            })
            .response
    }
}
