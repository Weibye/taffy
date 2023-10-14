#[test]
fn grid_align_items_baseline_child_margin_percent() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout, Taffy};
    let mut taffy: Taffy<crate::TextMeasure> = Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Grid,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(50f32),
                height: taffy::style::Dimension::Length(50f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Percent(0.05f32),
                right: taffy::style::LengthPercentageAuto::Percent(0.05f32),
                top: taffy::style::LengthPercentageAuto::Percent(0.05f32),
                bottom: taffy::style::LengthPercentageAuto::Percent(0.05f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node10 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Grid,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(50f32),
                height: taffy::style::Dimension::Length(10f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Percent(0.01f32),
                right: taffy::style::LengthPercentageAuto::Percent(0.01f32),
                top: taffy::style::LengthPercentageAuto::Percent(0.01f32),
                bottom: taffy::style::LengthPercentageAuto::Percent(0.01f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(50f32),
                    height: taffy::style::Dimension::Length(20f32),
                },
                ..Default::default()
            },
            &[node10],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                align_items: Some(taffy::style::AlignItems::Baseline),
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(100f32),
                    height: taffy::style::Dimension::Length(100f32),
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node, 100f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node, 100f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node0, 50f32, size.width);
    assert_eq!(size.height, 50f32, "height of node {:?}. Expected {}. Actual {}", node0, 50f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node0, 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node0, 5f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node1, 50f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node1, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 70f32, "y of node {:?}. Expected {}. Actual {}", node1, 70f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node10).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node10, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node10, 10f32, size.height);
    assert_eq!(location.x, 1f32, "x of node {:?}. Expected {}. Actual {}", node10, 1f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node10, 1f32, location.y);
}