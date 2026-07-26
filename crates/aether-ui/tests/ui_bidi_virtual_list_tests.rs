use aether_ui::{BiDiEngine, LayoutDirection, TextAlignment, VirtualListCalculator};

#[test]
fn test_bidi_hebrew_detection_and_alignment() {
    assert_eq!(
        BiDiEngine::detect_text_direction("שיר חדש"),
        LayoutDirection::Rtl
    );
    assert_eq!(
        BiDiEngine::detect_text_direction("New Song 2024"),
        LayoutDirection::Ltr
    );

    let engine = BiDiEngine::new(LayoutDirection::Rtl);
    assert_eq!(
        engine.resolve_alignment(TextAlignment::Start),
        TextAlignment::End
    );
}

#[test]
fn test_virtual_list_calculator() {
    let calc = VirtualListCalculator::new(50.0, 2);
    let win = calc.calculate(100000, 1000.0, 500.0);
    assert_eq!(win.start_index, 18); // 20 - 2 overdraw
    assert_eq!(win.end_index, 32); // 20 + 10 visible + 2 overdraw
    assert_eq!(win.top_padding_px, 900.0);
}
