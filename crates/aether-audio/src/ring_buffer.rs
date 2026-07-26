use rtrb::{Consumer, Producer, RingBuffer};

pub fn create_audio_ring_buffer(capacity: usize) -> (Producer<f32>, Consumer<f32>) {
    RingBuffer::new(capacity)
}
