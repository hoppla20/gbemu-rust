use egui::util::History;

pub struct Stats {
    frame_times: History<f32>,
    cycle_times: History<f32>,
}

impl Default for Stats {
    fn default() -> Self {
        let max_age = 1.0_f32;
        let max_len = (max_age * 300.0).round() as usize;
        Self {
            frame_times: History::new(0..max_len, max_age),
            cycle_times: History::new(0..max_len, max_age),
        }
    }
}

impl Stats {
    pub fn on_frame_update(&mut self, now: f64, previous_frame_time: f32, cycles: u32) {
        if let Some(latest) = self.frame_times.latest_mut() {
            *latest = previous_frame_time;
        }
        self.frame_times.add(now, previous_frame_time);
        self.cycle_times.add(now, cycles as f32);
    }

    pub fn frames_per_second(&self) -> f32 {
        self.frame_times.rate().unwrap_or_default()
    }

    pub fn cycles_per_second(&self) -> f32 {
        let result = self.cycle_times.sum() / self.cycle_times.duration();
        if !result.is_nan() { result } else { 0.0 }
    }

    pub fn status_bar_ui(&self, ui: &mut egui::Ui) {
        ui.label(format!("FPS (Hz): {:.2}", self.frames_per_second()));
        ui.label(format!(
            "CPS (MHz): {:.2}",
            self.cycles_per_second() / 1000.0 / 1000.0
        ));
    }
}
