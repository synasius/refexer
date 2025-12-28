use eframe::egui;
use eframe::egui::Response;
use egui_plot::{Legend, Line, Plot, PlotPoint, PlotPoints};

pub struct BorrowPointsExample {
    pub(crate) points: Vec<PlotPoint>,
}

impl Default for BorrowPointsExample {
    fn default() -> Self {
        Self { points: Vec::new() }
    }
}

impl BorrowPointsExample {
    pub fn show_plot(&self, ui: &mut egui::Ui) -> Response {
        Plot::new("My Plot")
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                plot_ui.line(Line::new("curve", PlotPoints::Borrowed(&self.points)).name("curve"));
            })
            .response
    }
}
