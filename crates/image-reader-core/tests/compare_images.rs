use image_reader_core::compare_images;
use image::{ImageBuffer, Rgba};
use tempfile::tempdir;

#[test]
fn compares_same_size_images_and_reports_changed_bbox() {
    let dir = tempdir().expect("temp dir");
    let before_path = dir.path().join("before.png");
    let after_path = dir.path().join("after.png");
    let before: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_pixel(4, 4, Rgba([0, 0, 0, 255]));
    let mut after = before.clone();
    after.put_pixel(2, 1, Rgba([255, 255, 255, 255]));
    before.save(&before_path).expect("save before");
    after.save(&after_path).expect("save after");

    let diff = compare_images(&before_path, &after_path, 1024 * 1024, 0).expect("compare");
    assert!(!diff.identical);
    assert_eq!(diff.changed_pixels, 1);
    assert_eq!(diff.changed_bbox.unwrap(), image_reader_core::RegionBBox { x: 2, y: 1, width: 1, height: 1 });
}
