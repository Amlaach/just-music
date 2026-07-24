use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisibleWindow {
    pub start_index: usize,
    pub end_index: usize,
    pub top_padding_px: f32,
    pub bottom_padding_px: f32,
}

#[derive(Debug, Clone)]
pub struct VirtualListCalculator {
    item_height: f32,
    overdraw_count: usize,
}

impl VirtualListCalculator {
    pub fn new(item_height: f32, overdraw_count: usize) -> Self {
        Self {
            item_height,
            overdraw_count,
        }
    }

    pub fn calculate(
        &self,
        total_items: usize,
        scroll_offset_y: f32,
        viewport_height: f32,
    ) -> VisibleWindow {
        if total_items == 0 || viewport_height <= 0.0 {
            return VisibleWindow {
                start_index: 0,
                end_index: 0,
                top_padding_px: 0.0,
                bottom_padding_px: 0.0,
            };
        }

        let first_visible = (scroll_offset_y / self.item_height).floor() as usize;
        let visible_count = (viewport_height / self.item_height).ceil() as usize;

        let start_index = first_visible.saturating_sub(self.overdraw_count);
        let end_index = (first_visible + visible_count + self.overdraw_count).min(total_items);

        let top_padding_px = start_index as f32 * self.item_height;
        let bottom_padding_px = (total_items.saturating_sub(end_index)) as f32 * self.item_height;

        VisibleWindow {
            start_index,
            end_index,
            top_padding_px,
            bottom_padding_px,
        }
    }
}

impl Default for VirtualListCalculator {
    fn default() -> Self {
        Self::new(48.0, 5)
    }
}
