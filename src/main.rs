use eframe::{egui, epi, NativeOptions};

struct App {
    coefs: [f64; 3],
    resolution: usize,
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Quadratic Function Graph"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(
                    egui::DragValue::new(&mut self.coefs[0])
                        .speed(0.01)
                        .suffix("x\u{00b2}"),
                );
                ui.label("+");
                ui.add(
                    egui::DragValue::new(&mut self.coefs[1])
                        .speed(0.01)
                        .suffix("x"),
                );
                ui.label("+");
                ui.add(egui::DragValue::new(&mut self.coefs[2]).speed(0.01));
            });

            ui.add(
                egui::Slider::new(&mut self.resolution, 10..=1000)
                    .prefix("Resolution: ")
                    .logarithmic(true),
            );

            egui::plot::Plot::new("plot_0").show(ui, |plot| {
                let coefs = self.coefs;
                let values = egui::plot::Values::from_explicit_callback(
                    move |x| eval_quadratic(coefs, x),
                    ..,
                    self.resolution,
                );
                let line = egui::plot::Line::new(values);
                plot.line(line);
            });
        });
    }
}

fn eval_quadratic([a, b, c]: [f64; 3], x: f64) -> f64 {
    a * x * x + b * x + c
}

fn main() {
    let app = App {
        coefs: [0.0, 1.0, 0.0],
        resolution: 1000,
    };
    eframe::run_native(Box::new(app), NativeOptions::default());
}
