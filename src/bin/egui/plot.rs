use eframe::egui;
use egui_plot::{Line, Plot, PlotPoint, PlotPoints};

pub struct WaveformPlot {
    pub(crate) points: Vec<PlotPoint>,
}

impl Default for WaveformPlot {
    fn default() -> Self {
        Self { points: Vec::new() }
    }
}

impl WaveformPlot {
    pub fn show_plot(&self, ui: &mut egui::Ui) -> egui::Response {
        Plot::new("Waveform")
            .show(ui, |plot_ui| {
                plot_ui.line(Line::new("waveform", PlotPoints::Borrowed(&self.points)));
            })
            .response
    }
}
