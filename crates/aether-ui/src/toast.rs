use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Toast {
    pub message: String,
    pub created_at: Instant,
    pub duration: Duration,
}

impl Toast {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            created_at: Instant::now(),
            duration: Duration::from_secs(3),
        }
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() >= self.duration
    }

    pub fn opacity(&self) -> f32 {
        let elapsed = self.created_at.elapsed().as_secs_f32();
        let total = self.duration.as_secs_f32();
        if elapsed > total {
            0.0
        } else if elapsed > total - 0.5 {
            (total - elapsed) / 0.5
        } else {
            1.0
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ToastManager {
    pub toasts: Vec<Toast>,
}

impl ToastManager {
    pub fn notify(&mut self, message: impl Into<String>) {
        self.toasts.push(Toast::new(message));
    }

    pub fn update(&mut self) {
        self.toasts.retain(|t| !t.is_expired());
    }
}
