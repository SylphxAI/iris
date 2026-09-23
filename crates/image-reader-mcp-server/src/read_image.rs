use image_reader_core::{read_image_from_value, ProbeErrorCode, READ_IMAGE_ROUTE};
use rmcp::model::CallToolResult;
use serde_json::{json, Value};

use crate::SERVER_VERSION;

fn with_family_envelope(tool: &str, route_path: &str, mut body: Value) -> Value {
    let obj = body.as_object_mut().expect("structured body object");
    let warnings = obj
        .get("envelope")
        .and_then(|e| e.get("warnings"))
        .cloned()
        .unwrap_or_else(|| json!([]));
    let gaps = obj.get("gaps").cloned().unwrap_or_else(|| json!([]));
    let status = obj
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or("ok")
        .to_string();
    let path = obj
        .get("envelope")
        .and_then(|e| e.get("source").or_else(|| e.get("source_path")).or_else(|| e.get("locator").and_then(|l| l.get("path"))))
        .cloned();

    obj.insert("envelope_version".into(), json!("1"));
    obj.insert("status".into(), json!(status));
    obj.insert("tool".into(), json!(tool));
    obj.insert("product".into(), json!("iris"));
    obj.insert("product_version".into(), json!(SERVER_VERSION));
    obj.insert(
        "route".into(),
        json!({ "engine": "rust-core", "path": route_path }),
    );
    // Preserve string route for legacy tests under domain_route if needed.
    obj.insert("domain_route".into(), json!(route_path));
    obj.entry("warnings".to_string())
        .or_insert(warnings);
    obj.entry("gaps".to_string()).or_insert(gaps);
    obj.entry("confidence".to_string())
        .or_insert(json!({ "kind": "deterministic", "notes": [] }));
    if let Some(p) = path {
        obj.entry("source".to_string())
            .or_insert(json!({ "path": p }));
    }
    if let Some(twin) = obj.get("twin").cloned() {
        obj.entry("payload".to_string()).or_insert(twin);
    }
    body
}

pub fn read_image(args: Value) -> Result<CallToolResult, rmcp::ErrorData> {
    let success = read_image_from_value(&args).map_err(|error| match error.code {
        ProbeErrorCode::InvalidParams => rmcp::ErrorData::invalid_params(error.message, None),
        ProbeErrorCode::InvalidRequest => {
            rmcp::ErrorData::invalid_request(error.message, None)
        }
    })?;

    let mut structured = with_family_envelope(
        "read_image",
        READ_IMAGE_ROUTE,
        serde_json::json!({
            "tool": "read_image",
            "route": READ_IMAGE_ROUTE,
            "engine": image_reader_core::ENGINE_NAME,
            "twin": success.twin,
            "envelope": success.envelope,
            "status": "ok",
            "warnings": success.envelope.warnings.clone(),
            "gaps": [],
        }),
    );

    if crate::ocr::ocr_requested(&args) {
        if let Some(path) = args.get("path").and_then(Value::as_str) {
            let ocr = crate::ocr::run_opt_in_ocr(path, &args);
            if ocr.get("available").and_then(Value::as_bool) == Some(false) {
                if let Some(gaps) = structured.get_mut("gaps").and_then(Value::as_array_mut) {
                    gaps.push(serde_json::json!({
                        "code": "OCR_UNAVAILABLE",
                        "message": ocr.get("skipped_reason").and_then(Value::as_str).unwrap_or("OCR unavailable"),
                    }));
                }
            }
            if let Some(obj) = structured.as_object_mut() {
                obj.insert("ocr".into(), ocr);
            }
        }
    }

    Ok(CallToolResult::structured(structured))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn reads_fixture_through_rust_core_route() {
        let fixture =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../test/fixtures/sample.png");
        if !fixture.is_file() {
            return;
        }

        let result = read_image(serde_json::json!({
            "path": fixture,
            "include_metadata": false
        }))
        .expect("read_image");

        let structured = result.structured_content.expect("structured");
        assert_eq!(
            structured.get("domain_route").and_then(Value::as_str),
            Some(READ_IMAGE_ROUTE)
        );
        assert_eq!(
            structured.get("envelope_version").and_then(Value::as_str),
            Some("1")
        );
        assert_eq!(
            structured.get("product").and_then(Value::as_str),
            Some("iris")
        );
        assert_eq!(
            structured
                .get("route")
                .and_then(|r| r.get("engine"))
                .and_then(Value::as_str),
            Some("rust-core")
        );
        assert_eq!(
            structured
                .get("twin")
                .and_then(|value| value.get("mime"))
                .and_then(Value::as_str),
            Some("image/png")
        );
        assert!(structured.get("ocr").is_none());

        let with_ocr = read_image(serde_json::json!({
            "path": fixture,
            "include_ocr": true,
            "include_metadata": false
        }))
        .expect("read_image ocr");
        let ocr_body = with_ocr.structured_content.expect("ocr structured");
        let ocr = ocr_body.get("ocr").expect("ocr field");
        assert_eq!(ocr.get("route").and_then(Value::as_str), Some("tesseract_tsv"));
        if ocr.get("available").and_then(Value::as_bool) == Some(false) {
            assert!(ocr.get("skipped_reason").and_then(Value::as_str).is_some());
        }

        let probe = image_probe(serde_json::json!({ "path": fixture })).expect("probe");
        let probe_body = probe.structured_content.expect("probe body");
        assert_eq!(
            probe_body.get("probe").and_then(|value| value.get("mime")).and_then(Value::as_str),
            Some("image/png")
        );

        let crop = crop_region(serde_json::json!({
            "path": fixture,
            "region": { "x": 0, "y": 0, "width": 1, "height": 1 }
        }))
        .expect("crop");
        let crop_body = crop.structured_content.expect("crop body");
        assert_eq!(
            crop_body.get("region_evidence").and_then(|value| value.get("width")).and_then(Value::as_u64),
            Some(1)
        );
        assert!(crop_body
            .get("region_evidence")
            .and_then(|value| value.get("imageBase64"))
            .is_none());
    }
}


pub fn image_probe(args: Value) -> Result<CallToolResult, rmcp::ErrorData> {
    let path = args.get("path").and_then(Value::as_str).ok_or_else(|| {
        rmcp::ErrorData::invalid_params("path is required", None)
    })?;
    let max_file_bytes = args
        .get("max_file_bytes")
        .and_then(Value::as_u64)
        .unwrap_or(32 * 1024 * 1024);
    let probe = image_reader_core::probe_image(std::path::Path::new(path), max_file_bytes)
        .map_err(probe_error)?;
    Ok(CallToolResult::structured(serde_json::json!({
        "status": "ok",
        "tool": "image_probe",
        "product": "iris",
        "product_version": crate::SERVER_VERSION,
        "envelope_version": "1",
        "route": { "engine": "rust-core", "path": image_reader_core::DECODE_ROUTE },
        "probe": probe,
        "warnings": [],
        "gaps": [],
    })))
}

pub fn crop_region(args: Value) -> Result<CallToolResult, rmcp::ErrorData> {
    let path = args.get("path").and_then(Value::as_str).ok_or_else(|| {
        rmcp::ErrorData::invalid_params("path is required", None)
    })?;
    let region = args.get("region").ok_or_else(|| {
        rmcp::ErrorData::invalid_params("region is required", None)
    })?;
    let bbox = image_reader_core::RegionBBox {
        x: required_u32(region, "x")?,
        y: required_u32(region, "y")?,
        width: required_u32(region, "width")?,
        height: required_u32(region, "height")?,
    };
    let max_file_bytes = args
        .get("max_file_bytes")
        .and_then(Value::as_u64)
        .unwrap_or(32 * 1024 * 1024);
    let max_pixels = args
        .get("max_pixels")
        .and_then(Value::as_u64)
        .unwrap_or(64 * 1024 * 1024);
    let max_dimension = args
        .get("max_region_dimension")
        .and_then(Value::as_u64)
        .map(|value| value as u32);
    let include_image_base64 = args
        .get("include_region_image")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let evidence = image_reader_core::crop_region(
        std::path::Path::new(path),
        max_file_bytes,
        max_pixels,
        bbox,
        max_dimension,
        include_image_base64,
    )
    .map_err(probe_error)?;
    Ok(CallToolResult::structured(serde_json::json!({
        "status": "ok",
        "tool": "crop_region",
        "product": "iris",
        "product_version": crate::SERVER_VERSION,
        "envelope_version": "1",
        "route": { "engine": "rust-core", "path": image_reader_core::CROP_ROUTE },
        "region_evidence": evidence,
        "warnings": [],
        "gaps": [],
    })))
}

fn required_u32(value: &Value, key: &str) -> Result<u32, rmcp::ErrorData> {
    value
        .get(key)
        .and_then(Value::as_u64)
        .map(|number| number as u32)
        .ok_or_else(|| {
            rmcp::ErrorData::invalid_params(format!("region.{key} is required"), None)
        })
}

fn probe_error(error: image_reader_core::ProbeError) -> rmcp::ErrorData {
    match error.code {
        image_reader_core::ProbeErrorCode::InvalidParams => {
            rmcp::ErrorData::invalid_params(error.message, None)
        }
        image_reader_core::ProbeErrorCode::InvalidRequest => {
            rmcp::ErrorData::invalid_request(error.message, None)
        }
    }
}

pub fn compare_images(args: Value) -> Result<CallToolResult, rmcp::ErrorData> {
    let before = args.get("before").and_then(Value::as_str).ok_or_else(|| rmcp::ErrorData::invalid_params("before is required", None))?;
    let after = args.get("after").and_then(Value::as_str).ok_or_else(|| rmcp::ErrorData::invalid_params("after is required", None))?;
    let max_file_bytes = args.get("max_file_bytes").and_then(Value::as_u64).unwrap_or(32 * 1024 * 1024);
    let threshold = args.get("threshold").and_then(Value::as_u64).unwrap_or(0) as u8;
    let diff = image_reader_core::compare_images(std::path::Path::new(before), std::path::Path::new(after), max_file_bytes, threshold)
        .map_err(|error| rmcp::ErrorData::invalid_request(error.message, None))?;
    let structured = serde_json::json!({
        "status": "ok",
        "tool": "compare_images",
        "product": "iris",
        "product_version": crate::SERVER_VERSION,
        "envelope_version": "1",
        "route": { "engine": "rust-core", "path": "rust-image-diff" },
        "diff": diff,
        "warnings": [],
        "gaps": [],
    });
    Ok(CallToolResult::structured(structured))
}
