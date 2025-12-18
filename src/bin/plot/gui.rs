use anyhow::anyhow;
use eframe::egui;
use eframe::egui::Response;
use egui_plot::Legend;
use egui_plot::Line;
use egui_plot::Plot;
use egui_plot::PlotPoint;
use egui_plot::PlotPoints;
use refexer::synth::Synth;
use refexer::synth::presets::SoundType::HitHurt;
use refexer::synth::presets::SynthPreset;

pub struct BorrowPointsExample {
    points: Vec<PlotPoint>,
}

impl Default for BorrowPointsExample {
    fn default() -> Self {
        let points: Vec<[f64; 2]> = vec![[0.0, 1.0], [2.0, 3.0], [3.0, 2.0]];
        let points = points.iter().map(|p| PlotPoint::new(p[0], p[1])).collect();
        Self { points }
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

    #[expect(clippy::unused_self, reason = "required by the example template")]
    pub fn show_controls(&self, ui: &mut egui::Ui) -> Response {
        ui.scope(|_ui| {}).response
    }
}

#[derive(Default)]
pub struct AppWrapper {
    pub inner: BorrowPointsExample,
}

impl eframe::App for AppWrapper {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.inner.show_plot(ui);
        });
    }
}

fn main() -> anyhow::Result<()> {
    let mut plot_data = BorrowPointsExample { points: Vec::new() };

    // initialize the synth and the audio stream
    let mut preset = SynthPreset::new();
    let mut synth = Synth::new(preset.generate(HitHurt));
    synth.play_sample();

    let mut index = 0.0;
    while let Some(value) = synth.synth_sample() {
        plot_data.points.push(PlotPoint::new(index, value as f64));
        index += 1.0;
    }

    // let synth = Arc::new(Mutex::new(synth));
    // let stream = stream_setup(Arc::clone(&synth))?;
    // stream.play()?;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Refexer - Retro Sound FX Generator",
        options,
        Box::new(|_cc| Ok(Box::new(AppWrapper { inner: plot_data }))),
    )
    .map_err(|e| anyhow!("Failed to start eframe: {}", e))
}
