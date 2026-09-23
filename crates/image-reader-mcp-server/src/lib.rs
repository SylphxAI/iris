pub mod http_transport;
pub mod ocr;
pub mod read_image;
pub mod tool_routes;

use rmcp::{
    handler::server::router::tool::ToolRouter,
    handler::server::wrapper::Parameters,
    model::{Implementation, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router, ErrorData, ServerHandler,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Free-form MCP tool args object (root type=object required by rmcp ≥1.8 schema gate).
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
struct FreeformToolArgs(Map<String, Value>);

impl FreeformToolArgs {
    fn into_value(self) -> Value {
        Value::Object(self.0)
    }
}

pub const SERVER_NAME: &str = "iris";
/// The product version, injected at build time from package.json (see the
/// `build:rust` script). Falls back to the release this source last shipped so a
/// plain `cargo build` still compiles.
pub const SERVER_VERSION: &str = match option_env!("IRIS_PRODUCT_VERSION") {
    Some(version) => version,
    None => "0.3.3",
};
pub const SERVER_INSTRUCTIONS: &str =
    "Image facts with pixel-level proof. read_image returns dimensions, format, and metadata, and does not run OCR unless include_ocr is true or profile is quality. image_probe is geometry only. crop_region extracts one region. compare_images diffs two same-size images. OCR uses local tesseract and reports a gap when that binary is missing. No generative vision model.";

#[derive(Clone)]
pub struct ImageReaderMcp {
    pub tool_router: ToolRouter<Self>,
}

impl ImageReaderMcp {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_router]
impl ImageReaderMcp {
    #[tool(
        description = "Read one local image. Returns dimensions, format, metadata, and trust warnings. Does not run OCR unless include_ocr is true or profile is quality. A region argument crops as part of this read. No generative vision model."
    )]
    fn read_image(
        &self,
        Parameters(args): Parameters<FreeformToolArgs>,
    ) -> Result<rmcp::model::CallToolResult, ErrorData> {
        read_image::read_image(args.into_value())
    }

    #[tool(
        description = "Compare two same-size images and report changed pixels and a changed bounding box. This does not run OCR."
    )]
    fn compare_images(
        &self,
        Parameters(args): Parameters<FreeformToolArgs>,
    ) -> Result<rmcp::model::CallToolResult, ErrorData> {
        read_image::compare_images(args.into_value())
    }

    #[tool(
        description = "Cheap image probe. Returns format, dimensions, pixel count, and source hash. Does not run OCR, crop, or a vision model."
    )]
    fn image_probe(
        &self,
        Parameters(args): Parameters<FreeformToolArgs>,
    ) -> Result<rmcp::model::CallToolResult, ErrorData> {
        read_image::image_probe(args.into_value())
    }

    #[tool(
        description = "Extract one citeable region from a local image. Returns the crop hash and pixel bounds. Set include_region_image to true for PNG bytes. Does not run OCR."
    )]
    fn crop_region(
        &self,
        Parameters(args): Parameters<FreeformToolArgs>,
    ) -> Result<rmcp::model::CallToolResult, ErrorData> {
        read_image::crop_region(args.into_value())
    }
}

#[tool_handler]
impl ServerHandler for ImageReaderMcp {
    fn get_info(&self) -> ServerInfo {
        // rmcp >=1.8: ServerInfo/Implementation are #[non_exhaustive] — use builders only.
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(
                Implementation::new(SERVER_NAME, SERVER_VERSION)
                    .with_description(
                        "Rust-native MCP server for Iris (@sylphx/iris) (modelcontextprotocol/rust-sdk rmcp)",
                    )
                    .with_website_url("https://sylphxai.github.io/iris/"),
            )
            .with_instructions(SERVER_INSTRUCTIONS)
    }
}

#[cfg(test)]
mod tests {
    use super::ImageReaderMcp;
    #[test]
    fn exposes_read_image_tool_surface() {
        let tools = ImageReaderMcp::new().tool_router.list_all();
        let names: Vec<_> = tools.iter().map(|tool| tool.name.to_string()).collect();
        assert!(names.contains(&"read_image".to_string()));
        assert!(names.contains(&"image_probe".to_string()));
        assert!(names.contains(&"crop_region".to_string()));
        assert!(names.contains(&"compare_images".to_string()));
        let read_image = tools.iter().find(|tool| tool.name == "read_image").expect("read_image");
        let description = read_image.description.as_deref().unwrap_or("");
        assert!(description.contains("Does not run OCR unless"));
    }

    #[test]
    fn server_info_is_brand_sole_iris() {
        use rmcp::ServerHandler;
        use super::{SERVER_NAME, SERVER_VERSION};
        let info = ImageReaderMcp::new().get_info();
        let name = info.server_info.name.to_string();
        let version = info.server_info.version.to_string();
        assert_eq!(name, SERVER_NAME);
        assert_eq!(version, SERVER_VERSION);
        assert_eq!(SERVER_NAME, "iris");
        assert!(!name.contains("image-reader"));
    }
}
