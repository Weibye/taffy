#[test]
fn grid_basic_implicit_tracks() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout, Taffy};
    let mut taffy: Taffy<crate::TextMeasure> = Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(35f32),
                height: taffy::style::Dimension::Length(35f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_rows: vec![length(40f32)],
                grid_template_columns: vec![length(40f32)],
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
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node, 40f32, size.width);
    assert_eq!(size.height, 75f32, "height of node {:?}. Expected {}. Actual {}", node, 75f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node0, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node0, 40f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 35f32, "width of node {:?}. Expected {}. Actual {}", node1, 35f32, size.width);
    assert_eq!(size.height, 35f32, "height of node {:?}. Expected {}. Actual {}", node1, 35f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node1, 40f32, location.y);
}
