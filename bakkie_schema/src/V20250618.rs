#![allow(irrefutable_let_patterns)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "Optional annotations for the client. The client can use annotations to inform how objects are used or displayed"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Optional annotations for the client. The client can use annotations to inform how objects are used or displayed\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"audience\": {"]
#[doc = "      \"description\": \"Describes who the intended customer of this object or data is.\\n\\nIt can include multiple entries to indicate content useful for multiple audiences (e.g., `[\\\"user\\\", \\\"assistant\\\"]`).\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Role\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"lastModified\": {"]
#[doc = "      \"description\": \"The moment the resource was last modified, as an ISO 8601 formatted string.\\n\\nShould be an ISO 8601 formatted string (e.g., \\\"2025-01-12T15:00:58Z\\\").\\n\\nExamples: last activity timestamp in an open file, timestamp when the resource\\nwas attached, etc.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"priority\": {"]
#[doc = "      \"description\": \"Describes how important this data is for operating the server.\\n\\nA value of 1 means \\\"most important,\\\" and indicates that the data is\\neffectively required, while 0 means \\\"least important,\\\" and indicates that\\nthe data is entirely optional.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct Annotations {
    #[doc = "Describes who the intended customer of this object or data is.\n\nIt can include multiple entries to indicate content useful for multiple audiences (e.g., `[\"user\", \"assistant\"]`)."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub audience: ::std::vec::Vec<Role>,
    #[doc = "The moment the resource was last modified, as an ISO 8601 formatted string.\n\nShould be an ISO 8601 formatted string (e.g., \"2025-01-12T15:00:58Z\").\n\nExamples: last activity timestamp in an open file, timestamp when the resource\nwas attached, etc."]
    #[serde(
        rename = "lastModified",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_modified: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<f64>,
}
impl ::std::convert::From<&Annotations> for Annotations {
    fn from(value: &Annotations) -> Self {
        value.clone()
    }
}
impl Annotations {
    pub fn builder() -> builder::Annotations {
        Default::default()
    }
}
#[doc = "Audio provided to or from an LLM."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Audio provided to or from an LLM.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\","]
#[doc = "    \"mimeType\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"annotations\": {"]
#[doc = "      \"description\": \"Optional annotations for the client.\","]
#[doc = "      \"$ref\": \"#/definitions/Annotations\""]
#[doc = "    },"]
#[doc = "    \"data\": {"]
#[doc = "      \"description\": \"The base64-encoded audio data.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"byte\""]
#[doc = "    },"]
#[doc = "    \"mimeType\": {"]
#[doc = "      \"description\": \"The MIME type of the audio. Different providers may support different audio types.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"audio\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AudioContent {
    #[doc = "Optional annotations for the client."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations: ::std::option::Option<Annotations>,
    #[doc = "The base64-encoded audio data."]
    pub data: ::std::string::String,
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The MIME type of the audio. Different providers may support different audio types."]
    #[serde(rename = "mimeType")]
    pub mime_type: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&AudioContent> for AudioContent {
    fn from(value: &AudioContent) -> Self {
        value.clone()
    }
}
impl AudioContent {
    pub fn builder() -> builder::AudioContent {
        Default::default()
    }
}
#[doc = "Base interface for metadata with name (identifier) and title (display name) properties."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Base interface for metadata with name (identifier) and title (display name) properties.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"description\": \"Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\\neven by those unfamiliar with domain-specific terminology.\\n\\nIf not provided, the name should be used for display (except for Tool,\\nwhere `annotations.title` should be given precedence over using `name`,\\nif present).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BaseMetadata {
    #[doc = "Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present)."]
    pub name: ::std::string::String,
    #[doc = "Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\neven by those unfamiliar with domain-specific terminology.\n\nIf not provided, the name should be used for display (except for Tool,\nwhere `annotations.title` should be given precedence over using `name`,\nif present)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&BaseMetadata> for BaseMetadata {
    fn from(value: &BaseMetadata) -> Self {
        value.clone()
    }
}
impl BaseMetadata {
    pub fn builder() -> builder::BaseMetadata {
        Default::default()
    }
}
#[doc = "`BlobResourceContents`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"blob\","]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"blob\": {"]
#[doc = "      \"description\": \"A base64-encoded string representing the binary data of the item.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"byte\""]
#[doc = "    },"]
#[doc = "    \"mimeType\": {"]
#[doc = "      \"description\": \"The MIME type of this resource, if known.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"The URI of this resource.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BlobResourceContents {
    #[doc = "A base64-encoded string representing the binary data of the item."]
    pub blob: ::std::string::String,
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The MIME type of this resource, if known."]
    #[serde(
        rename = "mimeType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[doc = "The URI of this resource."]
    pub uri: ::std::string::String,
}
impl ::std::convert::From<&BlobResourceContents> for BlobResourceContents {
    fn from(value: &BlobResourceContents) -> Self {
        value.clone()
    }
}
impl BlobResourceContents {
    pub fn builder() -> builder::BlobResourceContents {
        Default::default()
    }
}
#[doc = "`BooleanSchema`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"default\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BooleanSchema {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&BooleanSchema> for BooleanSchema {
    fn from(value: &BooleanSchema) -> Self {
        value.clone()
    }
}
impl BooleanSchema {
    pub fn builder() -> builder::BooleanSchema {
        Default::default()
    }
}
#[doc = "Used by the client to invoke a tool provided by the server."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Used by the client to invoke a tool provided by the server.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"tools/call\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"name\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"arguments\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {}"]
#[doc = "        },"]
#[doc = "        \"name\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CallToolRequest {
    pub method: ::std::string::String,
    pub params: CallToolRequestParams,
}
impl ::std::convert::From<&CallToolRequest> for CallToolRequest {
    fn from(value: &CallToolRequest) -> Self {
        value.clone()
    }
}
impl CallToolRequest {
    pub fn builder() -> builder::CallToolRequest {
        Default::default()
    }
}
#[doc = "`CallToolRequestParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"arguments\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CallToolRequestParams {
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub arguments: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub name: ::std::string::String,
}
impl ::std::convert::From<&CallToolRequestParams> for CallToolRequestParams {
    fn from(value: &CallToolRequestParams) -> Self {
        value.clone()
    }
}
impl CallToolRequestParams {
    pub fn builder() -> builder::CallToolRequestParams {
        Default::default()
    }
}
#[doc = "The server's response to a tool call."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The server's response to a tool call.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"content\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"content\": {"]
#[doc = "      \"description\": \"A list of content objects that represent the unstructured result of the tool call.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ContentBlock\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"isError\": {"]
#[doc = "      \"description\": \"Whether the tool call ended in an error.\\n\\nIf not set, this is assumed to be false (the call was successful).\\n\\nAny errors that originate from the tool SHOULD be reported inside the result\\nobject, with `isError` set to true, _not_ as an MCP protocol-level error\\nresponse. Otherwise, the LLM would not be able to see that an error occurred\\nand self-correct.\\n\\nHowever, any errors in _finding_ the tool, an error indicating that the\\nserver does not support tool calls, or any other exceptional conditions,\\nshould be reported as an MCP error response.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"structuredContent\": {"]
#[doc = "      \"description\": \"An optional JSON object that represents the structured result of the tool call.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CallToolResult {
    #[doc = "A list of content objects that represent the unstructured result of the tool call."]
    pub content: ::std::vec::Vec<ContentBlock>,
    #[doc = "Whether the tool call ended in an error.\n\nIf not set, this is assumed to be false (the call was successful).\n\nAny errors that originate from the tool SHOULD be reported inside the result\nobject, with `isError` set to true, _not_ as an MCP protocol-level error\nresponse. Otherwise, the LLM would not be able to see that an error occurred\nand self-correct.\n\nHowever, any errors in _finding_ the tool, an error indicating that the\nserver does not support tool calls, or any other exceptional conditions,\nshould be reported as an MCP error response."]
    #[serde(
        rename = "isError",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_error: ::std::option::Option<bool>,
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "An optional JSON object that represents the structured result of the tool call."]
    #[serde(
        rename = "structuredContent",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub structured_content: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&CallToolResult> for CallToolResult {
    fn from(value: &CallToolResult) -> Self {
        value.clone()
    }
}
impl CallToolResult {
    pub fn builder() -> builder::CallToolResult {
        Default::default()
    }
}
#[doc = "This notification can be sent by either side to indicate that it is cancelling a previously-issued request.\n\nThe request SHOULD still be in-flight, but due to communication latency, it is always possible that this notification MAY arrive after the request has already finished.\n\nThis notification indicates that the result will be unused, so any associated processing SHOULD cease.\n\nA client MUST NOT attempt to cancel its `initialize` request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"This notification can be sent by either side to indicate that it is cancelling a previously-issued request.\\n\\nThe request SHOULD still be in-flight, but due to communication latency, it is always possible that this notification MAY arrive after the request has already finished.\\n\\nThis notification indicates that the result will be unused, so any associated processing SHOULD cease.\\n\\nA client MUST NOT attempt to cancel its `initialize` request.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"notifications/cancelled\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"requestId\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"reason\": {"]
#[doc = "          \"description\": \"An optional string describing the reason for the cancellation. This MAY be logged or presented to the user.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"requestId\": {"]
#[doc = "          \"description\": \"The ID of the request to cancel.\\n\\nThis MUST correspond to the ID of a request previously issued in the same direction.\","]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CancelledNotification {
    pub method: ::std::string::String,
    pub params: CancelledNotificationParams,
}
impl ::std::convert::From<&CancelledNotification> for CancelledNotification {
    fn from(value: &CancelledNotification) -> Self {
        value.clone()
    }
}
impl CancelledNotification {
    pub fn builder() -> builder::CancelledNotification {
        Default::default()
    }
}
#[doc = "`CancelledNotificationParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"requestId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"reason\": {"]
#[doc = "      \"description\": \"An optional string describing the reason for the cancellation. This MAY be logged or presented to the user.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"requestId\": {"]
#[doc = "      \"description\": \"The ID of the request to cancel.\\n\\nThis MUST correspond to the ID of a request previously issued in the same direction.\","]
#[doc = "      \"$ref\": \"#/definitions/RequestId\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CancelledNotificationParams {
    #[doc = "An optional string describing the reason for the cancellation. This MAY be logged or presented to the user."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: ::std::option::Option<::std::string::String>,
    #[doc = "The ID of the request to cancel.\n\nThis MUST correspond to the ID of a request previously issued in the same direction."]
    #[serde(rename = "requestId")]
    pub request_id: RequestId,
}
impl ::std::convert::From<&CancelledNotificationParams> for CancelledNotificationParams {
    fn from(value: &CancelledNotificationParams) -> Self {
        value.clone()
    }
}
impl CancelledNotificationParams {
    pub fn builder() -> builder::CancelledNotificationParams {
        Default::default()
    }
}
#[doc = "Capabilities a client may support. Known capabilities are defined here, in this schema, but this is not a closed set: any client can define its own, additional capabilities."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Capabilities a client may support. Known capabilities are defined here, in this schema, but this is not a closed set: any client can define its own, additional capabilities.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"elicitation\": {"]
#[doc = "      \"description\": \"Present if the client supports elicitation from the server.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"experimental\": {"]
#[doc = "      \"description\": \"Experimental, non-standard capabilities that the client supports.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"additionalProperties\": true"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"roots\": {"]
#[doc = "      \"description\": \"Present if the client supports listing roots.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"listChanged\": {"]
#[doc = "          \"description\": \"Whether the client supports notifications for changes to the roots list.\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"sampling\": {"]
#[doc = "      \"description\": \"Present if the client supports sampling from an LLM.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ClientCapabilities {
    #[doc = "Present if the client supports elicitation from the server."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub elicitation: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Experimental, non-standard capabilities that the client supports."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub experimental: ::std::collections::HashMap<
        ::std::string::String,
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub roots: ::std::option::Option<ClientCapabilitiesRoots>,
    #[doc = "Present if the client supports sampling from an LLM."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub sampling: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&ClientCapabilities> for ClientCapabilities {
    fn from(value: &ClientCapabilities) -> Self {
        value.clone()
    }
}
impl ClientCapabilities {
    pub fn builder() -> builder::ClientCapabilities {
        Default::default()
    }
}
#[doc = "Present if the client supports listing roots."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Present if the client supports listing roots.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"listChanged\": {"]
#[doc = "      \"description\": \"Whether the client supports notifications for changes to the roots list.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ClientCapabilitiesRoots {
    #[doc = "Whether the client supports notifications for changes to the roots list."]
    #[serde(
        rename = "listChanged",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub list_changed: ::std::option::Option<bool>,
}
impl ::std::convert::From<&ClientCapabilitiesRoots> for ClientCapabilitiesRoots {
    fn from(value: &ClientCapabilitiesRoots) -> Self {
        value.clone()
    }
}
impl ClientCapabilitiesRoots {
    pub fn builder() -> builder::ClientCapabilitiesRoots {
        Default::default()
    }
}
#[doc = "`ClientNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/CancelledNotification\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/InitializedNotification\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ProgressNotification\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/RootsListChangedNotification\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ClientNotification {
    CancelledNotification(CancelledNotification),
    InitializedNotification(InitializedNotification),
    ProgressNotification(ProgressNotification),
    RootsListChangedNotification(RootsListChangedNotification),
}
impl ::std::convert::From<&Self> for ClientNotification {
    fn from(value: &ClientNotification) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<CancelledNotification> for ClientNotification {
    fn from(value: CancelledNotification) -> Self {
        Self::CancelledNotification(value)
    }
}
impl ::std::convert::From<InitializedNotification> for ClientNotification {
    fn from(value: InitializedNotification) -> Self {
        Self::InitializedNotification(value)
    }
}
impl ::std::convert::From<ProgressNotification> for ClientNotification {
    fn from(value: ProgressNotification) -> Self {
        Self::ProgressNotification(value)
    }
}
impl ::std::convert::From<RootsListChangedNotification> for ClientNotification {
    fn from(value: RootsListChangedNotification) -> Self {
        Self::RootsListChangedNotification(value)
    }
}
#[doc = "`ClientRequest`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/InitializeRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/PingRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ListResourcesRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ListResourceTemplatesRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ReadResourceRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/SubscribeRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/UnsubscribeRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ListPromptsRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/GetPromptRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ListToolsRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/CallToolRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/SetLevelRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/CompleteRequest\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ClientRequest {
    InitializeRequest(InitializeRequest),
    PingRequest(PingRequest),
    ListResourcesRequest(ListResourcesRequest),
    ListResourceTemplatesRequest(ListResourceTemplatesRequest),
    ReadResourceRequest(ReadResourceRequest),
    SubscribeRequest(SubscribeRequest),
    UnsubscribeRequest(UnsubscribeRequest),
    ListPromptsRequest(ListPromptsRequest),
    GetPromptRequest(GetPromptRequest),
    ListToolsRequest(ListToolsRequest),
    CallToolRequest(CallToolRequest),
    SetLevelRequest(SetLevelRequest),
    CompleteRequest(CompleteRequest),
}
impl ::std::convert::From<&Self> for ClientRequest {
    fn from(value: &ClientRequest) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<InitializeRequest> for ClientRequest {
    fn from(value: InitializeRequest) -> Self {
        Self::InitializeRequest(value)
    }
}
impl ::std::convert::From<PingRequest> for ClientRequest {
    fn from(value: PingRequest) -> Self {
        Self::PingRequest(value)
    }
}
impl ::std::convert::From<ListResourcesRequest> for ClientRequest {
    fn from(value: ListResourcesRequest) -> Self {
        Self::ListResourcesRequest(value)
    }
}
impl ::std::convert::From<ListResourceTemplatesRequest> for ClientRequest {
    fn from(value: ListResourceTemplatesRequest) -> Self {
        Self::ListResourceTemplatesRequest(value)
    }
}
impl ::std::convert::From<ReadResourceRequest> for ClientRequest {
    fn from(value: ReadResourceRequest) -> Self {
        Self::ReadResourceRequest(value)
    }
}
impl ::std::convert::From<SubscribeRequest> for ClientRequest {
    fn from(value: SubscribeRequest) -> Self {
        Self::SubscribeRequest(value)
    }
}
impl ::std::convert::From<UnsubscribeRequest> for ClientRequest {
    fn from(value: UnsubscribeRequest) -> Self {
        Self::UnsubscribeRequest(value)
    }
}
impl ::std::convert::From<ListPromptsRequest> for ClientRequest {
    fn from(value: ListPromptsRequest) -> Self {
        Self::ListPromptsRequest(value)
    }
}
impl ::std::convert::From<GetPromptRequest> for ClientRequest {
    fn from(value: GetPromptRequest) -> Self {
        Self::GetPromptRequest(value)
    }
}
impl ::std::convert::From<ListToolsRequest> for ClientRequest {
    fn from(value: ListToolsRequest) -> Self {
        Self::ListToolsRequest(value)
    }
}
impl ::std::convert::From<CallToolRequest> for ClientRequest {
    fn from(value: CallToolRequest) -> Self {
        Self::CallToolRequest(value)
    }
}
impl ::std::convert::From<SetLevelRequest> for ClientRequest {
    fn from(value: SetLevelRequest) -> Self {
        Self::SetLevelRequest(value)
    }
}
impl ::std::convert::From<CompleteRequest> for ClientRequest {
    fn from(value: CompleteRequest) -> Self {
        Self::CompleteRequest(value)
    }
}
#[doc = "`ClientResult`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Result\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/CreateMessageResult\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ListRootsResult\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ElicitResult\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ClientResult {
    Result(Result),
    CreateMessageResult(CreateMessageResult),
    ListRootsResult(ListRootsResult),
    ElicitResult(ElicitResult),
}
impl ::std::convert::From<&Self> for ClientResult {
    fn from(value: &ClientResult) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<Result> for ClientResult {
    fn from(value: Result) -> Self {
        Self::Result(value)
    }
}
impl ::std::convert::From<CreateMessageResult> for ClientResult {
    fn from(value: CreateMessageResult) -> Self {
        Self::CreateMessageResult(value)
    }
}
impl ::std::convert::From<ListRootsResult> for ClientResult {
    fn from(value: ListRootsResult) -> Self {
        Self::ListRootsResult(value)
    }
}
impl ::std::convert::From<ElicitResult> for ClientResult {
    fn from(value: ElicitResult) -> Self {
        Self::ElicitResult(value)
    }
}
#[doc = "A request from the client to the server, to ask for completion options."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A request from the client to the server, to ask for completion options.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"completion/complete\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"argument\","]
#[doc = "        \"ref\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"argument\": {"]
#[doc = "          \"description\": \"The argument's information\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"name\","]
#[doc = "            \"value\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"name\": {"]
#[doc = "              \"description\": \"The name of the argument\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"value\": {"]
#[doc = "              \"description\": \"The value of the argument to use for completion matching.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"context\": {"]
#[doc = "          \"description\": \"Additional, optional context for completions\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"arguments\": {"]
#[doc = "              \"description\": \"Previously-resolved variables in a URI template or prompt.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"additionalProperties\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"ref\": {"]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/PromptReference\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/ResourceTemplateReference\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CompleteRequest {
    pub method: ::std::string::String,
    pub params: CompleteRequestParams,
}
impl ::std::convert::From<&CompleteRequest> for CompleteRequest {
    fn from(value: &CompleteRequest) -> Self {
        value.clone()
    }
}
impl CompleteRequest {
    pub fn builder() -> builder::CompleteRequest {
        Default::default()
    }
}
#[doc = "`CompleteRequestParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"argument\","]
#[doc = "    \"ref\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"argument\": {"]
#[doc = "      \"description\": \"The argument's information\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"name\","]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"name\": {"]
#[doc = "          \"description\": \"The name of the argument\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"value\": {"]
#[doc = "          \"description\": \"The value of the argument to use for completion matching.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"context\": {"]
#[doc = "      \"description\": \"Additional, optional context for completions\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"arguments\": {"]
#[doc = "          \"description\": \"Previously-resolved variables in a URI template or prompt.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"ref\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/PromptReference\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ResourceTemplateReference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CompleteRequestParams {
    pub argument: CompleteRequestParamsArgument,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub context: ::std::option::Option<CompleteRequestParamsContext>,
    #[serde(rename = "ref")]
    pub ref_: CompleteRequestParamsRef,
}
impl ::std::convert::From<&CompleteRequestParams> for CompleteRequestParams {
    fn from(value: &CompleteRequestParams) -> Self {
        value.clone()
    }
}
impl CompleteRequestParams {
    pub fn builder() -> builder::CompleteRequestParams {
        Default::default()
    }
}
#[doc = "The argument's information"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The argument's information\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The name of the argument\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"description\": \"The value of the argument to use for completion matching.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CompleteRequestParamsArgument {
    #[doc = "The name of the argument"]
    pub name: ::std::string::String,
    #[doc = "The value of the argument to use for completion matching."]
    pub value: ::std::string::String,
}
impl ::std::convert::From<&CompleteRequestParamsArgument> for CompleteRequestParamsArgument {
    fn from(value: &CompleteRequestParamsArgument) -> Self {
        value.clone()
    }
}
impl CompleteRequestParamsArgument {
    pub fn builder() -> builder::CompleteRequestParamsArgument {
        Default::default()
    }
}
#[doc = "Additional, optional context for completions"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Additional, optional context for completions\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"arguments\": {"]
#[doc = "      \"description\": \"Previously-resolved variables in a URI template or prompt.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct CompleteRequestParamsContext {
    #[doc = "Previously-resolved variables in a URI template or prompt."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub arguments: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
}
impl ::std::convert::From<&CompleteRequestParamsContext> for CompleteRequestParamsContext {
    fn from(value: &CompleteRequestParamsContext) -> Self {
        value.clone()
    }
}
impl CompleteRequestParamsContext {
    pub fn builder() -> builder::CompleteRequestParamsContext {
        Default::default()
    }
}
#[doc = "`CompleteRequestParamsRef`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/PromptReference\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ResourceTemplateReference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CompleteRequestParamsRef {
    PromptReference(PromptReference),
    ResourceTemplateReference(ResourceTemplateReference),
}
impl ::std::convert::From<&Self> for CompleteRequestParamsRef {
    fn from(value: &CompleteRequestParamsRef) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<PromptReference> for CompleteRequestParamsRef {
    fn from(value: PromptReference) -> Self {
        Self::PromptReference(value)
    }
}
impl ::std::convert::From<ResourceTemplateReference> for CompleteRequestParamsRef {
    fn from(value: ResourceTemplateReference) -> Self {
        Self::ResourceTemplateReference(value)
    }
}
#[doc = "The server's response to a completion/complete request"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The server's response to a completion/complete request\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"completion\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"completion\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"values\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"hasMore\": {"]
#[doc = "          \"description\": \"Indicates whether there are additional completion options beyond those provided in the current response, even if the exact total is unknown.\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"total\": {"]
#[doc = "          \"description\": \"The total number of completion options available. This can exceed the number of values actually sent in the response.\","]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"values\": {"]
#[doc = "          \"description\": \"An array of completion values. Must not exceed 100 items.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CompleteResult {
    pub completion: CompleteResultCompletion,
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&CompleteResult> for CompleteResult {
    fn from(value: &CompleteResult) -> Self {
        value.clone()
    }
}
impl CompleteResult {
    pub fn builder() -> builder::CompleteResult {
        Default::default()
    }
}
#[doc = "`CompleteResultCompletion`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"values\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"hasMore\": {"]
#[doc = "      \"description\": \"Indicates whether there are additional completion options beyond those provided in the current response, even if the exact total is unknown.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"total\": {"]
#[doc = "      \"description\": \"The total number of completion options available. This can exceed the number of values actually sent in the response.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"values\": {"]
#[doc = "      \"description\": \"An array of completion values. Must not exceed 100 items.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CompleteResultCompletion {
    #[doc = "Indicates whether there are additional completion options beyond those provided in the current response, even if the exact total is unknown."]
    #[serde(
        rename = "hasMore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_more: ::std::option::Option<bool>,
    #[doc = "The total number of completion options available. This can exceed the number of values actually sent in the response."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub total: ::std::option::Option<i64>,
    #[doc = "An array of completion values. Must not exceed 100 items."]
    pub values: ::std::vec::Vec<::std::string::String>,
}
impl ::std::convert::From<&CompleteResultCompletion> for CompleteResultCompletion {
    fn from(value: &CompleteResultCompletion) -> Self {
        value.clone()
    }
}
impl CompleteResultCompletion {
    pub fn builder() -> builder::CompleteResultCompletion {
        Default::default()
    }
}
#[doc = "`ContentBlock`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TextContent\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ImageContent\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/AudioContent\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ResourceLink\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/EmbeddedResource\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ContentBlock {
    TextContent(TextContent),
    ImageContent(ImageContent),
    AudioContent(AudioContent),
    ResourceLink(ResourceLink),
    EmbeddedResource(EmbeddedResource),
}
impl ::std::convert::From<&Self> for ContentBlock {
    fn from(value: &ContentBlock) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<TextContent> for ContentBlock {
    fn from(value: TextContent) -> Self {
        Self::TextContent(value)
    }
}
impl ::std::convert::From<ImageContent> for ContentBlock {
    fn from(value: ImageContent) -> Self {
        Self::ImageContent(value)
    }
}
impl ::std::convert::From<AudioContent> for ContentBlock {
    fn from(value: AudioContent) -> Self {
        Self::AudioContent(value)
    }
}
impl ::std::convert::From<ResourceLink> for ContentBlock {
    fn from(value: ResourceLink) -> Self {
        Self::ResourceLink(value)
    }
}
impl ::std::convert::From<EmbeddedResource> for ContentBlock {
    fn from(value: EmbeddedResource) -> Self {
        Self::EmbeddedResource(value)
    }
}
#[doc = "A request from the server to sample an LLM via the client. The client has full discretion over which model to select. The client should also inform the user before beginning sampling, to allow them to inspect the request (human in the loop) and decide whether to approve it."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A request from the server to sample an LLM via the client. The client has full discretion over which model to select. The client should also inform the user before beginning sampling, to allow them to inspect the request (human in the loop) and decide whether to approve it.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"sampling/createMessage\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"maxTokens\","]
#[doc = "        \"messages\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"includeContext\": {"]
#[doc = "          \"description\": \"A request to include context from one or more MCP servers (including the caller), to be attached to the prompt. The client MAY ignore this request.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"allServers\","]
#[doc = "            \"none\","]
#[doc = "            \"thisServer\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"maxTokens\": {"]
#[doc = "          \"description\": \"The maximum number of tokens to sample, as requested by the server. The client MAY choose to sample fewer tokens than requested.\","]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"messages\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/SamplingMessage\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"metadata\": {"]
#[doc = "          \"description\": \"Optional metadata to pass through to the LLM provider. The format of this metadata is provider-specific.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": true"]
#[doc = "        },"]
#[doc = "        \"modelPreferences\": {"]
#[doc = "          \"description\": \"The server's preferences for which model to select. The client MAY ignore these preferences.\","]
#[doc = "          \"$ref\": \"#/definitions/ModelPreferences\""]
#[doc = "        },"]
#[doc = "        \"stopSequences\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"systemPrompt\": {"]
#[doc = "          \"description\": \"An optional system prompt the server wants to use for sampling. The client MAY modify or omit this prompt.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"temperature\": {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateMessageRequest {
    pub method: ::std::string::String,
    pub params: CreateMessageRequestParams,
}
impl ::std::convert::From<&CreateMessageRequest> for CreateMessageRequest {
    fn from(value: &CreateMessageRequest) -> Self {
        value.clone()
    }
}
impl CreateMessageRequest {
    pub fn builder() -> builder::CreateMessageRequest {
        Default::default()
    }
}
#[doc = "`CreateMessageRequestParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"maxTokens\","]
#[doc = "    \"messages\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"includeContext\": {"]
#[doc = "      \"description\": \"A request to include context from one or more MCP servers (including the caller), to be attached to the prompt. The client MAY ignore this request.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"allServers\","]
#[doc = "        \"none\","]
#[doc = "        \"thisServer\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"maxTokens\": {"]
#[doc = "      \"description\": \"The maximum number of tokens to sample, as requested by the server. The client MAY choose to sample fewer tokens than requested.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"messages\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/SamplingMessage\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Optional metadata to pass through to the LLM provider. The format of this metadata is provider-specific.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"modelPreferences\": {"]
#[doc = "      \"description\": \"The server's preferences for which model to select. The client MAY ignore these preferences.\","]
#[doc = "      \"$ref\": \"#/definitions/ModelPreferences\""]
#[doc = "    },"]
#[doc = "    \"stopSequences\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"systemPrompt\": {"]
#[doc = "      \"description\": \"An optional system prompt the server wants to use for sampling. The client MAY modify or omit this prompt.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"temperature\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateMessageRequestParams {
    #[doc = "A request to include context from one or more MCP servers (including the caller), to be attached to the prompt. The client MAY ignore this request."]
    #[serde(
        rename = "includeContext",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub include_context: ::std::option::Option<CreateMessageRequestParamsIncludeContext>,
    #[doc = "The maximum number of tokens to sample, as requested by the server. The client MAY choose to sample fewer tokens than requested."]
    #[serde(rename = "maxTokens")]
    pub max_tokens: i64,
    pub messages: ::std::vec::Vec<SamplingMessage>,
    #[doc = "Optional metadata to pass through to the LLM provider. The format of this metadata is provider-specific."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The server's preferences for which model to select. The client MAY ignore these preferences."]
    #[serde(
        rename = "modelPreferences",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub model_preferences: ::std::option::Option<ModelPreferences>,
    #[serde(
        rename = "stopSequences",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub stop_sequences: ::std::vec::Vec<::std::string::String>,
    #[doc = "An optional system prompt the server wants to use for sampling. The client MAY modify or omit this prompt."]
    #[serde(
        rename = "systemPrompt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub system_prompt: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub temperature: ::std::option::Option<f64>,
}
impl ::std::convert::From<&CreateMessageRequestParams> for CreateMessageRequestParams {
    fn from(value: &CreateMessageRequestParams) -> Self {
        value.clone()
    }
}
impl CreateMessageRequestParams {
    pub fn builder() -> builder::CreateMessageRequestParams {
        Default::default()
    }
}
#[doc = "A request to include context from one or more MCP servers (including the caller), to be attached to the prompt. The client MAY ignore this request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A request to include context from one or more MCP servers (including the caller), to be attached to the prompt. The client MAY ignore this request.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"allServers\","]
#[doc = "    \"none\","]
#[doc = "    \"thisServer\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateMessageRequestParamsIncludeContext {
    #[serde(rename = "allServers")]
    AllServers,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "thisServer")]
    ThisServer,
}
impl ::std::convert::From<&Self> for CreateMessageRequestParamsIncludeContext {
    fn from(value: &CreateMessageRequestParamsIncludeContext) -> Self {
        *value
    }
}
impl ::std::fmt::Display for CreateMessageRequestParamsIncludeContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AllServers => write!(f, "allServers"),
            Self::None => write!(f, "none"),
            Self::ThisServer => write!(f, "thisServer"),
        }
    }
}
impl ::std::str::FromStr for CreateMessageRequestParamsIncludeContext {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "allServers" => Ok(Self::AllServers),
            "none" => Ok(Self::None),
            "thisServer" => Ok(Self::ThisServer),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateMessageRequestParamsIncludeContext {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CreateMessageRequestParamsIncludeContext {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateMessageRequestParamsIncludeContext {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The client's response to a sampling/create_message request from the server. The client should inform the user before returning the sampled message, to allow them to inspect the response (human in the loop) and decide whether to allow the server to see it."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The client's response to a sampling/create_message request from the server. The client should inform the user before returning the sampled message, to allow them to inspect the response (human in the loop) and decide whether to allow the server to see it.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"content\","]
#[doc = "    \"model\","]
#[doc = "    \"role\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"content\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/TextContent\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ImageContent\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AudioContent\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"model\": {"]
#[doc = "      \"description\": \"The name of the model that generated the message.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"role\": {"]
#[doc = "      \"$ref\": \"#/definitions/Role\""]
#[doc = "    },"]
#[doc = "    \"stopReason\": {"]
#[doc = "      \"description\": \"The reason why sampling stopped, if known.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateMessageResult {
    pub content: CreateMessageResultContent,
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The name of the model that generated the message."]
    pub model: ::std::string::String,
    pub role: Role,
    #[doc = "The reason why sampling stopped, if known."]
    #[serde(
        rename = "stopReason",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stop_reason: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&CreateMessageResult> for CreateMessageResult {
    fn from(value: &CreateMessageResult) -> Self {
        value.clone()
    }
}
impl CreateMessageResult {
    pub fn builder() -> builder::CreateMessageResult {
        Default::default()
    }
}
#[doc = "`CreateMessageResultContent`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TextContent\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ImageContent\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/AudioContent\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CreateMessageResultContent {
    TextContent(TextContent),
    ImageContent(ImageContent),
    AudioContent(AudioContent),
}
impl ::std::convert::From<&Self> for CreateMessageResultContent {
    fn from(value: &CreateMessageResultContent) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<TextContent> for CreateMessageResultContent {
    fn from(value: TextContent) -> Self {
        Self::TextContent(value)
    }
}
impl ::std::convert::From<ImageContent> for CreateMessageResultContent {
    fn from(value: ImageContent) -> Self {
        Self::ImageContent(value)
    }
}
impl ::std::convert::From<AudioContent> for CreateMessageResultContent {
    fn from(value: AudioContent) -> Self {
        Self::AudioContent(value)
    }
}
#[doc = "An opaque token used to represent a cursor for pagination."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An opaque token used to represent a cursor for pagination.\","]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(transparent)]
pub struct Cursor(pub ::std::string::String);
impl ::std::ops::Deref for Cursor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Cursor> for ::std::string::String {
    fn from(value: Cursor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Cursor> for Cursor {
    fn from(value: &Cursor) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for Cursor {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Cursor {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for Cursor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "A request from the server to elicit additional information from the user via the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A request from the server to elicit additional information from the user via the client.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"elicitation/create\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"message\","]
#[doc = "        \"requestedSchema\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"message\": {"]
#[doc = "          \"description\": \"The message to present to the user.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"requestedSchema\": {"]
#[doc = "          \"description\": \"A restricted subset of JSON Schema.\\nOnly top-level properties are allowed, without nesting.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"properties\","]
#[doc = "            \"type\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"properties\": {"]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"additionalProperties\": {"]
#[doc = "                \"$ref\": \"#/definitions/PrimitiveSchemaDefinition\""]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            \"required\": {"]
#[doc = "              \"type\": \"array\","]
#[doc = "              \"items\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"type\": \"string\","]
#[doc = "              \"const\": \"object\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ElicitRequest {
    pub method: ::std::string::String,
    pub params: ElicitRequestParams,
}
impl ::std::convert::From<&ElicitRequest> for ElicitRequest {
    fn from(value: &ElicitRequest) -> Self {
        value.clone()
    }
}
impl ElicitRequest {
    pub fn builder() -> builder::ElicitRequest {
        Default::default()
    }
}
#[doc = "`ElicitRequestParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"message\","]
#[doc = "    \"requestedSchema\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"The message to present to the user.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"requestedSchema\": {"]
#[doc = "      \"description\": \"A restricted subset of JSON Schema.\\nOnly top-level properties are allowed, without nesting.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"properties\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"properties\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"$ref\": \"#/definitions/PrimitiveSchemaDefinition\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"required\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"object\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ElicitRequestParams {
    #[doc = "The message to present to the user."]
    pub message: ::std::string::String,
    #[serde(rename = "requestedSchema")]
    pub requested_schema: ElicitRequestParamsRequestedSchema,
}
impl ::std::convert::From<&ElicitRequestParams> for ElicitRequestParams {
    fn from(value: &ElicitRequestParams) -> Self {
        value.clone()
    }
}
impl ElicitRequestParams {
    pub fn builder() -> builder::ElicitRequestParams {
        Default::default()
    }
}
#[doc = "A restricted subset of JSON Schema.\nOnly top-level properties are allowed, without nesting."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A restricted subset of JSON Schema.\\nOnly top-level properties are allowed, without nesting.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"properties\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"properties\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/definitions/PrimitiveSchemaDefinition\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"required\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"object\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ElicitRequestParamsRequestedSchema {
    pub properties: ::std::collections::HashMap<::std::string::String, PrimitiveSchemaDefinition>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub required: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&ElicitRequestParamsRequestedSchema>
    for ElicitRequestParamsRequestedSchema
{
    fn from(value: &ElicitRequestParamsRequestedSchema) -> Self {
        value.clone()
    }
}
impl ElicitRequestParamsRequestedSchema {
    pub fn builder() -> builder::ElicitRequestParamsRequestedSchema {
        Default::default()
    }
}
#[doc = "The client's response to an elicitation request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The client's response to an elicitation request.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"action\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"action\": {"]
#[doc = "      \"description\": \"The user action in response to the elicitation.\\n- \\\"accept\\\": User submitted the form/confirmed the action\\n- \\\"decline\\\": User explicitly declined the action\\n- \\\"cancel\\\": User dismissed without making an explicit choice\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"accept\","]
#[doc = "        \"cancel\","]
#[doc = "        \"decline\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"content\": {"]
#[doc = "      \"description\": \"The submitted form data, only present when action is \\\"accept\\\".\\nContains values matching the requested schema.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": ["]
#[doc = "          \"string\","]
#[doc = "          \"integer\","]
#[doc = "          \"boolean\""]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ElicitResult {
    #[doc = "The user action in response to the elicitation.\n- \"accept\": User submitted the form/confirmed the action\n- \"decline\": User explicitly declined the action\n- \"cancel\": User dismissed without making an explicit choice"]
    pub action: ElicitResultAction,
    #[doc = "The submitted form data, only present when action is \"accept\".\nContains values matching the requested schema."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub content: ::std::collections::HashMap<::std::string::String, ElicitResultContentValue>,
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&ElicitResult> for ElicitResult {
    fn from(value: &ElicitResult) -> Self {
        value.clone()
    }
}
impl ElicitResult {
    pub fn builder() -> builder::ElicitResult {
        Default::default()
    }
}
#[doc = "The user action in response to the elicitation.\n- \"accept\": User submitted the form/confirmed the action\n- \"decline\": User explicitly declined the action\n- \"cancel\": User dismissed without making an explicit choice"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The user action in response to the elicitation.\\n- \\\"accept\\\": User submitted the form/confirmed the action\\n- \\\"decline\\\": User explicitly declined the action\\n- \\\"cancel\\\": User dismissed without making an explicit choice\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"accept\","]
#[doc = "    \"cancel\","]
#[doc = "    \"decline\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ElicitResultAction {
    #[serde(rename = "accept")]
    Accept,
    #[serde(rename = "cancel")]
    Cancel,
    #[serde(rename = "decline")]
    Decline,
}
impl ::std::convert::From<&Self> for ElicitResultAction {
    fn from(value: &ElicitResultAction) -> Self {
        *value
    }
}
impl ::std::fmt::Display for ElicitResultAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Accept => write!(f, "accept"),
            Self::Cancel => write!(f, "cancel"),
            Self::Decline => write!(f, "decline"),
        }
    }
}
impl ::std::str::FromStr for ElicitResultAction {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "accept" => Ok(Self::Accept),
            "cancel" => Ok(Self::Cancel),
            "decline" => Ok(Self::Decline),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ElicitResultAction {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ElicitResultAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ElicitResultAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ElicitResultContentValue`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\","]
#[doc = "    \"boolean\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ElicitResultContentValue {
    Boolean(bool),
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<&Self> for ElicitResultContentValue {
    fn from(value: &ElicitResultContentValue) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ElicitResultContentValue {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Boolean(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for ElicitResultContentValue {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ElicitResultContentValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ElicitResultContentValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for ElicitResultContentValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Boolean(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool> for ElicitResultContentValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl ::std::convert::From<i64> for ElicitResultContentValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "The contents of a resource, embedded into a prompt or tool call result.\n\nIt is up to the client how best to render embedded resources for the benefit\nof the LLM and/or the user."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The contents of a resource, embedded into a prompt or tool call result.\\n\\nIt is up to the client how best to render embedded resources for the benefit\\nof the LLM and/or the user.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"resource\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"annotations\": {"]
#[doc = "      \"description\": \"Optional annotations for the client.\","]
#[doc = "      \"$ref\": \"#/definitions/Annotations\""]
#[doc = "    },"]
#[doc = "    \"resource\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/TextResourceContents\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/BlobResourceContents\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"resource\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct EmbeddedResource {
    #[doc = "Optional annotations for the client."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations: ::std::option::Option<Annotations>,
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub resource: EmbeddedResourceResource,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&EmbeddedResource> for EmbeddedResource {
    fn from(value: &EmbeddedResource) -> Self {
        value.clone()
    }
}
impl EmbeddedResource {
    pub fn builder() -> builder::EmbeddedResource {
        Default::default()
    }
}
#[doc = "`EmbeddedResourceResource`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TextResourceContents\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/BlobResourceContents\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum EmbeddedResourceResource {
    TextResourceContents(TextResourceContents),
    BlobResourceContents(BlobResourceContents),
}
impl ::std::convert::From<&Self> for EmbeddedResourceResource {
    fn from(value: &EmbeddedResourceResource) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<TextResourceContents> for EmbeddedResourceResource {
    fn from(value: TextResourceContents) -> Self {
        Self::TextResourceContents(value)
    }
}
impl ::std::convert::From<BlobResourceContents> for EmbeddedResourceResource {
    fn from(value: BlobResourceContents) -> Self {
        Self::BlobResourceContents(value)
    }
}
#[doc = "`EmptyResult`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$ref\": \"#/definitions/Result\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct EmptyResult(pub Result);
impl ::std::ops::Deref for EmptyResult {
    type Target = Result;
    fn deref(&self) -> &Result {
        &self.0
    }
}
impl ::std::convert::From<EmptyResult> for Result {
    fn from(value: EmptyResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmptyResult> for EmptyResult {
    fn from(value: &EmptyResult) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<Result> for EmptyResult {
    fn from(value: Result) -> Self {
        Self(value)
    }
}
#[doc = "`EnumSchema`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"enum\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"enum\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"enumNames\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct EnumSchema {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enum")]
    pub enum_: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "enumNames",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub enum_names: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&EnumSchema> for EnumSchema {
    fn from(value: &EnumSchema) -> Self {
        value.clone()
    }
}
impl EnumSchema {
    pub fn builder() -> builder::EnumSchema {
        Default::default()
    }
}
#[doc = "Used by the client to get a prompt provided by the server."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Used by the client to get a prompt provided by the server.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"prompts/get\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"name\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"arguments\": {"]
#[doc = "          \"description\": \"Arguments to use for templating the prompt.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"name\": {"]
#[doc = "          \"description\": \"The name of the prompt or prompt template.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GetPromptRequest {
    pub method: ::std::string::String,
    pub params: GetPromptRequestParams,
}
impl ::std::convert::From<&GetPromptRequest> for GetPromptRequest {
    fn from(value: &GetPromptRequest) -> Self {
        value.clone()
    }
}
impl GetPromptRequest {
    pub fn builder() -> builder::GetPromptRequest {
        Default::default()
    }
}
#[doc = "`GetPromptRequestParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"arguments\": {"]
#[doc = "      \"description\": \"Arguments to use for templating the prompt.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The name of the prompt or prompt template.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GetPromptRequestParams {
    #[doc = "Arguments to use for templating the prompt."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub arguments: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[doc = "The name of the prompt or prompt template."]
    pub name: ::std::string::String,
}
impl ::std::convert::From<&GetPromptRequestParams> for GetPromptRequestParams {
    fn from(value: &GetPromptRequestParams) -> Self {
        value.clone()
    }
}
impl GetPromptRequestParams {
    pub fn builder() -> builder::GetPromptRequestParams {
        Default::default()
    }
}
#[doc = "The server's response to a prompts/get request from the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The server's response to a prompts/get request from the client.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"messages\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"An optional description for the prompt.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"messages\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/PromptMessage\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GetPromptResult {
    #[doc = "An optional description for the prompt."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub messages: ::std::vec::Vec<PromptMessage>,
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&GetPromptResult> for GetPromptResult {
    fn from(value: &GetPromptResult) -> Self {
        value.clone()
    }
}
impl GetPromptResult {
    pub fn builder() -> builder::GetPromptResult {
        Default::default()
    }
}
#[doc = "An image provided to or from an LLM."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An image provided to or from an LLM.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\","]
#[doc = "    \"mimeType\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"annotations\": {"]
#[doc = "      \"description\": \"Optional annotations for the client.\","]
#[doc = "      \"$ref\": \"#/definitions/Annotations\""]
#[doc = "    },"]
#[doc = "    \"data\": {"]
#[doc = "      \"description\": \"The base64-encoded image data.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"byte\""]
#[doc = "    },"]
#[doc = "    \"mimeType\": {"]
#[doc = "      \"description\": \"The MIME type of the image. Different providers may support different image types.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"image\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ImageContent {
    #[doc = "Optional annotations for the client."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations: ::std::option::Option<Annotations>,
    #[doc = "The base64-encoded image data."]
    pub data: ::std::string::String,
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The MIME type of the image. Different providers may support different image types."]
    #[serde(rename = "mimeType")]
    pub mime_type: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&ImageContent> for ImageContent {
    fn from(value: &ImageContent) -> Self {
        value.clone()
    }
}
impl ImageContent {
    pub fn builder() -> builder::ImageContent {
        Default::default()
    }
}
#[doc = "Describes the name and version of an MCP implementation, with an optional title for UI representation."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Describes the name and version of an MCP implementation, with an optional title for UI representation.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"description\": \"Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\\neven by those unfamiliar with domain-specific terminology.\\n\\nIf not provided, the name should be used for display (except for Tool,\\nwhere `annotations.title` should be given precedence over using `name`,\\nif present).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Implementation {
    #[doc = "Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present)."]
    pub name: ::std::string::String,
    #[doc = "Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\neven by those unfamiliar with domain-specific terminology.\n\nIf not provided, the name should be used for display (except for Tool,\nwhere `annotations.title` should be given precedence over using `name`,\nif present)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    pub version: ::std::string::String,
}
impl ::std::convert::From<&Implementation> for Implementation {
    fn from(value: &Implementation) -> Self {
        value.clone()
    }
}
impl Implementation {
    pub fn builder() -> builder::Implementation {
        Default::default()
    }
}
#[doc = "This request is sent from the client to the server when it first connects, asking it to begin initialization."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"This request is sent from the client to the server when it first connects, asking it to begin initialization.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"initialize\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"capabilities\","]
#[doc = "        \"clientInfo\","]
#[doc = "        \"protocolVersion\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"capabilities\": {"]
#[doc = "          \"$ref\": \"#/definitions/ClientCapabilities\""]
#[doc = "        },"]
#[doc = "        \"clientInfo\": {"]
#[doc = "          \"$ref\": \"#/definitions/Implementation\""]
#[doc = "        },"]
#[doc = "        \"protocolVersion\": {"]
#[doc = "          \"description\": \"The latest version of the Model Context Protocol that the client supports. The client MAY decide to support older versions as well.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InitializeRequest {
    pub method: ::std::string::String,
    pub params: InitializeRequestParams,
}
impl ::std::convert::From<&InitializeRequest> for InitializeRequest {
    fn from(value: &InitializeRequest) -> Self {
        value.clone()
    }
}
impl InitializeRequest {
    pub fn builder() -> builder::InitializeRequest {
        Default::default()
    }
}
#[doc = "`InitializeRequestParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"capabilities\","]
#[doc = "    \"clientInfo\","]
#[doc = "    \"protocolVersion\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"capabilities\": {"]
#[doc = "      \"$ref\": \"#/definitions/ClientCapabilities\""]
#[doc = "    },"]
#[doc = "    \"clientInfo\": {"]
#[doc = "      \"$ref\": \"#/definitions/Implementation\""]
#[doc = "    },"]
#[doc = "    \"protocolVersion\": {"]
#[doc = "      \"description\": \"The latest version of the Model Context Protocol that the client supports. The client MAY decide to support older versions as well.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InitializeRequestParams {
    pub capabilities: ClientCapabilities,
    #[serde(rename = "clientInfo")]
    pub client_info: Implementation,
    #[doc = "The latest version of the Model Context Protocol that the client supports. The client MAY decide to support older versions as well."]
    #[serde(rename = "protocolVersion")]
    pub protocol_version: ::std::string::String,
}
impl ::std::convert::From<&InitializeRequestParams> for InitializeRequestParams {
    fn from(value: &InitializeRequestParams) -> Self {
        value.clone()
    }
}
impl InitializeRequestParams {
    pub fn builder() -> builder::InitializeRequestParams {
        Default::default()
    }
}
#[doc = "After receiving an initialize request from the client, the server sends this response."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"After receiving an initialize request from the client, the server sends this response.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"capabilities\","]
#[doc = "    \"protocolVersion\","]
#[doc = "    \"serverInfo\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"capabilities\": {"]
#[doc = "      \"$ref\": \"#/definitions/ServerCapabilities\""]
#[doc = "    },"]
#[doc = "    \"instructions\": {"]
#[doc = "      \"description\": \"Instructions describing how to use the server and its features.\\n\\nThis can be used by clients to improve the LLM's understanding of available tools, resources, etc. It can be thought of like a \\\"hint\\\" to the model. For example, this information MAY be added to the system prompt.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"protocolVersion\": {"]
#[doc = "      \"description\": \"The version of the Model Context Protocol that the server wants to use. This may not match the version that the client requested. If the client cannot support this version, it MUST disconnect.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"serverInfo\": {"]
#[doc = "      \"$ref\": \"#/definitions/Implementation\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InitializeResult {
    pub capabilities: ServerCapabilities,
    #[doc = "Instructions describing how to use the server and its features.\n\nThis can be used by clients to improve the LLM's understanding of available tools, resources, etc. It can be thought of like a \"hint\" to the model. For example, this information MAY be added to the system prompt."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The version of the Model Context Protocol that the server wants to use. This may not match the version that the client requested. If the client cannot support this version, it MUST disconnect."]
    #[serde(rename = "protocolVersion")]
    pub protocol_version: ::std::string::String,
    #[serde(rename = "serverInfo")]
    pub server_info: Implementation,
}
impl ::std::convert::From<&InitializeResult> for InitializeResult {
    fn from(value: &InitializeResult) -> Self {
        value.clone()
    }
}
impl InitializeResult {
    pub fn builder() -> builder::InitializeResult {
        Default::default()
    }
}
#[doc = "This notification is sent from the client to the server after initialization has finished."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"This notification is sent from the client to the server after initialization has finished.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"notifications/initialized\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"_meta\": {"]
#[doc = "          \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {}"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InitializedNotification {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<InitializedNotificationParams>,
}
impl ::std::convert::From<&InitializedNotification> for InitializedNotification {
    fn from(value: &InitializedNotification) -> Self {
        value.clone()
    }
}
impl InitializedNotification {
    pub fn builder() -> builder::InitializedNotification {
        Default::default()
    }
}
#[doc = "`InitializedNotificationParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InitializedNotificationParams {
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&InitializedNotificationParams> for InitializedNotificationParams {
    fn from(value: &InitializedNotificationParams) -> Self {
        value.clone()
    }
}
impl InitializedNotificationParams {
    pub fn builder() -> builder::InitializedNotificationParams {
        Default::default()
    }
}
#[doc = "A response to a request that indicates an error occurred."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A response to a request that indicates an error occurred.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"error\","]
#[doc = "    \"id\","]
#[doc = "    \"jsonrpc\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"error\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"code\","]
#[doc = "        \"message\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"code\": {"]
#[doc = "          \"description\": \"The error type that occurred.\","]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"data\": {"]
#[doc = "          \"description\": \"Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.).\""]
#[doc = "        },"]
#[doc = "        \"message\": {"]
#[doc = "          \"description\": \"A short description of the error. The message SHOULD be limited to a concise single sentence.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"$ref\": \"#/definitions/RequestId\""]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcError {
    pub error: JsonrpcErrorError,
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
}
impl ::std::convert::From<&JsonrpcError> for JsonrpcError {
    fn from(value: &JsonrpcError) -> Self {
        value.clone()
    }
}
impl JsonrpcError {
    pub fn builder() -> builder::JsonrpcError {
        Default::default()
    }
}
#[doc = "`JsonrpcErrorError`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"code\","]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"The error type that occurred.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"data\": {"]
#[doc = "      \"description\": \"Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.).\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"A short description of the error. The message SHOULD be limited to a concise single sentence.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcErrorError {
    #[doc = "The error type that occurred."]
    pub code: i64,
    #[doc = "Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::serde_json::Value>,
    #[doc = "A short description of the error. The message SHOULD be limited to a concise single sentence."]
    pub message: ::std::string::String,
}
impl ::std::convert::From<&JsonrpcErrorError> for JsonrpcErrorError {
    fn from(value: &JsonrpcErrorError) -> Self {
        value.clone()
    }
}
impl JsonrpcErrorError {
    pub fn builder() -> builder::JsonrpcErrorError {
        Default::default()
    }
}
#[doc = "Refers to any valid JSON-RPC object that can be decoded off the wire, or encoded to be sent."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Refers to any valid JSON-RPC object that can be decoded off the wire, or encoded to be sent.\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/JSONRPCRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/JSONRPCNotification\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/JSONRPCResponse\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/JSONRPCError\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonrpcMessage {
    Request(JsonrpcRequest),
    Notification(JsonrpcNotification),
    Response(JsonrpcResponse),
    Error(JsonrpcError),
}
impl ::std::convert::From<&Self> for JsonrpcMessage {
    fn from(value: &JsonrpcMessage) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<JsonrpcRequest> for JsonrpcMessage {
    fn from(value: JsonrpcRequest) -> Self {
        Self::Request(value)
    }
}
impl ::std::convert::From<JsonrpcNotification> for JsonrpcMessage {
    fn from(value: JsonrpcNotification) -> Self {
        Self::Notification(value)
    }
}
impl ::std::convert::From<JsonrpcResponse> for JsonrpcMessage {
    fn from(value: JsonrpcResponse) -> Self {
        Self::Response(value)
    }
}
impl ::std::convert::From<JsonrpcError> for JsonrpcMessage {
    fn from(value: JsonrpcError) -> Self {
        Self::Error(value)
    }
}
#[doc = "A notification which does not expect a response."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A notification which does not expect a response.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"jsonrpc\","]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"_meta\": {"]
#[doc = "          \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {}"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcNotification {
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<JsonrpcNotificationParams>,
}
impl ::std::convert::From<&JsonrpcNotification> for JsonrpcNotification {
    fn from(value: &JsonrpcNotification) -> Self {
        value.clone()
    }
}
impl JsonrpcNotification {
    pub fn builder() -> builder::JsonrpcNotification {
        Default::default()
    }
}
#[doc = "`JsonrpcNotificationParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcNotificationParams {
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&JsonrpcNotificationParams> for JsonrpcNotificationParams {
    fn from(value: &JsonrpcNotificationParams) -> Self {
        value.clone()
    }
}
impl JsonrpcNotificationParams {
    pub fn builder() -> builder::JsonrpcNotificationParams {
        Default::default()
    }
}
#[doc = "A request that expects a response."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A request that expects a response.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"jsonrpc\","]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"$ref\": \"#/definitions/RequestId\""]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"_meta\": {"]
#[doc = "          \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"progressToken\": {"]
#[doc = "              \"description\": \"If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.\","]
#[doc = "              \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": {}"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcRequest {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<JsonrpcRequestParams>,
}
impl ::std::convert::From<&JsonrpcRequest> for JsonrpcRequest {
    fn from(value: &JsonrpcRequest) -> Self {
        value.clone()
    }
}
impl JsonrpcRequest {
    pub fn builder() -> builder::JsonrpcRequest {
        Default::default()
    }
}
#[doc = "`JsonrpcRequestParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"progressToken\": {"]
#[doc = "          \"description\": \"If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.\","]
#[doc = "          \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcRequestParams {
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<JsonrpcRequestParamsMeta>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&JsonrpcRequestParams> for JsonrpcRequestParams {
    fn from(value: &JsonrpcRequestParams) -> Self {
        value.clone()
    }
}
impl JsonrpcRequestParams {
    pub fn builder() -> builder::JsonrpcRequestParams {
        Default::default()
    }
}
#[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"progressToken\": {"]
#[doc = "      \"description\": \"If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.\","]
#[doc = "      \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcRequestParamsMeta {
    #[doc = "If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications."]
    #[serde(
        rename = "progressToken",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub progress_token: ::std::option::Option<ProgressToken>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&JsonrpcRequestParamsMeta> for JsonrpcRequestParamsMeta {
    fn from(value: &JsonrpcRequestParamsMeta) -> Self {
        value.clone()
    }
}
impl JsonrpcRequestParamsMeta {
    pub fn builder() -> builder::JsonrpcRequestParamsMeta {
        Default::default()
    }
}
#[doc = "A successful (non-error) response to a request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A successful (non-error) response to a request.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"jsonrpc\","]
#[doc = "    \"result\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"$ref\": \"#/definitions/RequestId\""]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"result\": {"]
#[doc = "      \"$ref\": \"#/definitions/Result\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcResponse {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub result: Result,
}
impl ::std::convert::From<&JsonrpcResponse> for JsonrpcResponse {
    fn from(value: &JsonrpcResponse) -> Self {
        value.clone()
    }
}
impl JsonrpcResponse {
    pub fn builder() -> builder::JsonrpcResponse {
        Default::default()
    }
}
#[doc = "Sent from the client to request a list of prompts and prompt templates the server has."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Sent from the client to request a list of prompts and prompt templates the server has.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"prompts/list\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"cursor\": {"]
#[doc = "          \"description\": \"An opaque token representing the current pagination position.\\nIf provided, the server should return results starting after this cursor.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListPromptsRequest {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<ListPromptsRequestParams>,
}
impl ::std::convert::From<&ListPromptsRequest> for ListPromptsRequest {
    fn from(value: &ListPromptsRequest) -> Self {
        value.clone()
    }
}
impl ListPromptsRequest {
    pub fn builder() -> builder::ListPromptsRequest {
        Default::default()
    }
}
#[doc = "`ListPromptsRequestParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cursor\": {"]
#[doc = "      \"description\": \"An opaque token representing the current pagination position.\\nIf provided, the server should return results starting after this cursor.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ListPromptsRequestParams {
    #[doc = "An opaque token representing the current pagination position.\nIf provided, the server should return results starting after this cursor."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cursor: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ListPromptsRequestParams> for ListPromptsRequestParams {
    fn from(value: &ListPromptsRequestParams) -> Self {
        value.clone()
    }
}
impl ListPromptsRequestParams {
    pub fn builder() -> builder::ListPromptsRequestParams {
        Default::default()
    }
}
#[doc = "The server's response to a prompts/list request from the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The server's response to a prompts/list request from the client.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"prompts\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"nextCursor\": {"]
#[doc = "      \"description\": \"An opaque token representing the pagination position after the last returned result.\\nIf present, there may be more results available.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"prompts\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Prompt\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListPromptsResult {
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "An opaque token representing the pagination position after the last returned result.\nIf present, there may be more results available."]
    #[serde(
        rename = "nextCursor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub next_cursor: ::std::option::Option<::std::string::String>,
    pub prompts: ::std::vec::Vec<Prompt>,
}
impl ::std::convert::From<&ListPromptsResult> for ListPromptsResult {
    fn from(value: &ListPromptsResult) -> Self {
        value.clone()
    }
}
impl ListPromptsResult {
    pub fn builder() -> builder::ListPromptsResult {
        Default::default()
    }
}
#[doc = "Sent from the client to request a list of resource templates the server has."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Sent from the client to request a list of resource templates the server has.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"resources/templates/list\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"cursor\": {"]
#[doc = "          \"description\": \"An opaque token representing the current pagination position.\\nIf provided, the server should return results starting after this cursor.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListResourceTemplatesRequest {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<ListResourceTemplatesRequestParams>,
}
impl ::std::convert::From<&ListResourceTemplatesRequest> for ListResourceTemplatesRequest {
    fn from(value: &ListResourceTemplatesRequest) -> Self {
        value.clone()
    }
}
impl ListResourceTemplatesRequest {
    pub fn builder() -> builder::ListResourceTemplatesRequest {
        Default::default()
    }
}
#[doc = "`ListResourceTemplatesRequestParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cursor\": {"]
#[doc = "      \"description\": \"An opaque token representing the current pagination position.\\nIf provided, the server should return results starting after this cursor.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ListResourceTemplatesRequestParams {
    #[doc = "An opaque token representing the current pagination position.\nIf provided, the server should return results starting after this cursor."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cursor: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ListResourceTemplatesRequestParams>
    for ListResourceTemplatesRequestParams
{
    fn from(value: &ListResourceTemplatesRequestParams) -> Self {
        value.clone()
    }
}
impl ListResourceTemplatesRequestParams {
    pub fn builder() -> builder::ListResourceTemplatesRequestParams {
        Default::default()
    }
}
#[doc = "The server's response to a resources/templates/list request from the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The server's response to a resources/templates/list request from the client.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"resourceTemplates\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"nextCursor\": {"]
#[doc = "      \"description\": \"An opaque token representing the pagination position after the last returned result.\\nIf present, there may be more results available.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"resourceTemplates\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ResourceTemplate\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListResourceTemplatesResult {
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "An opaque token representing the pagination position after the last returned result.\nIf present, there may be more results available."]
    #[serde(
        rename = "nextCursor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub next_cursor: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceTemplates")]
    pub resource_templates: ::std::vec::Vec<ResourceTemplate>,
}
impl ::std::convert::From<&ListResourceTemplatesResult> for ListResourceTemplatesResult {
    fn from(value: &ListResourceTemplatesResult) -> Self {
        value.clone()
    }
}
impl ListResourceTemplatesResult {
    pub fn builder() -> builder::ListResourceTemplatesResult {
        Default::default()
    }
}
#[doc = "Sent from the client to request a list of resources the server has."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Sent from the client to request a list of resources the server has.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"resources/list\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"cursor\": {"]
#[doc = "          \"description\": \"An opaque token representing the current pagination position.\\nIf provided, the server should return results starting after this cursor.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListResourcesRequest {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<ListResourcesRequestParams>,
}
impl ::std::convert::From<&ListResourcesRequest> for ListResourcesRequest {
    fn from(value: &ListResourcesRequest) -> Self {
        value.clone()
    }
}
impl ListResourcesRequest {
    pub fn builder() -> builder::ListResourcesRequest {
        Default::default()
    }
}
#[doc = "`ListResourcesRequestParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cursor\": {"]
#[doc = "      \"description\": \"An opaque token representing the current pagination position.\\nIf provided, the server should return results starting after this cursor.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ListResourcesRequestParams {
    #[doc = "An opaque token representing the current pagination position.\nIf provided, the server should return results starting after this cursor."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cursor: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ListResourcesRequestParams> for ListResourcesRequestParams {
    fn from(value: &ListResourcesRequestParams) -> Self {
        value.clone()
    }
}
impl ListResourcesRequestParams {
    pub fn builder() -> builder::ListResourcesRequestParams {
        Default::default()
    }
}
#[doc = "The server's response to a resources/list request from the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The server's response to a resources/list request from the client.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"resources\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"nextCursor\": {"]
#[doc = "      \"description\": \"An opaque token representing the pagination position after the last returned result.\\nIf present, there may be more results available.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"resources\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Resource\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListResourcesResult {
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "An opaque token representing the pagination position after the last returned result.\nIf present, there may be more results available."]
    #[serde(
        rename = "nextCursor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub next_cursor: ::std::option::Option<::std::string::String>,
    pub resources: ::std::vec::Vec<Resource>,
}
impl ::std::convert::From<&ListResourcesResult> for ListResourcesResult {
    fn from(value: &ListResourcesResult) -> Self {
        value.clone()
    }
}
impl ListResourcesResult {
    pub fn builder() -> builder::ListResourcesResult {
        Default::default()
    }
}
#[doc = "Sent from the server to request a list of root URIs from the client. Roots allow\nservers to ask for specific directories or files to operate on. A common example\nfor roots is providing a set of repositories or directories a server should operate\non.\n\nThis request is typically used when the server needs to understand the file system\nstructure or access specific locations that the client has permission to read from."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Sent from the server to request a list of root URIs from the client. Roots allow\\nservers to ask for specific directories or files to operate on. A common example\\nfor roots is providing a set of repositories or directories a server should operate\\non.\\n\\nThis request is typically used when the server needs to understand the file system\\nstructure or access specific locations that the client has permission to read from.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"roots/list\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"_meta\": {"]
#[doc = "          \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"progressToken\": {"]
#[doc = "              \"description\": \"If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.\","]
#[doc = "              \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": {}"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListRootsRequest {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<ListRootsRequestParams>,
}
impl ::std::convert::From<&ListRootsRequest> for ListRootsRequest {
    fn from(value: &ListRootsRequest) -> Self {
        value.clone()
    }
}
impl ListRootsRequest {
    pub fn builder() -> builder::ListRootsRequest {
        Default::default()
    }
}
#[doc = "`ListRootsRequestParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"progressToken\": {"]
#[doc = "          \"description\": \"If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.\","]
#[doc = "          \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListRootsRequestParams {
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<ListRootsRequestParamsMeta>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&ListRootsRequestParams> for ListRootsRequestParams {
    fn from(value: &ListRootsRequestParams) -> Self {
        value.clone()
    }
}
impl ListRootsRequestParams {
    pub fn builder() -> builder::ListRootsRequestParams {
        Default::default()
    }
}
#[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"progressToken\": {"]
#[doc = "      \"description\": \"If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.\","]
#[doc = "      \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListRootsRequestParamsMeta {
    #[doc = "If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications."]
    #[serde(
        rename = "progressToken",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub progress_token: ::std::option::Option<ProgressToken>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&ListRootsRequestParamsMeta> for ListRootsRequestParamsMeta {
    fn from(value: &ListRootsRequestParamsMeta) -> Self {
        value.clone()
    }
}
impl ListRootsRequestParamsMeta {
    pub fn builder() -> builder::ListRootsRequestParamsMeta {
        Default::default()
    }
}
#[doc = "The client's response to a roots/list request from the server.\nThis result contains an array of Root objects, each representing a root directory\nor file that the server can operate on."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The client's response to a roots/list request from the server.\\nThis result contains an array of Root objects, each representing a root directory\\nor file that the server can operate on.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"roots\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"roots\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Root\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListRootsResult {
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub roots: ::std::vec::Vec<Root>,
}
impl ::std::convert::From<&ListRootsResult> for ListRootsResult {
    fn from(value: &ListRootsResult) -> Self {
        value.clone()
    }
}
impl ListRootsResult {
    pub fn builder() -> builder::ListRootsResult {
        Default::default()
    }
}
#[doc = "Sent from the client to request a list of tools the server has."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Sent from the client to request a list of tools the server has.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"tools/list\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"cursor\": {"]
#[doc = "          \"description\": \"An opaque token representing the current pagination position.\\nIf provided, the server should return results starting after this cursor.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListToolsRequest {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<ListToolsRequestParams>,
}
impl ::std::convert::From<&ListToolsRequest> for ListToolsRequest {
    fn from(value: &ListToolsRequest) -> Self {
        value.clone()
    }
}
impl ListToolsRequest {
    pub fn builder() -> builder::ListToolsRequest {
        Default::default()
    }
}
#[doc = "`ListToolsRequestParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cursor\": {"]
#[doc = "      \"description\": \"An opaque token representing the current pagination position.\\nIf provided, the server should return results starting after this cursor.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ListToolsRequestParams {
    #[doc = "An opaque token representing the current pagination position.\nIf provided, the server should return results starting after this cursor."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cursor: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ListToolsRequestParams> for ListToolsRequestParams {
    fn from(value: &ListToolsRequestParams) -> Self {
        value.clone()
    }
}
impl ListToolsRequestParams {
    pub fn builder() -> builder::ListToolsRequestParams {
        Default::default()
    }
}
#[doc = "The server's response to a tools/list request from the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The server's response to a tools/list request from the client.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"tools\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"nextCursor\": {"]
#[doc = "      \"description\": \"An opaque token representing the pagination position after the last returned result.\\nIf present, there may be more results available.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"tools\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Tool\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListToolsResult {
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "An opaque token representing the pagination position after the last returned result.\nIf present, there may be more results available."]
    #[serde(
        rename = "nextCursor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub next_cursor: ::std::option::Option<::std::string::String>,
    pub tools: ::std::vec::Vec<Tool>,
}
impl ::std::convert::From<&ListToolsResult> for ListToolsResult {
    fn from(value: &ListToolsResult) -> Self {
        value.clone()
    }
}
impl ListToolsResult {
    pub fn builder() -> builder::ListToolsResult {
        Default::default()
    }
}
#[doc = "The severity of a log message.\n\nThese map to syslog message severities, as specified in RFC-5424:\nhttps://datatracker.ietf.org/doc/html/rfc5424#section-6.2.1"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The severity of a log message.\\n\\nThese map to syslog message severities, as specified in RFC-5424:\\nhttps://datatracker.ietf.org/doc/html/rfc5424#section-6.2.1\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"alert\","]
#[doc = "    \"critical\","]
#[doc = "    \"debug\","]
#[doc = "    \"emergency\","]
#[doc = "    \"error\","]
#[doc = "    \"info\","]
#[doc = "    \"notice\","]
#[doc = "    \"warning\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LoggingLevel {
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "emergency")]
    Emergency,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "notice")]
    Notice,
    #[serde(rename = "warning")]
    Warning,
}
impl ::std::convert::From<&Self> for LoggingLevel {
    fn from(value: &LoggingLevel) -> Self {
        *value
    }
}
impl ::std::fmt::Display for LoggingLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Alert => write!(f, "alert"),
            Self::Critical => write!(f, "critical"),
            Self::Debug => write!(f, "debug"),
            Self::Emergency => write!(f, "emergency"),
            Self::Error => write!(f, "error"),
            Self::Info => write!(f, "info"),
            Self::Notice => write!(f, "notice"),
            Self::Warning => write!(f, "warning"),
        }
    }
}
impl ::std::str::FromStr for LoggingLevel {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "alert" => Ok(Self::Alert),
            "critical" => Ok(Self::Critical),
            "debug" => Ok(Self::Debug),
            "emergency" => Ok(Self::Emergency),
            "error" => Ok(Self::Error),
            "info" => Ok(Self::Info),
            "notice" => Ok(Self::Notice),
            "warning" => Ok(Self::Warning),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LoggingLevel {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for LoggingLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LoggingLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Notification of a log message passed from server to client. If no logging/setLevel request has been sent from the client, the server MAY decide which messages to send automatically."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Notification of a log message passed from server to client. If no logging/setLevel request has been sent from the client, the server MAY decide which messages to send automatically.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"notifications/message\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"data\","]
#[doc = "        \"level\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"data\": {"]
#[doc = "          \"description\": \"The data to be logged, such as a string message or an object. Any JSON serializable type is allowed here.\""]
#[doc = "        },"]
#[doc = "        \"level\": {"]
#[doc = "          \"description\": \"The severity of this log message.\","]
#[doc = "          \"$ref\": \"#/definitions/LoggingLevel\""]
#[doc = "        },"]
#[doc = "        \"logger\": {"]
#[doc = "          \"description\": \"An optional name of the logger issuing this message.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct LoggingMessageNotification {
    pub method: ::std::string::String,
    pub params: LoggingMessageNotificationParams,
}
impl ::std::convert::From<&LoggingMessageNotification> for LoggingMessageNotification {
    fn from(value: &LoggingMessageNotification) -> Self {
        value.clone()
    }
}
impl LoggingMessageNotification {
    pub fn builder() -> builder::LoggingMessageNotification {
        Default::default()
    }
}
#[doc = "`LoggingMessageNotificationParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\","]
#[doc = "    \"level\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"description\": \"The data to be logged, such as a string message or an object. Any JSON serializable type is allowed here.\""]
#[doc = "    },"]
#[doc = "    \"level\": {"]
#[doc = "      \"description\": \"The severity of this log message.\","]
#[doc = "      \"$ref\": \"#/definitions/LoggingLevel\""]
#[doc = "    },"]
#[doc = "    \"logger\": {"]
#[doc = "      \"description\": \"An optional name of the logger issuing this message.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct LoggingMessageNotificationParams {
    #[doc = "The data to be logged, such as a string message or an object. Any JSON serializable type is allowed here."]
    pub data: ::serde_json::Value,
    #[doc = "The severity of this log message."]
    pub level: LoggingLevel,
    #[doc = "An optional name of the logger issuing this message."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub logger: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&LoggingMessageNotificationParams> for LoggingMessageNotificationParams {
    fn from(value: &LoggingMessageNotificationParams) -> Self {
        value.clone()
    }
}
impl LoggingMessageNotificationParams {
    pub fn builder() -> builder::LoggingMessageNotificationParams {
        Default::default()
    }
}
#[doc = "Hints to use for model selection.\n\nKeys not declared here are currently left unspecified by the spec and are up\nto the client to interpret."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Hints to use for model selection.\\n\\nKeys not declared here are currently left unspecified by the spec and are up\\nto the client to interpret.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"A hint for a model name.\\n\\nThe client SHOULD treat this as a substring of a model name; for example:\\n - `claude-3-5-sonnet` should match `claude-3-5-sonnet-20241022`\\n - `sonnet` should match `claude-3-5-sonnet-20241022`, `claude-3-sonnet-20240229`, etc.\\n - `claude` should match any Claude model\\n\\nThe client MAY also map the string to a different provider's model name or a different model family, as long as it fills a similar niche; for example:\\n - `gemini-1.5-flash` could match `claude-3-haiku-20240307`\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ModelHint {
    #[doc = "A hint for a model name.\n\nThe client SHOULD treat this as a substring of a model name; for example:\n - `claude-3-5-sonnet` should match `claude-3-5-sonnet-20241022`\n - `sonnet` should match `claude-3-5-sonnet-20241022`, `claude-3-sonnet-20240229`, etc.\n - `claude` should match any Claude model\n\nThe client MAY also map the string to a different provider's model name or a different model family, as long as it fills a similar niche; for example:\n - `gemini-1.5-flash` could match `claude-3-haiku-20240307`"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ModelHint> for ModelHint {
    fn from(value: &ModelHint) -> Self {
        value.clone()
    }
}
impl ModelHint {
    pub fn builder() -> builder::ModelHint {
        Default::default()
    }
}
#[doc = "The server's preferences for model selection, requested of the client during sampling.\n\nBecause LLMs can vary along multiple dimensions, choosing the \"best\" model is\nrarely straightforward.  Different models excel in different areas—some are\nfaster but less capable, others are more capable but more expensive, and so\non. This interface allows servers to express their priorities across multiple\ndimensions to help clients make an appropriate selection for their use case.\n\nThese preferences are always advisory. The client MAY ignore them. It is also\nup to the client to decide how to interpret these preferences and how to\nbalance them against other considerations."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The server's preferences for model selection, requested of the client during sampling.\\n\\nBecause LLMs can vary along multiple dimensions, choosing the \\\"best\\\" model is\\nrarely straightforward.  Different models excel in different areas—some are\\nfaster but less capable, others are more capable but more expensive, and so\\non. This interface allows servers to express their priorities across multiple\\ndimensions to help clients make an appropriate selection for their use case.\\n\\nThese preferences are always advisory. The client MAY ignore them. It is also\\nup to the client to decide how to interpret these preferences and how to\\nbalance them against other considerations.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"costPriority\": {"]
#[doc = "      \"description\": \"How much to prioritize cost when selecting a model. A value of 0 means cost\\nis not important, while a value of 1 means cost is the most important\\nfactor.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"hints\": {"]
#[doc = "      \"description\": \"Optional hints to use for model selection.\\n\\nIf multiple hints are specified, the client MUST evaluate them in order\\n(such that the first match is taken).\\n\\nThe client SHOULD prioritize these hints over the numeric priorities, but\\nMAY still use the priorities to select from ambiguous matches.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ModelHint\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"intelligencePriority\": {"]
#[doc = "      \"description\": \"How much to prioritize intelligence and capabilities when selecting a\\nmodel. A value of 0 means intelligence is not important, while a value of 1\\nmeans intelligence is the most important factor.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"speedPriority\": {"]
#[doc = "      \"description\": \"How much to prioritize sampling speed (latency) when selecting a model. A\\nvalue of 0 means speed is not important, while a value of 1 means speed is\\nthe most important factor.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ModelPreferences {
    #[serde(
        rename = "costPriority",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cost_priority: ::std::option::Option<f64>,
    #[doc = "Optional hints to use for model selection.\n\nIf multiple hints are specified, the client MUST evaluate them in order\n(such that the first match is taken).\n\nThe client SHOULD prioritize these hints over the numeric priorities, but\nMAY still use the priorities to select from ambiguous matches."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub hints: ::std::vec::Vec<ModelHint>,
    #[serde(
        rename = "intelligencePriority",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub intelligence_priority: ::std::option::Option<f64>,
    #[serde(
        rename = "speedPriority",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub speed_priority: ::std::option::Option<f64>,
}
impl ::std::convert::From<&ModelPreferences> for ModelPreferences {
    fn from(value: &ModelPreferences) -> Self {
        value.clone()
    }
}
impl ModelPreferences {
    pub fn builder() -> builder::ModelPreferences {
        Default::default()
    }
}
#[doc = "`Notification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"_meta\": {"]
#[doc = "          \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {}"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Notification {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<NotificationParams>,
}
impl ::std::convert::From<&Notification> for Notification {
    fn from(value: &Notification) -> Self {
        value.clone()
    }
}
impl Notification {
    pub fn builder() -> builder::Notification {
        Default::default()
    }
}
#[doc = "`NotificationParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct NotificationParams {
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&NotificationParams> for NotificationParams {
    fn from(value: &NotificationParams) -> Self {
        value.clone()
    }
}
impl NotificationParams {
    pub fn builder() -> builder::NotificationParams {
        Default::default()
    }
}
#[doc = "`NumberSchema`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"maximum\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"minimum\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"number\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct NumberSchema {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub maximum: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub minimum: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: NumberSchemaType,
}
impl ::std::convert::From<&NumberSchema> for NumberSchema {
    fn from(value: &NumberSchema) -> Self {
        value.clone()
    }
}
impl NumberSchema {
    pub fn builder() -> builder::NumberSchema {
        Default::default()
    }
}
#[doc = "`NumberSchemaType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"integer\","]
#[doc = "    \"number\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum NumberSchemaType {
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "number")]
    Number,
}
impl ::std::convert::From<&Self> for NumberSchemaType {
    fn from(value: &NumberSchemaType) -> Self {
        *value
    }
}
impl ::std::fmt::Display for NumberSchemaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Integer => write!(f, "integer"),
            Self::Number => write!(f, "number"),
        }
    }
}
impl ::std::str::FromStr for NumberSchemaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "integer" => Ok(Self::Integer),
            "number" => Ok(Self::Number),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for NumberSchemaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for NumberSchemaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NumberSchemaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PaginatedRequest`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"cursor\": {"]
#[doc = "          \"description\": \"An opaque token representing the current pagination position.\\nIf provided, the server should return results starting after this cursor.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PaginatedRequest {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<PaginatedRequestParams>,
}
impl ::std::convert::From<&PaginatedRequest> for PaginatedRequest {
    fn from(value: &PaginatedRequest) -> Self {
        value.clone()
    }
}
impl PaginatedRequest {
    pub fn builder() -> builder::PaginatedRequest {
        Default::default()
    }
}
#[doc = "`PaginatedRequestParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cursor\": {"]
#[doc = "      \"description\": \"An opaque token representing the current pagination position.\\nIf provided, the server should return results starting after this cursor.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct PaginatedRequestParams {
    #[doc = "An opaque token representing the current pagination position.\nIf provided, the server should return results starting after this cursor."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cursor: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&PaginatedRequestParams> for PaginatedRequestParams {
    fn from(value: &PaginatedRequestParams) -> Self {
        value.clone()
    }
}
impl PaginatedRequestParams {
    pub fn builder() -> builder::PaginatedRequestParams {
        Default::default()
    }
}
#[doc = "`PaginatedResult`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"nextCursor\": {"]
#[doc = "      \"description\": \"An opaque token representing the pagination position after the last returned result.\\nIf present, there may be more results available.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct PaginatedResult {
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "An opaque token representing the pagination position after the last returned result.\nIf present, there may be more results available."]
    #[serde(
        rename = "nextCursor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub next_cursor: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&PaginatedResult> for PaginatedResult {
    fn from(value: &PaginatedResult) -> Self {
        value.clone()
    }
}
impl PaginatedResult {
    pub fn builder() -> builder::PaginatedResult {
        Default::default()
    }
}
#[doc = "A ping, issued by either the server or the client, to check that the other party is still alive. The receiver must promptly respond, or else may be disconnected."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A ping, issued by either the server or the client, to check that the other party is still alive. The receiver must promptly respond, or else may be disconnected.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ping\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"_meta\": {"]
#[doc = "          \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"progressToken\": {"]
#[doc = "              \"description\": \"If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.\","]
#[doc = "              \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": {}"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PingRequest {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<PingRequestParams>,
}
impl ::std::convert::From<&PingRequest> for PingRequest {
    fn from(value: &PingRequest) -> Self {
        value.clone()
    }
}
impl PingRequest {
    pub fn builder() -> builder::PingRequest {
        Default::default()
    }
}
#[doc = "`PingRequestParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"progressToken\": {"]
#[doc = "          \"description\": \"If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.\","]
#[doc = "          \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PingRequestParams {
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<PingRequestParamsMeta>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&PingRequestParams> for PingRequestParams {
    fn from(value: &PingRequestParams) -> Self {
        value.clone()
    }
}
impl PingRequestParams {
    pub fn builder() -> builder::PingRequestParams {
        Default::default()
    }
}
#[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"progressToken\": {"]
#[doc = "      \"description\": \"If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.\","]
#[doc = "      \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PingRequestParamsMeta {
    #[doc = "If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications."]
    #[serde(
        rename = "progressToken",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub progress_token: ::std::option::Option<ProgressToken>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&PingRequestParamsMeta> for PingRequestParamsMeta {
    fn from(value: &PingRequestParamsMeta) -> Self {
        value.clone()
    }
}
impl PingRequestParamsMeta {
    pub fn builder() -> builder::PingRequestParamsMeta {
        Default::default()
    }
}
#[doc = "Restricted schema definitions that only allow primitive types\nwithout nested objects or arrays."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Restricted schema definitions that only allow primitive types\\nwithout nested objects or arrays.\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/StringSchema\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/NumberSchema\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/BooleanSchema\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/EnumSchema\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct PrimitiveSchemaDefinition {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_0: ::std::option::Option<StringSchema>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_1: ::std::option::Option<NumberSchema>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_2: ::std::option::Option<BooleanSchema>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_3: ::std::option::Option<EnumSchema>,
}
impl ::std::convert::From<&PrimitiveSchemaDefinition> for PrimitiveSchemaDefinition {
    fn from(value: &PrimitiveSchemaDefinition) -> Self {
        value.clone()
    }
}
impl PrimitiveSchemaDefinition {
    pub fn builder() -> builder::PrimitiveSchemaDefinition {
        Default::default()
    }
}
#[doc = "An out-of-band notification used to inform the receiver of a progress update for a long-running request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An out-of-band notification used to inform the receiver of a progress update for a long-running request.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"notifications/progress\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"progress\","]
#[doc = "        \"progressToken\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"message\": {"]
#[doc = "          \"description\": \"An optional message describing the current progress.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"progress\": {"]
#[doc = "          \"description\": \"The progress thus far. This should increase every time progress is made, even if the total is unknown.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"progressToken\": {"]
#[doc = "          \"description\": \"The progress token which was given in the initial request, used to associate this notification with the request that is proceeding.\","]
#[doc = "          \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "        },"]
#[doc = "        \"total\": {"]
#[doc = "          \"description\": \"Total number of items to process (or total progress required), if known.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ProgressNotification {
    pub method: ::std::string::String,
    pub params: ProgressNotificationParams,
}
impl ::std::convert::From<&ProgressNotification> for ProgressNotification {
    fn from(value: &ProgressNotification) -> Self {
        value.clone()
    }
}
impl ProgressNotification {
    pub fn builder() -> builder::ProgressNotification {
        Default::default()
    }
}
#[doc = "`ProgressNotificationParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"progress\","]
#[doc = "    \"progressToken\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"An optional message describing the current progress.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"progress\": {"]
#[doc = "      \"description\": \"The progress thus far. This should increase every time progress is made, even if the total is unknown.\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"progressToken\": {"]
#[doc = "      \"description\": \"The progress token which was given in the initial request, used to associate this notification with the request that is proceeding.\","]
#[doc = "      \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "    },"]
#[doc = "    \"total\": {"]
#[doc = "      \"description\": \"Total number of items to process (or total progress required), if known.\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ProgressNotificationParams {
    #[doc = "An optional message describing the current progress."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<::std::string::String>,
    pub progress: f64,
    #[doc = "The progress token which was given in the initial request, used to associate this notification with the request that is proceeding."]
    #[serde(rename = "progressToken")]
    pub progress_token: ProgressToken,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub total: ::std::option::Option<f64>,
}
impl ::std::convert::From<&ProgressNotificationParams> for ProgressNotificationParams {
    fn from(value: &ProgressNotificationParams) -> Self {
        value.clone()
    }
}
impl ProgressNotificationParams {
    pub fn builder() -> builder::ProgressNotificationParams {
        Default::default()
    }
}
#[doc = "A progress token, used to associate progress notifications with the original request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A progress token, used to associate progress notifications with the original request.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ProgressToken {
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<&Self> for ProgressToken {
    fn from(value: &ProgressToken) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ProgressToken {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProgressToken {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ProgressToken {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProgressToken {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for ProgressToken {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<i64> for ProgressToken {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "A prompt or prompt template that the server offers."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A prompt or prompt template that the server offers.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"arguments\": {"]
#[doc = "      \"description\": \"A list of arguments to use for templating the prompt.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/PromptArgument\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"An optional description of what this prompt provides\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"description\": \"Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\\neven by those unfamiliar with domain-specific terminology.\\n\\nIf not provided, the name should be used for display (except for Tool,\\nwhere `annotations.title` should be given precedence over using `name`,\\nif present).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Prompt {
    #[doc = "A list of arguments to use for templating the prompt."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub arguments: ::std::vec::Vec<PromptArgument>,
    #[doc = "An optional description of what this prompt provides"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present)."]
    pub name: ::std::string::String,
    #[doc = "Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\neven by those unfamiliar with domain-specific terminology.\n\nIf not provided, the name should be used for display (except for Tool,\nwhere `annotations.title` should be given precedence over using `name`,\nif present)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Prompt> for Prompt {
    fn from(value: &Prompt) -> Self {
        value.clone()
    }
}
impl Prompt {
    pub fn builder() -> builder::Prompt {
        Default::default()
    }
}
#[doc = "Describes an argument that a prompt can accept."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Describes an argument that a prompt can accept.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A human-readable description of the argument.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"required\": {"]
#[doc = "      \"description\": \"Whether this argument must be provided.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"description\": \"Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\\neven by those unfamiliar with domain-specific terminology.\\n\\nIf not provided, the name should be used for display (except for Tool,\\nwhere `annotations.title` should be given precedence over using `name`,\\nif present).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PromptArgument {
    #[doc = "A human-readable description of the argument."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present)."]
    pub name: ::std::string::String,
    #[doc = "Whether this argument must be provided."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub required: ::std::option::Option<bool>,
    #[doc = "Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\neven by those unfamiliar with domain-specific terminology.\n\nIf not provided, the name should be used for display (except for Tool,\nwhere `annotations.title` should be given precedence over using `name`,\nif present)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&PromptArgument> for PromptArgument {
    fn from(value: &PromptArgument) -> Self {
        value.clone()
    }
}
impl PromptArgument {
    pub fn builder() -> builder::PromptArgument {
        Default::default()
    }
}
#[doc = "An optional notification from the server to the client, informing it that the list of prompts it offers has changed. This may be issued by servers without any previous subscription from the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An optional notification from the server to the client, informing it that the list of prompts it offers has changed. This may be issued by servers without any previous subscription from the client.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"notifications/prompts/list_changed\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"_meta\": {"]
#[doc = "          \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {}"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PromptListChangedNotification {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<PromptListChangedNotificationParams>,
}
impl ::std::convert::From<&PromptListChangedNotification> for PromptListChangedNotification {
    fn from(value: &PromptListChangedNotification) -> Self {
        value.clone()
    }
}
impl PromptListChangedNotification {
    pub fn builder() -> builder::PromptListChangedNotification {
        Default::default()
    }
}
#[doc = "`PromptListChangedNotificationParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PromptListChangedNotificationParams {
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&PromptListChangedNotificationParams>
    for PromptListChangedNotificationParams
{
    fn from(value: &PromptListChangedNotificationParams) -> Self {
        value.clone()
    }
}
impl PromptListChangedNotificationParams {
    pub fn builder() -> builder::PromptListChangedNotificationParams {
        Default::default()
    }
}
#[doc = "Describes a message returned as part of a prompt.\n\nThis is similar to `SamplingMessage`, but also supports the embedding of\nresources from the MCP server."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Describes a message returned as part of a prompt.\\n\\nThis is similar to `SamplingMessage`, but also supports the embedding of\\nresources from the MCP server.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"content\","]
#[doc = "    \"role\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"content\": {"]
#[doc = "      \"$ref\": \"#/definitions/ContentBlock\""]
#[doc = "    },"]
#[doc = "    \"role\": {"]
#[doc = "      \"$ref\": \"#/definitions/Role\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PromptMessage {
    pub content: ContentBlock,
    pub role: Role,
}
impl ::std::convert::From<&PromptMessage> for PromptMessage {
    fn from(value: &PromptMessage) -> Self {
        value.clone()
    }
}
impl PromptMessage {
    pub fn builder() -> builder::PromptMessage {
        Default::default()
    }
}
#[doc = "Identifies a prompt."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Identifies a prompt.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"description\": \"Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\\neven by those unfamiliar with domain-specific terminology.\\n\\nIf not provided, the name should be used for display (except for Tool,\\nwhere `annotations.title` should be given precedence over using `name`,\\nif present).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ref/prompt\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PromptReference {
    #[doc = "Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present)."]
    pub name: ::std::string::String,
    #[doc = "Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\neven by those unfamiliar with domain-specific terminology.\n\nIf not provided, the name should be used for display (except for Tool,\nwhere `annotations.title` should be given precedence over using `name`,\nif present)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&PromptReference> for PromptReference {
    fn from(value: &PromptReference) -> Self {
        value.clone()
    }
}
impl PromptReference {
    pub fn builder() -> builder::PromptReference {
        Default::default()
    }
}
#[doc = "Sent from the client to the server, to read a specific resource URI."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Sent from the client to the server, to read a specific resource URI.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"resources/read\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"uri\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"uri\": {"]
#[doc = "          \"description\": \"The URI of the resource to read. The URI can use any protocol; it is up to the server how to interpret it.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uri\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ReadResourceRequest {
    pub method: ::std::string::String,
    pub params: ReadResourceRequestParams,
}
impl ::std::convert::From<&ReadResourceRequest> for ReadResourceRequest {
    fn from(value: &ReadResourceRequest) -> Self {
        value.clone()
    }
}
impl ReadResourceRequest {
    pub fn builder() -> builder::ReadResourceRequest {
        Default::default()
    }
}
#[doc = "`ReadResourceRequestParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"The URI of the resource to read. The URI can use any protocol; it is up to the server how to interpret it.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ReadResourceRequestParams {
    #[doc = "The URI of the resource to read. The URI can use any protocol; it is up to the server how to interpret it."]
    pub uri: ::std::string::String,
}
impl ::std::convert::From<&ReadResourceRequestParams> for ReadResourceRequestParams {
    fn from(value: &ReadResourceRequestParams) -> Self {
        value.clone()
    }
}
impl ReadResourceRequestParams {
    pub fn builder() -> builder::ReadResourceRequestParams {
        Default::default()
    }
}
#[doc = "The server's response to a resources/read request from the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The server's response to a resources/read request from the client.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"contents\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"contents\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/TextResourceContents\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/BlobResourceContents\""]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ReadResourceResult {
    pub contents: ::std::vec::Vec<ReadResourceResultContentsItem>,
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&ReadResourceResult> for ReadResourceResult {
    fn from(value: &ReadResourceResult) -> Self {
        value.clone()
    }
}
impl ReadResourceResult {
    pub fn builder() -> builder::ReadResourceResult {
        Default::default()
    }
}
#[doc = "`ReadResourceResultContentsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TextResourceContents\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/BlobResourceContents\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ReadResourceResultContentsItem {
    TextResourceContents(TextResourceContents),
    BlobResourceContents(BlobResourceContents),
}
impl ::std::convert::From<&Self> for ReadResourceResultContentsItem {
    fn from(value: &ReadResourceResultContentsItem) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<TextResourceContents> for ReadResourceResultContentsItem {
    fn from(value: TextResourceContents) -> Self {
        Self::TextResourceContents(value)
    }
}
impl ::std::convert::From<BlobResourceContents> for ReadResourceResultContentsItem {
    fn from(value: BlobResourceContents) -> Self {
        Self::BlobResourceContents(value)
    }
}
#[doc = "`Request`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"_meta\": {"]
#[doc = "          \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"progressToken\": {"]
#[doc = "              \"description\": \"If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.\","]
#[doc = "              \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": {}"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Request {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<RequestParams>,
}
impl ::std::convert::From<&Request> for Request {
    fn from(value: &Request) -> Self {
        value.clone()
    }
}
impl Request {
    pub fn builder() -> builder::Request {
        Default::default()
    }
}
#[doc = "A uniquely identifying ID for a request in JSON-RPC."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A uniquely identifying ID for a request in JSON-RPC.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RequestId {
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<&Self> for RequestId {
    fn from(value: &RequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RequestId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for RequestId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for RequestId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<i64> for RequestId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "`RequestParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"progressToken\": {"]
#[doc = "          \"description\": \"If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.\","]
#[doc = "          \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RequestParams {
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<RequestParamsMeta>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&RequestParams> for RequestParams {
    fn from(value: &RequestParams) -> Self {
        value.clone()
    }
}
impl RequestParams {
    pub fn builder() -> builder::RequestParams {
        Default::default()
    }
}
#[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"progressToken\": {"]
#[doc = "      \"description\": \"If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.\","]
#[doc = "      \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RequestParamsMeta {
    #[doc = "If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications."]
    #[serde(
        rename = "progressToken",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub progress_token: ::std::option::Option<ProgressToken>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&RequestParamsMeta> for RequestParamsMeta {
    fn from(value: &RequestParamsMeta) -> Self {
        value.clone()
    }
}
impl RequestParamsMeta {
    pub fn builder() -> builder::RequestParamsMeta {
        Default::default()
    }
}
#[doc = "A known resource that the server is capable of reading."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A known resource that the server is capable of reading.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"annotations\": {"]
#[doc = "      \"description\": \"Optional annotations for the client.\","]
#[doc = "      \"$ref\": \"#/definitions/Annotations\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A description of what this resource represents.\\n\\nThis can be used by clients to improve the LLM's understanding of available resources. It can be thought of like a \\\"hint\\\" to the model.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"mimeType\": {"]
#[doc = "      \"description\": \"The MIME type of this resource, if known.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"size\": {"]
#[doc = "      \"description\": \"The size of the raw resource content, in bytes (i.e., before base64 encoding or any tokenization), if known.\\n\\nThis can be used by Hosts to display file sizes and estimate context window usage.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"description\": \"Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\\neven by those unfamiliar with domain-specific terminology.\\n\\nIf not provided, the name should be used for display (except for Tool,\\nwhere `annotations.title` should be given precedence over using `name`,\\nif present).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"The URI of this resource.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Resource {
    #[doc = "Optional annotations for the client."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations: ::std::option::Option<Annotations>,
    #[doc = "A description of what this resource represents.\n\nThis can be used by clients to improve the LLM's understanding of available resources. It can be thought of like a \"hint\" to the model."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The MIME type of this resource, if known."]
    #[serde(
        rename = "mimeType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[doc = "Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present)."]
    pub name: ::std::string::String,
    #[doc = "The size of the raw resource content, in bytes (i.e., before base64 encoding or any tokenization), if known.\n\nThis can be used by Hosts to display file sizes and estimate context window usage."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub size: ::std::option::Option<i64>,
    #[doc = "Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\neven by those unfamiliar with domain-specific terminology.\n\nIf not provided, the name should be used for display (except for Tool,\nwhere `annotations.title` should be given precedence over using `name`,\nif present)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[doc = "The URI of this resource."]
    pub uri: ::std::string::String,
}
impl ::std::convert::From<&Resource> for Resource {
    fn from(value: &Resource) -> Self {
        value.clone()
    }
}
impl Resource {
    pub fn builder() -> builder::Resource {
        Default::default()
    }
}
#[doc = "The contents of a specific resource or sub-resource."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The contents of a specific resource or sub-resource.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"mimeType\": {"]
#[doc = "      \"description\": \"The MIME type of this resource, if known.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"The URI of this resource.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceContents {
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The MIME type of this resource, if known."]
    #[serde(
        rename = "mimeType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[doc = "The URI of this resource."]
    pub uri: ::std::string::String,
}
impl ::std::convert::From<&ResourceContents> for ResourceContents {
    fn from(value: &ResourceContents) -> Self {
        value.clone()
    }
}
impl ResourceContents {
    pub fn builder() -> builder::ResourceContents {
        Default::default()
    }
}
#[doc = "A resource that the server is capable of reading, included in a prompt or tool call result.\n\nNote: resource links returned by tools are not guaranteed to appear in the results of `resources/list` requests."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A resource that the server is capable of reading, included in a prompt or tool call result.\\n\\nNote: resource links returned by tools are not guaranteed to appear in the results of `resources/list` requests.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"type\","]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"annotations\": {"]
#[doc = "      \"description\": \"Optional annotations for the client.\","]
#[doc = "      \"$ref\": \"#/definitions/Annotations\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A description of what this resource represents.\\n\\nThis can be used by clients to improve the LLM's understanding of available resources. It can be thought of like a \\\"hint\\\" to the model.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"mimeType\": {"]
#[doc = "      \"description\": \"The MIME type of this resource, if known.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"size\": {"]
#[doc = "      \"description\": \"The size of the raw resource content, in bytes (i.e., before base64 encoding or any tokenization), if known.\\n\\nThis can be used by Hosts to display file sizes and estimate context window usage.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"description\": \"Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\\neven by those unfamiliar with domain-specific terminology.\\n\\nIf not provided, the name should be used for display (except for Tool,\\nwhere `annotations.title` should be given precedence over using `name`,\\nif present).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"resource_link\""]
#[doc = "    },"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"The URI of this resource.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceLink {
    #[doc = "Optional annotations for the client."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations: ::std::option::Option<Annotations>,
    #[doc = "A description of what this resource represents.\n\nThis can be used by clients to improve the LLM's understanding of available resources. It can be thought of like a \"hint\" to the model."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The MIME type of this resource, if known."]
    #[serde(
        rename = "mimeType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[doc = "Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present)."]
    pub name: ::std::string::String,
    #[doc = "The size of the raw resource content, in bytes (i.e., before base64 encoding or any tokenization), if known.\n\nThis can be used by Hosts to display file sizes and estimate context window usage."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub size: ::std::option::Option<i64>,
    #[doc = "Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\neven by those unfamiliar with domain-specific terminology.\n\nIf not provided, the name should be used for display (except for Tool,\nwhere `annotations.title` should be given precedence over using `name`,\nif present)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
    #[doc = "The URI of this resource."]
    pub uri: ::std::string::String,
}
impl ::std::convert::From<&ResourceLink> for ResourceLink {
    fn from(value: &ResourceLink) -> Self {
        value.clone()
    }
}
impl ResourceLink {
    pub fn builder() -> builder::ResourceLink {
        Default::default()
    }
}
#[doc = "An optional notification from the server to the client, informing it that the list of resources it can read from has changed. This may be issued by servers without any previous subscription from the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An optional notification from the server to the client, informing it that the list of resources it can read from has changed. This may be issued by servers without any previous subscription from the client.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"notifications/resources/list_changed\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"_meta\": {"]
#[doc = "          \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {}"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceListChangedNotification {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<ResourceListChangedNotificationParams>,
}
impl ::std::convert::From<&ResourceListChangedNotification> for ResourceListChangedNotification {
    fn from(value: &ResourceListChangedNotification) -> Self {
        value.clone()
    }
}
impl ResourceListChangedNotification {
    pub fn builder() -> builder::ResourceListChangedNotification {
        Default::default()
    }
}
#[doc = "`ResourceListChangedNotificationParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceListChangedNotificationParams {
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&ResourceListChangedNotificationParams>
    for ResourceListChangedNotificationParams
{
    fn from(value: &ResourceListChangedNotificationParams) -> Self {
        value.clone()
    }
}
impl ResourceListChangedNotificationParams {
    pub fn builder() -> builder::ResourceListChangedNotificationParams {
        Default::default()
    }
}
#[doc = "A template description for resources available on the server."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A template description for resources available on the server.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"uriTemplate\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"annotations\": {"]
#[doc = "      \"description\": \"Optional annotations for the client.\","]
#[doc = "      \"$ref\": \"#/definitions/Annotations\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A description of what this template is for.\\n\\nThis can be used by clients to improve the LLM's understanding of available resources. It can be thought of like a \\\"hint\\\" to the model.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"mimeType\": {"]
#[doc = "      \"description\": \"The MIME type for all resources that match this template. This should only be included if all resources matching this template have the same type.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"description\": \"Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\\neven by those unfamiliar with domain-specific terminology.\\n\\nIf not provided, the name should be used for display (except for Tool,\\nwhere `annotations.title` should be given precedence over using `name`,\\nif present).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"uriTemplate\": {"]
#[doc = "      \"description\": \"A URI template (according to RFC 6570) that can be used to construct resource URIs.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri-template\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceTemplate {
    #[doc = "Optional annotations for the client."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations: ::std::option::Option<Annotations>,
    #[doc = "A description of what this template is for.\n\nThis can be used by clients to improve the LLM's understanding of available resources. It can be thought of like a \"hint\" to the model."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The MIME type for all resources that match this template. This should only be included if all resources matching this template have the same type."]
    #[serde(
        rename = "mimeType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[doc = "Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present)."]
    pub name: ::std::string::String,
    #[doc = "Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\neven by those unfamiliar with domain-specific terminology.\n\nIf not provided, the name should be used for display (except for Tool,\nwhere `annotations.title` should be given precedence over using `name`,\nif present)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[doc = "A URI template (according to RFC 6570) that can be used to construct resource URIs."]
    #[serde(rename = "uriTemplate")]
    pub uri_template: ::std::string::String,
}
impl ::std::convert::From<&ResourceTemplate> for ResourceTemplate {
    fn from(value: &ResourceTemplate) -> Self {
        value.clone()
    }
}
impl ResourceTemplate {
    pub fn builder() -> builder::ResourceTemplate {
        Default::default()
    }
}
#[doc = "A reference to a resource or resource template definition."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A reference to a resource or resource template definition.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"type\","]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ref/resource\""]
#[doc = "    },"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"The URI or URI template of the resource.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri-template\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceTemplateReference {
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
    #[doc = "The URI or URI template of the resource."]
    pub uri: ::std::string::String,
}
impl ::std::convert::From<&ResourceTemplateReference> for ResourceTemplateReference {
    fn from(value: &ResourceTemplateReference) -> Self {
        value.clone()
    }
}
impl ResourceTemplateReference {
    pub fn builder() -> builder::ResourceTemplateReference {
        Default::default()
    }
}
#[doc = "A notification from the server to the client, informing it that a resource has changed and may need to be read again. This should only be sent if the client previously sent a resources/subscribe request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A notification from the server to the client, informing it that a resource has changed and may need to be read again. This should only be sent if the client previously sent a resources/subscribe request.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"notifications/resources/updated\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"uri\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"uri\": {"]
#[doc = "          \"description\": \"The URI of the resource that has been updated. This might be a sub-resource of the one that the client actually subscribed to.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uri\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceUpdatedNotification {
    pub method: ::std::string::String,
    pub params: ResourceUpdatedNotificationParams,
}
impl ::std::convert::From<&ResourceUpdatedNotification> for ResourceUpdatedNotification {
    fn from(value: &ResourceUpdatedNotification) -> Self {
        value.clone()
    }
}
impl ResourceUpdatedNotification {
    pub fn builder() -> builder::ResourceUpdatedNotification {
        Default::default()
    }
}
#[doc = "`ResourceUpdatedNotificationParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"The URI of the resource that has been updated. This might be a sub-resource of the one that the client actually subscribed to.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceUpdatedNotificationParams {
    #[doc = "The URI of the resource that has been updated. This might be a sub-resource of the one that the client actually subscribed to."]
    pub uri: ::std::string::String,
}
impl ::std::convert::From<&ResourceUpdatedNotificationParams>
    for ResourceUpdatedNotificationParams
{
    fn from(value: &ResourceUpdatedNotificationParams) -> Self {
        value.clone()
    }
}
impl ResourceUpdatedNotificationParams {
    pub fn builder() -> builder::ResourceUpdatedNotificationParams {
        Default::default()
    }
}
#[doc = "`Result`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Result {
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&Result> for Result {
    fn from(value: &Result) -> Self {
        value.clone()
    }
}
impl Result {
    pub fn builder() -> builder::Result {
        Default::default()
    }
}
#[doc = "The sender or recipient of messages and data in a conversation."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The sender or recipient of messages and data in a conversation.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"assistant\","]
#[doc = "    \"user\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum Role {
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "user")]
    User,
}
impl ::std::convert::From<&Self> for Role {
    fn from(value: &Role) -> Self {
        *value
    }
}
impl ::std::fmt::Display for Role {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Assistant => write!(f, "assistant"),
            Self::User => write!(f, "user"),
        }
    }
}
impl ::std::str::FromStr for Role {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "assistant" => Ok(Self::Assistant),
            "user" => Ok(Self::User),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Role {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Role {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Role {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents a root directory or file that the server can operate on."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a root directory or file that the server can operate on.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"An optional name for the root. This can be used to provide a human-readable\\nidentifier for the root, which may be useful for display purposes or for\\nreferencing the root in other parts of the application.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"The URI identifying the root. This *must* start with file:// for now.\\nThis restriction may be relaxed in future versions of the protocol to allow\\nother URI schemes.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Root {
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "An optional name for the root. This can be used to provide a human-readable\nidentifier for the root, which may be useful for display purposes or for\nreferencing the root in other parts of the application."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The URI identifying the root. This *must* start with file:// for now.\nThis restriction may be relaxed in future versions of the protocol to allow\nother URI schemes."]
    pub uri: ::std::string::String,
}
impl ::std::convert::From<&Root> for Root {
    fn from(value: &Root) -> Self {
        value.clone()
    }
}
impl Root {
    pub fn builder() -> builder::Root {
        Default::default()
    }
}
#[doc = "A notification from the client to the server, informing it that the list of roots has changed.\nThis notification should be sent whenever the client adds, removes, or modifies any root.\nThe server should then request an updated list of roots using the ListRootsRequest."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A notification from the client to the server, informing it that the list of roots has changed.\\nThis notification should be sent whenever the client adds, removes, or modifies any root.\\nThe server should then request an updated list of roots using the ListRootsRequest.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"notifications/roots/list_changed\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"_meta\": {"]
#[doc = "          \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {}"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RootsListChangedNotification {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<RootsListChangedNotificationParams>,
}
impl ::std::convert::From<&RootsListChangedNotification> for RootsListChangedNotification {
    fn from(value: &RootsListChangedNotification) -> Self {
        value.clone()
    }
}
impl RootsListChangedNotification {
    pub fn builder() -> builder::RootsListChangedNotification {
        Default::default()
    }
}
#[doc = "`RootsListChangedNotificationParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RootsListChangedNotificationParams {
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&RootsListChangedNotificationParams>
    for RootsListChangedNotificationParams
{
    fn from(value: &RootsListChangedNotificationParams) -> Self {
        value.clone()
    }
}
impl RootsListChangedNotificationParams {
    pub fn builder() -> builder::RootsListChangedNotificationParams {
        Default::default()
    }
}
#[doc = "Describes a message issued to or received from an LLM API."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Describes a message issued to or received from an LLM API.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"content\","]
#[doc = "    \"role\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"content\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/TextContent\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ImageContent\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AudioContent\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"role\": {"]
#[doc = "      \"$ref\": \"#/definitions/Role\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SamplingMessage {
    pub content: SamplingMessageContent,
    pub role: Role,
}
impl ::std::convert::From<&SamplingMessage> for SamplingMessage {
    fn from(value: &SamplingMessage) -> Self {
        value.clone()
    }
}
impl SamplingMessage {
    pub fn builder() -> builder::SamplingMessage {
        Default::default()
    }
}
#[doc = "`SamplingMessageContent`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TextContent\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ImageContent\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/AudioContent\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SamplingMessageContent {
    TextContent(TextContent),
    ImageContent(ImageContent),
    AudioContent(AudioContent),
}
impl ::std::convert::From<&Self> for SamplingMessageContent {
    fn from(value: &SamplingMessageContent) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<TextContent> for SamplingMessageContent {
    fn from(value: TextContent) -> Self {
        Self::TextContent(value)
    }
}
impl ::std::convert::From<ImageContent> for SamplingMessageContent {
    fn from(value: ImageContent) -> Self {
        Self::ImageContent(value)
    }
}
impl ::std::convert::From<AudioContent> for SamplingMessageContent {
    fn from(value: AudioContent) -> Self {
        Self::AudioContent(value)
    }
}
#[doc = "Capabilities that a server may support. Known capabilities are defined here, in this schema, but this is not a closed set: any server can define its own, additional capabilities."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Capabilities that a server may support. Known capabilities are defined here, in this schema, but this is not a closed set: any server can define its own, additional capabilities.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"completions\": {"]
#[doc = "      \"description\": \"Present if the server supports argument autocompletion suggestions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"experimental\": {"]
#[doc = "      \"description\": \"Experimental, non-standard capabilities that the server supports.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"additionalProperties\": true"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"logging\": {"]
#[doc = "      \"description\": \"Present if the server supports sending log messages to the client.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"prompts\": {"]
#[doc = "      \"description\": \"Present if the server offers any prompt templates.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"listChanged\": {"]
#[doc = "          \"description\": \"Whether this server supports notifications for changes to the prompt list.\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"resources\": {"]
#[doc = "      \"description\": \"Present if the server offers any resources to read.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"listChanged\": {"]
#[doc = "          \"description\": \"Whether this server supports notifications for changes to the resource list.\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"subscribe\": {"]
#[doc = "          \"description\": \"Whether this server supports subscribing to resource updates.\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"tools\": {"]
#[doc = "      \"description\": \"Present if the server offers any tools to call.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"listChanged\": {"]
#[doc = "          \"description\": \"Whether this server supports notifications for changes to the tool list.\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ServerCapabilities {
    #[doc = "Present if the server supports argument autocompletion suggestions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub completions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Experimental, non-standard capabilities that the server supports."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub experimental: ::std::collections::HashMap<
        ::std::string::String,
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[doc = "Present if the server supports sending log messages to the client."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub logging: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub prompts: ::std::option::Option<ServerCapabilitiesPrompts>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub resources: ::std::option::Option<ServerCapabilitiesResources>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tools: ::std::option::Option<ServerCapabilitiesTools>,
}
impl ::std::convert::From<&ServerCapabilities> for ServerCapabilities {
    fn from(value: &ServerCapabilities) -> Self {
        value.clone()
    }
}
impl ServerCapabilities {
    pub fn builder() -> builder::ServerCapabilities {
        Default::default()
    }
}
#[doc = "Present if the server offers any prompt templates."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Present if the server offers any prompt templates.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"listChanged\": {"]
#[doc = "      \"description\": \"Whether this server supports notifications for changes to the prompt list.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ServerCapabilitiesPrompts {
    #[doc = "Whether this server supports notifications for changes to the prompt list."]
    #[serde(
        rename = "listChanged",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub list_changed: ::std::option::Option<bool>,
}
impl ::std::convert::From<&ServerCapabilitiesPrompts> for ServerCapabilitiesPrompts {
    fn from(value: &ServerCapabilitiesPrompts) -> Self {
        value.clone()
    }
}
impl ServerCapabilitiesPrompts {
    pub fn builder() -> builder::ServerCapabilitiesPrompts {
        Default::default()
    }
}
#[doc = "Present if the server offers any resources to read."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Present if the server offers any resources to read.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"listChanged\": {"]
#[doc = "      \"description\": \"Whether this server supports notifications for changes to the resource list.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"subscribe\": {"]
#[doc = "      \"description\": \"Whether this server supports subscribing to resource updates.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ServerCapabilitiesResources {
    #[doc = "Whether this server supports notifications for changes to the resource list."]
    #[serde(
        rename = "listChanged",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub list_changed: ::std::option::Option<bool>,
    #[doc = "Whether this server supports subscribing to resource updates."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subscribe: ::std::option::Option<bool>,
}
impl ::std::convert::From<&ServerCapabilitiesResources> for ServerCapabilitiesResources {
    fn from(value: &ServerCapabilitiesResources) -> Self {
        value.clone()
    }
}
impl ServerCapabilitiesResources {
    pub fn builder() -> builder::ServerCapabilitiesResources {
        Default::default()
    }
}
#[doc = "Present if the server offers any tools to call."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Present if the server offers any tools to call.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"listChanged\": {"]
#[doc = "      \"description\": \"Whether this server supports notifications for changes to the tool list.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ServerCapabilitiesTools {
    #[doc = "Whether this server supports notifications for changes to the tool list."]
    #[serde(
        rename = "listChanged",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub list_changed: ::std::option::Option<bool>,
}
impl ::std::convert::From<&ServerCapabilitiesTools> for ServerCapabilitiesTools {
    fn from(value: &ServerCapabilitiesTools) -> Self {
        value.clone()
    }
}
impl ServerCapabilitiesTools {
    pub fn builder() -> builder::ServerCapabilitiesTools {
        Default::default()
    }
}
#[doc = "`ServerNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/CancelledNotification\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ProgressNotification\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ResourceListChangedNotification\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ResourceUpdatedNotification\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/PromptListChangedNotification\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ToolListChangedNotification\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/LoggingMessageNotification\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ServerNotification {
    CancelledNotification(CancelledNotification),
    ProgressNotification(ProgressNotification),
    ResourceListChangedNotification(ResourceListChangedNotification),
    ResourceUpdatedNotification(ResourceUpdatedNotification),
    PromptListChangedNotification(PromptListChangedNotification),
    ToolListChangedNotification(ToolListChangedNotification),
    LoggingMessageNotification(LoggingMessageNotification),
}
impl ::std::convert::From<&Self> for ServerNotification {
    fn from(value: &ServerNotification) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<CancelledNotification> for ServerNotification {
    fn from(value: CancelledNotification) -> Self {
        Self::CancelledNotification(value)
    }
}
impl ::std::convert::From<ProgressNotification> for ServerNotification {
    fn from(value: ProgressNotification) -> Self {
        Self::ProgressNotification(value)
    }
}
impl ::std::convert::From<ResourceListChangedNotification> for ServerNotification {
    fn from(value: ResourceListChangedNotification) -> Self {
        Self::ResourceListChangedNotification(value)
    }
}
impl ::std::convert::From<ResourceUpdatedNotification> for ServerNotification {
    fn from(value: ResourceUpdatedNotification) -> Self {
        Self::ResourceUpdatedNotification(value)
    }
}
impl ::std::convert::From<PromptListChangedNotification> for ServerNotification {
    fn from(value: PromptListChangedNotification) -> Self {
        Self::PromptListChangedNotification(value)
    }
}
impl ::std::convert::From<ToolListChangedNotification> for ServerNotification {
    fn from(value: ToolListChangedNotification) -> Self {
        Self::ToolListChangedNotification(value)
    }
}
impl ::std::convert::From<LoggingMessageNotification> for ServerNotification {
    fn from(value: LoggingMessageNotification) -> Self {
        Self::LoggingMessageNotification(value)
    }
}
#[doc = "`ServerRequest`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/PingRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/CreateMessageRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ListRootsRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ElicitRequest\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ServerRequest {
    PingRequest(PingRequest),
    CreateMessageRequest(CreateMessageRequest),
    ListRootsRequest(ListRootsRequest),
    ElicitRequest(ElicitRequest),
}
impl ::std::convert::From<&Self> for ServerRequest {
    fn from(value: &ServerRequest) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<PingRequest> for ServerRequest {
    fn from(value: PingRequest) -> Self {
        Self::PingRequest(value)
    }
}
impl ::std::convert::From<CreateMessageRequest> for ServerRequest {
    fn from(value: CreateMessageRequest) -> Self {
        Self::CreateMessageRequest(value)
    }
}
impl ::std::convert::From<ListRootsRequest> for ServerRequest {
    fn from(value: ListRootsRequest) -> Self {
        Self::ListRootsRequest(value)
    }
}
impl ::std::convert::From<ElicitRequest> for ServerRequest {
    fn from(value: ElicitRequest) -> Self {
        Self::ElicitRequest(value)
    }
}
#[doc = "`ServerResult`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Result\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/InitializeResult\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ListResourcesResult\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ListResourceTemplatesResult\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ReadResourceResult\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ListPromptsResult\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/GetPromptResult\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ListToolsResult\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/CallToolResult\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/CompleteResult\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ServerResult {
    Result(Result),
    InitializeResult(InitializeResult),
    ListResourcesResult(ListResourcesResult),
    ListResourceTemplatesResult(ListResourceTemplatesResult),
    ReadResourceResult(ReadResourceResult),
    ListPromptsResult(ListPromptsResult),
    GetPromptResult(GetPromptResult),
    ListToolsResult(ListToolsResult),
    CallToolResult(CallToolResult),
    CompleteResult(CompleteResult),
}
impl ::std::convert::From<&Self> for ServerResult {
    fn from(value: &ServerResult) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<Result> for ServerResult {
    fn from(value: Result) -> Self {
        Self::Result(value)
    }
}
impl ::std::convert::From<InitializeResult> for ServerResult {
    fn from(value: InitializeResult) -> Self {
        Self::InitializeResult(value)
    }
}
impl ::std::convert::From<ListResourcesResult> for ServerResult {
    fn from(value: ListResourcesResult) -> Self {
        Self::ListResourcesResult(value)
    }
}
impl ::std::convert::From<ListResourceTemplatesResult> for ServerResult {
    fn from(value: ListResourceTemplatesResult) -> Self {
        Self::ListResourceTemplatesResult(value)
    }
}
impl ::std::convert::From<ReadResourceResult> for ServerResult {
    fn from(value: ReadResourceResult) -> Self {
        Self::ReadResourceResult(value)
    }
}
impl ::std::convert::From<ListPromptsResult> for ServerResult {
    fn from(value: ListPromptsResult) -> Self {
        Self::ListPromptsResult(value)
    }
}
impl ::std::convert::From<GetPromptResult> for ServerResult {
    fn from(value: GetPromptResult) -> Self {
        Self::GetPromptResult(value)
    }
}
impl ::std::convert::From<ListToolsResult> for ServerResult {
    fn from(value: ListToolsResult) -> Self {
        Self::ListToolsResult(value)
    }
}
impl ::std::convert::From<CallToolResult> for ServerResult {
    fn from(value: CallToolResult) -> Self {
        Self::CallToolResult(value)
    }
}
impl ::std::convert::From<CompleteResult> for ServerResult {
    fn from(value: CompleteResult) -> Self {
        Self::CompleteResult(value)
    }
}
#[doc = "A request from the client to the server, to enable or adjust logging."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A request from the client to the server, to enable or adjust logging.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"logging/setLevel\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"level\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"level\": {"]
#[doc = "          \"description\": \"The level of logging that the client wants to receive from the server. The server should send all logs at this level and higher (i.e., more severe) to the client as notifications/message.\","]
#[doc = "          \"$ref\": \"#/definitions/LoggingLevel\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SetLevelRequest {
    pub method: ::std::string::String,
    pub params: SetLevelRequestParams,
}
impl ::std::convert::From<&SetLevelRequest> for SetLevelRequest {
    fn from(value: &SetLevelRequest) -> Self {
        value.clone()
    }
}
impl SetLevelRequest {
    pub fn builder() -> builder::SetLevelRequest {
        Default::default()
    }
}
#[doc = "`SetLevelRequestParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"level\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"level\": {"]
#[doc = "      \"description\": \"The level of logging that the client wants to receive from the server. The server should send all logs at this level and higher (i.e., more severe) to the client as notifications/message.\","]
#[doc = "      \"$ref\": \"#/definitions/LoggingLevel\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SetLevelRequestParams {
    #[doc = "The level of logging that the client wants to receive from the server. The server should send all logs at this level and higher (i.e., more severe) to the client as notifications/message."]
    pub level: LoggingLevel,
}
impl ::std::convert::From<&SetLevelRequestParams> for SetLevelRequestParams {
    fn from(value: &SetLevelRequestParams) -> Self {
        value.clone()
    }
}
impl SetLevelRequestParams {
    pub fn builder() -> builder::SetLevelRequestParams {
        Default::default()
    }
}
#[doc = "`StringSchema`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"format\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"date\","]
#[doc = "        \"date-time\","]
#[doc = "        \"email\","]
#[doc = "        \"uri\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"maxLength\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"minLength\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StringSchema {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<StringSchemaFormat>,
    #[serde(
        rename = "maxLength",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_length: ::std::option::Option<i64>,
    #[serde(
        rename = "minLength",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub min_length: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&StringSchema> for StringSchema {
    fn from(value: &StringSchema) -> Self {
        value.clone()
    }
}
impl StringSchema {
    pub fn builder() -> builder::StringSchema {
        Default::default()
    }
}
#[doc = "`StringSchemaFormat`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"date\","]
#[doc = "    \"date-time\","]
#[doc = "    \"email\","]
#[doc = "    \"uri\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum StringSchemaFormat {
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "date-time")]
    DateTime,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "uri")]
    Uri,
}
impl ::std::convert::From<&Self> for StringSchemaFormat {
    fn from(value: &StringSchemaFormat) -> Self {
        *value
    }
}
impl ::std::fmt::Display for StringSchemaFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Date => write!(f, "date"),
            Self::DateTime => write!(f, "date-time"),
            Self::Email => write!(f, "email"),
            Self::Uri => write!(f, "uri"),
        }
    }
}
impl ::std::str::FromStr for StringSchemaFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "date" => Ok(Self::Date),
            "date-time" => Ok(Self::DateTime),
            "email" => Ok(Self::Email),
            "uri" => Ok(Self::Uri),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StringSchemaFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StringSchemaFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StringSchemaFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Sent from the client to request resources/updated notifications from the server whenever a particular resource changes."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Sent from the client to request resources/updated notifications from the server whenever a particular resource changes.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"resources/subscribe\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"uri\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"uri\": {"]
#[doc = "          \"description\": \"The URI of the resource to subscribe to. The URI can use any protocol; it is up to the server how to interpret it.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uri\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SubscribeRequest {
    pub method: ::std::string::String,
    pub params: SubscribeRequestParams,
}
impl ::std::convert::From<&SubscribeRequest> for SubscribeRequest {
    fn from(value: &SubscribeRequest) -> Self {
        value.clone()
    }
}
impl SubscribeRequest {
    pub fn builder() -> builder::SubscribeRequest {
        Default::default()
    }
}
#[doc = "`SubscribeRequestParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"The URI of the resource to subscribe to. The URI can use any protocol; it is up to the server how to interpret it.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SubscribeRequestParams {
    #[doc = "The URI of the resource to subscribe to. The URI can use any protocol; it is up to the server how to interpret it."]
    pub uri: ::std::string::String,
}
impl ::std::convert::From<&SubscribeRequestParams> for SubscribeRequestParams {
    fn from(value: &SubscribeRequestParams) -> Self {
        value.clone()
    }
}
impl SubscribeRequestParams {
    pub fn builder() -> builder::SubscribeRequestParams {
        Default::default()
    }
}
#[doc = "Text provided to or from an LLM."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Text provided to or from an LLM.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"text\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"annotations\": {"]
#[doc = "      \"description\": \"Optional annotations for the client.\","]
#[doc = "      \"$ref\": \"#/definitions/Annotations\""]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"description\": \"The text content of the message.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"text\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TextContent {
    #[doc = "Optional annotations for the client."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations: ::std::option::Option<Annotations>,
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The text content of the message."]
    pub text: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&TextContent> for TextContent {
    fn from(value: &TextContent) -> Self {
        value.clone()
    }
}
impl TextContent {
    pub fn builder() -> builder::TextContent {
        Default::default()
    }
}
#[doc = "`TextResourceContents`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"text\","]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"mimeType\": {"]
#[doc = "      \"description\": \"The MIME type of this resource, if known.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"description\": \"The text of the item. This must only be set if the item can actually be represented as text (not binary data).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"The URI of this resource.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TextResourceContents {
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The MIME type of this resource, if known."]
    #[serde(
        rename = "mimeType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[doc = "The text of the item. This must only be set if the item can actually be represented as text (not binary data)."]
    pub text: ::std::string::String,
    #[doc = "The URI of this resource."]
    pub uri: ::std::string::String,
}
impl ::std::convert::From<&TextResourceContents> for TextResourceContents {
    fn from(value: &TextResourceContents) -> Self {
        value.clone()
    }
}
impl TextResourceContents {
    pub fn builder() -> builder::TextResourceContents {
        Default::default()
    }
}
#[doc = "Definition for a tool the client can call."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Definition for a tool the client can call.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"inputSchema\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"annotations\": {"]
#[doc = "      \"description\": \"Optional additional tool information.\\n\\nDisplay name precedence order is: title, annotations.title, then name.\","]
#[doc = "      \"$ref\": \"#/definitions/ToolAnnotations\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A human-readable description of the tool.\\n\\nThis can be used by clients to improve the LLM's understanding of available tools. It can be thought of like a \\\"hint\\\" to the model.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"inputSchema\": {"]
#[doc = "      \"description\": \"A JSON Schema object defining the expected parameters for the tool.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"properties\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"additionalProperties\": true"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"required\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"object\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"outputSchema\": {"]
#[doc = "      \"description\": \"An optional JSON Schema object defining the structure of the tool's output returned in\\nthe structuredContent field of a CallToolResult.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"properties\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"additionalProperties\": true"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"required\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"object\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"description\": \"Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\\neven by those unfamiliar with domain-specific terminology.\\n\\nIf not provided, the name should be used for display (except for Tool,\\nwhere `annotations.title` should be given precedence over using `name`,\\nif present).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Tool {
    #[doc = "Optional additional tool information.\n\nDisplay name precedence order is: title, annotations.title, then name."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations: ::std::option::Option<ToolAnnotations>,
    #[doc = "A human-readable description of the tool.\n\nThis can be used by clients to improve the LLM's understanding of available tools. It can be thought of like a \"hint\" to the model."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inputSchema")]
    pub input_schema: ToolInputSchema,
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present)."]
    pub name: ::std::string::String,
    #[serde(
        rename = "outputSchema",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub output_schema: ::std::option::Option<ToolOutputSchema>,
    #[doc = "Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\neven by those unfamiliar with domain-specific terminology.\n\nIf not provided, the name should be used for display (except for Tool,\nwhere `annotations.title` should be given precedence over using `name`,\nif present)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Tool> for Tool {
    fn from(value: &Tool) -> Self {
        value.clone()
    }
}
impl Tool {
    pub fn builder() -> builder::Tool {
        Default::default()
    }
}
#[doc = "Additional properties describing a Tool to clients.\n\nNOTE: all properties in ToolAnnotations are **hints**.\nThey are not guaranteed to provide a faithful description of\ntool behavior (including descriptive properties like `title`).\n\nClients should never make tool use decisions based on ToolAnnotations\nreceived from untrusted servers."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Additional properties describing a Tool to clients.\\n\\nNOTE: all properties in ToolAnnotations are **hints**.\\nThey are not guaranteed to provide a faithful description of\\ntool behavior (including descriptive properties like `title`).\\n\\nClients should never make tool use decisions based on ToolAnnotations\\nreceived from untrusted servers.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"destructiveHint\": {"]
#[doc = "      \"description\": \"If true, the tool may perform destructive updates to its environment.\\nIf false, the tool performs only additive updates.\\n\\n(This property is meaningful only when `readOnlyHint == false`)\\n\\nDefault: true\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"idempotentHint\": {"]
#[doc = "      \"description\": \"If true, calling the tool repeatedly with the same arguments\\nwill have no additional effect on the its environment.\\n\\n(This property is meaningful only when `readOnlyHint == false`)\\n\\nDefault: false\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"openWorldHint\": {"]
#[doc = "      \"description\": \"If true, this tool may interact with an \\\"open world\\\" of external\\nentities. If false, the tool's domain of interaction is closed.\\nFor example, the world of a web search tool is open, whereas that\\nof a memory tool is not.\\n\\nDefault: true\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"readOnlyHint\": {"]
#[doc = "      \"description\": \"If true, the tool does not modify its environment.\\n\\nDefault: false\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"description\": \"A human-readable title for the tool.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ToolAnnotations {
    #[doc = "If true, the tool may perform destructive updates to its environment.\nIf false, the tool performs only additive updates.\n\n(This property is meaningful only when `readOnlyHint == false`)\n\nDefault: true"]
    #[serde(
        rename = "destructiveHint",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub destructive_hint: ::std::option::Option<bool>,
    #[doc = "If true, calling the tool repeatedly with the same arguments\nwill have no additional effect on the its environment.\n\n(This property is meaningful only when `readOnlyHint == false`)\n\nDefault: false"]
    #[serde(
        rename = "idempotentHint",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub idempotent_hint: ::std::option::Option<bool>,
    #[doc = "If true, this tool may interact with an \"open world\" of external\nentities. If false, the tool's domain of interaction is closed.\nFor example, the world of a web search tool is open, whereas that\nof a memory tool is not.\n\nDefault: true"]
    #[serde(
        rename = "openWorldHint",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub open_world_hint: ::std::option::Option<bool>,
    #[doc = "If true, the tool does not modify its environment.\n\nDefault: false"]
    #[serde(
        rename = "readOnlyHint",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub read_only_hint: ::std::option::Option<bool>,
    #[doc = "A human-readable title for the tool."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ToolAnnotations> for ToolAnnotations {
    fn from(value: &ToolAnnotations) -> Self {
        value.clone()
    }
}
impl ToolAnnotations {
    pub fn builder() -> builder::ToolAnnotations {
        Default::default()
    }
}
#[doc = "A JSON Schema object defining the expected parameters for the tool."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A JSON Schema object defining the expected parameters for the tool.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"properties\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"additionalProperties\": true"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"required\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"object\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ToolInputSchema {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub properties: ::std::collections::HashMap<
        ::std::string::String,
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub required: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&ToolInputSchema> for ToolInputSchema {
    fn from(value: &ToolInputSchema) -> Self {
        value.clone()
    }
}
impl ToolInputSchema {
    pub fn builder() -> builder::ToolInputSchema {
        Default::default()
    }
}
#[doc = "An optional notification from the server to the client, informing it that the list of tools it offers has changed. This may be issued by servers without any previous subscription from the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An optional notification from the server to the client, informing it that the list of tools it offers has changed. This may be issued by servers without any previous subscription from the client.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"notifications/tools/list_changed\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"_meta\": {"]
#[doc = "          \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {}"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ToolListChangedNotification {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<ToolListChangedNotificationParams>,
}
impl ::std::convert::From<&ToolListChangedNotification> for ToolListChangedNotification {
    fn from(value: &ToolListChangedNotification) -> Self {
        value.clone()
    }
}
impl ToolListChangedNotification {
    pub fn builder() -> builder::ToolListChangedNotification {
        Default::default()
    }
}
#[doc = "`ToolListChangedNotificationParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ToolListChangedNotificationParams {
    #[doc = "See [specification/2025-06-18/basic/index#general-fields] for notes on _meta usage."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&ToolListChangedNotificationParams>
    for ToolListChangedNotificationParams
{
    fn from(value: &ToolListChangedNotificationParams) -> Self {
        value.clone()
    }
}
impl ToolListChangedNotificationParams {
    pub fn builder() -> builder::ToolListChangedNotificationParams {
        Default::default()
    }
}
#[doc = "An optional JSON Schema object defining the structure of the tool's output returned in\nthe structuredContent field of a CallToolResult."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An optional JSON Schema object defining the structure of the tool's output returned in\\nthe structuredContent field of a CallToolResult.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"properties\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"additionalProperties\": true"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"required\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"object\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ToolOutputSchema {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub properties: ::std::collections::HashMap<
        ::std::string::String,
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub required: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&ToolOutputSchema> for ToolOutputSchema {
    fn from(value: &ToolOutputSchema) -> Self {
        value.clone()
    }
}
impl ToolOutputSchema {
    pub fn builder() -> builder::ToolOutputSchema {
        Default::default()
    }
}
#[doc = "Sent from the client to request cancellation of resources/updated notifications from the server. This should follow a previous resources/subscribe request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Sent from the client to request cancellation of resources/updated notifications from the server. This should follow a previous resources/subscribe request.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"resources/unsubscribe\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"uri\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"uri\": {"]
#[doc = "          \"description\": \"The URI of the resource to unsubscribe from.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uri\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UnsubscribeRequest {
    pub method: ::std::string::String,
    pub params: UnsubscribeRequestParams,
}
impl ::std::convert::From<&UnsubscribeRequest> for UnsubscribeRequest {
    fn from(value: &UnsubscribeRequest) -> Self {
        value.clone()
    }
}
impl UnsubscribeRequest {
    pub fn builder() -> builder::UnsubscribeRequest {
        Default::default()
    }
}
#[doc = "`UnsubscribeRequestParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"The URI of the resource to unsubscribe from.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UnsubscribeRequestParams {
    #[doc = "The URI of the resource to unsubscribe from."]
    pub uri: ::std::string::String,
}
impl ::std::convert::From<&UnsubscribeRequestParams> for UnsubscribeRequestParams {
    fn from(value: &UnsubscribeRequestParams) -> Self {
        value.clone()
    }
}
impl UnsubscribeRequestParams {
    pub fn builder() -> builder::UnsubscribeRequestParams {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Annotations {
        audience: ::std::result::Result<::std::vec::Vec<super::Role>, ::std::string::String>,
        last_modified: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        priority: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for Annotations {
        fn default() -> Self {
            Self {
                audience: Ok(Default::default()),
                last_modified: Ok(Default::default()),
                priority: Ok(Default::default()),
            }
        }
    }
    impl Annotations {
        pub fn audience<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Role>>,
            T::Error: ::std::fmt::Display,
        {
            self.audience = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for audience: {e}"));
            self
        }
        pub fn last_modified<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.last_modified = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_modified: {e}"));
            self
        }
        pub fn priority<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.priority = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for priority: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Annotations> for super::Annotations {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Annotations,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                audience: value.audience?,
                last_modified: value.last_modified?,
                priority: value.priority?,
            })
        }
    }
    impl ::std::convert::From<super::Annotations> for Annotations {
        fn from(value: super::Annotations) -> Self {
            Self {
                audience: Ok(value.audience),
                last_modified: Ok(value.last_modified),
                priority: Ok(value.priority),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AudioContent {
        annotations:
            ::std::result::Result<::std::option::Option<super::Annotations>, ::std::string::String>,
        data: ::std::result::Result<::std::string::String, ::std::string::String>,
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        mime_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AudioContent {
        fn default() -> Self {
            Self {
                annotations: Ok(Default::default()),
                data: Err("no value supplied for data".to_string()),
                meta: Ok(Default::default()),
                mime_type: Err("no value supplied for mime_type".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AudioContent {
        pub fn annotations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Annotations>>,
            T::Error: ::std::fmt::Display,
        {
            self.annotations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for annotations: {e}"));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn mime_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.mime_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mime_type: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AudioContent> for super::AudioContent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AudioContent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                annotations: value.annotations?,
                data: value.data?,
                meta: value.meta?,
                mime_type: value.mime_type?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::AudioContent> for AudioContent {
        fn from(value: super::AudioContent) -> Self {
            Self {
                annotations: Ok(value.annotations),
                data: Ok(value.data),
                meta: Ok(value.meta),
                mime_type: Ok(value.mime_type),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BaseMetadata {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for BaseMetadata {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                title: Ok(Default::default()),
            }
        }
    }
    impl BaseMetadata {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<BaseMetadata> for super::BaseMetadata {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BaseMetadata,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::BaseMetadata> for BaseMetadata {
        fn from(value: super::BaseMetadata) -> Self {
            Self {
                name: Ok(value.name),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BlobResourceContents {
        blob: ::std::result::Result<::std::string::String, ::std::string::String>,
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        mime_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for BlobResourceContents {
        fn default() -> Self {
            Self {
                blob: Err("no value supplied for blob".to_string()),
                meta: Ok(Default::default()),
                mime_type: Ok(Default::default()),
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl BlobResourceContents {
        pub fn blob<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.blob = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for blob: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn mime_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.mime_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mime_type: {e}"));
            self
        }
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<BlobResourceContents> for super::BlobResourceContents {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BlobResourceContents,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                blob: value.blob?,
                meta: value.meta?,
                mime_type: value.mime_type?,
                uri: value.uri?,
            })
        }
    }
    impl ::std::convert::From<super::BlobResourceContents> for BlobResourceContents {
        fn from(value: super::BlobResourceContents) -> Self {
            Self {
                blob: Ok(value.blob),
                meta: Ok(value.meta),
                mime_type: Ok(value.mime_type),
                uri: Ok(value.uri),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BooleanSchema {
        default: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for BooleanSchema {
        fn default() -> Self {
            Self {
                default: Ok(Default::default()),
                description: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl BooleanSchema {
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<BooleanSchema> for super::BooleanSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BooleanSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                description: value.description?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::BooleanSchema> for BooleanSchema {
        fn from(value: super::BooleanSchema) -> Self {
            Self {
                default: Ok(value.default),
                description: Ok(value.description),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallToolRequest {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::CallToolRequestParams, ::std::string::String>,
    }
    impl ::std::default::Default for CallToolRequest {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl CallToolRequest {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CallToolRequestParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CallToolRequest> for super::CallToolRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallToolRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::CallToolRequest> for CallToolRequest {
        fn from(value: super::CallToolRequest) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallToolRequestParams {
        arguments: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CallToolRequestParams {
        fn default() -> Self {
            Self {
                arguments: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl CallToolRequestParams {
        pub fn arguments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.arguments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for arguments: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CallToolRequestParams> for super::CallToolRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallToolRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                arguments: value.arguments?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::CallToolRequestParams> for CallToolRequestParams {
        fn from(value: super::CallToolRequestParams) -> Self {
            Self {
                arguments: Ok(value.arguments),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallToolResult {
        content: ::std::result::Result<::std::vec::Vec<super::ContentBlock>, ::std::string::String>,
        is_error: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        structured_content: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CallToolResult {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                is_error: Ok(Default::default()),
                meta: Ok(Default::default()),
                structured_content: Ok(Default::default()),
            }
        }
    }
    impl CallToolResult {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ContentBlock>>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn is_error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_error: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn structured_content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.structured_content = value.try_into().map_err(|e| {
                format!("error converting supplied value for structured_content: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<CallToolResult> for super::CallToolResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallToolResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                is_error: value.is_error?,
                meta: value.meta?,
                structured_content: value.structured_content?,
            })
        }
    }
    impl ::std::convert::From<super::CallToolResult> for CallToolResult {
        fn from(value: super::CallToolResult) -> Self {
            Self {
                content: Ok(value.content),
                is_error: Ok(value.is_error),
                meta: Ok(value.meta),
                structured_content: Ok(value.structured_content),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CancelledNotification {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::CancelledNotificationParams, ::std::string::String>,
    }
    impl ::std::default::Default for CancelledNotification {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl CancelledNotification {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CancelledNotificationParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CancelledNotification> for super::CancelledNotification {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CancelledNotification,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::CancelledNotification> for CancelledNotification {
        fn from(value: super::CancelledNotification) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CancelledNotificationParams {
        reason: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        request_id: ::std::result::Result<super::RequestId, ::std::string::String>,
    }
    impl ::std::default::Default for CancelledNotificationParams {
        fn default() -> Self {
            Self {
                reason: Ok(Default::default()),
                request_id: Err("no value supplied for request_id".to_string()),
            }
        }
    }
    impl CancelledNotificationParams {
        pub fn reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reason: {e}"));
            self
        }
        pub fn request_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.request_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for request_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CancelledNotificationParams> for super::CancelledNotificationParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CancelledNotificationParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                reason: value.reason?,
                request_id: value.request_id?,
            })
        }
    }
    impl ::std::convert::From<super::CancelledNotificationParams> for CancelledNotificationParams {
        fn from(value: super::CancelledNotificationParams) -> Self {
            Self {
                reason: Ok(value.reason),
                request_id: Ok(value.request_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ClientCapabilities {
        elicitation: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        experimental: ::std::result::Result<
            ::std::collections::HashMap<
                ::std::string::String,
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            ::std::string::String,
        >,
        roots: ::std::result::Result<
            ::std::option::Option<super::ClientCapabilitiesRoots>,
            ::std::string::String,
        >,
        sampling: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ClientCapabilities {
        fn default() -> Self {
            Self {
                elicitation: Ok(Default::default()),
                experimental: Ok(Default::default()),
                roots: Ok(Default::default()),
                sampling: Ok(Default::default()),
            }
        }
    }
    impl ClientCapabilities {
        pub fn elicitation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.elicitation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for elicitation: {e}"));
            self
        }
        pub fn experimental<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::collections::HashMap<
                        ::std::string::String,
                        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                    >,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.experimental = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for experimental: {e}"));
            self
        }
        pub fn roots<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ClientCapabilitiesRoots>>,
            T::Error: ::std::fmt::Display,
        {
            self.roots = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for roots: {e}"));
            self
        }
        pub fn sampling<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.sampling = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sampling: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ClientCapabilities> for super::ClientCapabilities {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ClientCapabilities,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                elicitation: value.elicitation?,
                experimental: value.experimental?,
                roots: value.roots?,
                sampling: value.sampling?,
            })
        }
    }
    impl ::std::convert::From<super::ClientCapabilities> for ClientCapabilities {
        fn from(value: super::ClientCapabilities) -> Self {
            Self {
                elicitation: Ok(value.elicitation),
                experimental: Ok(value.experimental),
                roots: Ok(value.roots),
                sampling: Ok(value.sampling),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ClientCapabilitiesRoots {
        list_changed: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for ClientCapabilitiesRoots {
        fn default() -> Self {
            Self {
                list_changed: Ok(Default::default()),
            }
        }
    }
    impl ClientCapabilitiesRoots {
        pub fn list_changed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.list_changed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list_changed: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ClientCapabilitiesRoots> for super::ClientCapabilitiesRoots {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ClientCapabilitiesRoots,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                list_changed: value.list_changed?,
            })
        }
    }
    impl ::std::convert::From<super::ClientCapabilitiesRoots> for ClientCapabilitiesRoots {
        fn from(value: super::ClientCapabilitiesRoots) -> Self {
            Self {
                list_changed: Ok(value.list_changed),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CompleteRequest {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::CompleteRequestParams, ::std::string::String>,
    }
    impl ::std::default::Default for CompleteRequest {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl CompleteRequest {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CompleteRequestParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CompleteRequest> for super::CompleteRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CompleteRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::CompleteRequest> for CompleteRequest {
        fn from(value: super::CompleteRequest) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CompleteRequestParams {
        argument:
            ::std::result::Result<super::CompleteRequestParamsArgument, ::std::string::String>,
        context: ::std::result::Result<
            ::std::option::Option<super::CompleteRequestParamsContext>,
            ::std::string::String,
        >,
        ref_: ::std::result::Result<super::CompleteRequestParamsRef, ::std::string::String>,
    }
    impl ::std::default::Default for CompleteRequestParams {
        fn default() -> Self {
            Self {
                argument: Err("no value supplied for argument".to_string()),
                context: Ok(Default::default()),
                ref_: Err("no value supplied for ref_".to_string()),
            }
        }
    }
    impl CompleteRequestParams {
        pub fn argument<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CompleteRequestParamsArgument>,
            T::Error: ::std::fmt::Display,
        {
            self.argument = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for argument: {e}"));
            self
        }
        pub fn context<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CompleteRequestParamsContext>>,
            T::Error: ::std::fmt::Display,
        {
            self.context = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for context: {e}"));
            self
        }
        pub fn ref_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CompleteRequestParamsRef>,
            T::Error: ::std::fmt::Display,
        {
            self.ref_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ref_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CompleteRequestParams> for super::CompleteRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CompleteRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                argument: value.argument?,
                context: value.context?,
                ref_: value.ref_?,
            })
        }
    }
    impl ::std::convert::From<super::CompleteRequestParams> for CompleteRequestParams {
        fn from(value: super::CompleteRequestParams) -> Self {
            Self {
                argument: Ok(value.argument),
                context: Ok(value.context),
                ref_: Ok(value.ref_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CompleteRequestParamsArgument {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        value: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CompleteRequestParamsArgument {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl CompleteRequestParamsArgument {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CompleteRequestParamsArgument>
        for super::CompleteRequestParamsArgument
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CompleteRequestParamsArgument,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::CompleteRequestParamsArgument> for CompleteRequestParamsArgument {
        fn from(value: super::CompleteRequestParamsArgument) -> Self {
            Self {
                name: Ok(value.name),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CompleteRequestParamsContext {
        arguments: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CompleteRequestParamsContext {
        fn default() -> Self {
            Self {
                arguments: Ok(Default::default()),
            }
        }
    }
    impl CompleteRequestParamsContext {
        pub fn arguments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::collections::HashMap<::std::string::String, ::std::string::String>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.arguments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for arguments: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CompleteRequestParamsContext> for super::CompleteRequestParamsContext {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CompleteRequestParamsContext,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                arguments: value.arguments?,
            })
        }
    }
    impl ::std::convert::From<super::CompleteRequestParamsContext> for CompleteRequestParamsContext {
        fn from(value: super::CompleteRequestParamsContext) -> Self {
            Self {
                arguments: Ok(value.arguments),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CompleteResult {
        completion: ::std::result::Result<super::CompleteResultCompletion, ::std::string::String>,
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CompleteResult {
        fn default() -> Self {
            Self {
                completion: Err("no value supplied for completion".to_string()),
                meta: Ok(Default::default()),
            }
        }
    }
    impl CompleteResult {
        pub fn completion<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CompleteResultCompletion>,
            T::Error: ::std::fmt::Display,
        {
            self.completion = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for completion: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CompleteResult> for super::CompleteResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CompleteResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                completion: value.completion?,
                meta: value.meta?,
            })
        }
    }
    impl ::std::convert::From<super::CompleteResult> for CompleteResult {
        fn from(value: super::CompleteResult) -> Self {
            Self {
                completion: Ok(value.completion),
                meta: Ok(value.meta),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CompleteResultCompletion {
        has_more: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        total: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        values:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for CompleteResultCompletion {
        fn default() -> Self {
            Self {
                has_more: Ok(Default::default()),
                total: Ok(Default::default()),
                values: Err("no value supplied for values".to_string()),
            }
        }
    }
    impl CompleteResultCompletion {
        pub fn has_more<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.has_more = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for has_more: {e}"));
            self
        }
        pub fn total<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.total = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for total: {e}"));
            self
        }
        pub fn values<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.values = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for values: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CompleteResultCompletion> for super::CompleteResultCompletion {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CompleteResultCompletion,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                has_more: value.has_more?,
                total: value.total?,
                values: value.values?,
            })
        }
    }
    impl ::std::convert::From<super::CompleteResultCompletion> for CompleteResultCompletion {
        fn from(value: super::CompleteResultCompletion) -> Self {
            Self {
                has_more: Ok(value.has_more),
                total: Ok(value.total),
                values: Ok(value.values),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateMessageRequest {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::CreateMessageRequestParams, ::std::string::String>,
    }
    impl ::std::default::Default for CreateMessageRequest {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl CreateMessageRequest {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateMessageRequestParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateMessageRequest> for super::CreateMessageRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateMessageRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::CreateMessageRequest> for CreateMessageRequest {
        fn from(value: super::CreateMessageRequest) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateMessageRequestParams {
        include_context: ::std::result::Result<
            ::std::option::Option<super::CreateMessageRequestParamsIncludeContext>,
            ::std::string::String,
        >,
        max_tokens: ::std::result::Result<i64, ::std::string::String>,
        messages:
            ::std::result::Result<::std::vec::Vec<super::SamplingMessage>, ::std::string::String>,
        metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        model_preferences: ::std::result::Result<
            ::std::option::Option<super::ModelPreferences>,
            ::std::string::String,
        >,
        stop_sequences:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        system_prompt: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        temperature: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateMessageRequestParams {
        fn default() -> Self {
            Self {
                include_context: Ok(Default::default()),
                max_tokens: Err("no value supplied for max_tokens".to_string()),
                messages: Err("no value supplied for messages".to_string()),
                metadata: Ok(Default::default()),
                model_preferences: Ok(Default::default()),
                stop_sequences: Ok(Default::default()),
                system_prompt: Ok(Default::default()),
                temperature: Ok(Default::default()),
            }
        }
    }
    impl CreateMessageRequestParams {
        pub fn include_context<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<super::CreateMessageRequestParamsIncludeContext>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.include_context = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for include_context: {e}"));
            self
        }
        pub fn max_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.max_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_tokens: {e}"));
            self
        }
        pub fn messages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::SamplingMessage>>,
            T::Error: ::std::fmt::Display,
        {
            self.messages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for messages: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn model_preferences<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ModelPreferences>>,
            T::Error: ::std::fmt::Display,
        {
            self.model_preferences = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model_preferences: {e}"));
            self
        }
        pub fn stop_sequences<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.stop_sequences = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stop_sequences: {e}"));
            self
        }
        pub fn system_prompt<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_prompt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for system_prompt: {e}"));
            self
        }
        pub fn temperature<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.temperature = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for temperature: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateMessageRequestParams> for super::CreateMessageRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateMessageRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                include_context: value.include_context?,
                max_tokens: value.max_tokens?,
                messages: value.messages?,
                metadata: value.metadata?,
                model_preferences: value.model_preferences?,
                stop_sequences: value.stop_sequences?,
                system_prompt: value.system_prompt?,
                temperature: value.temperature?,
            })
        }
    }
    impl ::std::convert::From<super::CreateMessageRequestParams> for CreateMessageRequestParams {
        fn from(value: super::CreateMessageRequestParams) -> Self {
            Self {
                include_context: Ok(value.include_context),
                max_tokens: Ok(value.max_tokens),
                messages: Ok(value.messages),
                metadata: Ok(value.metadata),
                model_preferences: Ok(value.model_preferences),
                stop_sequences: Ok(value.stop_sequences),
                system_prompt: Ok(value.system_prompt),
                temperature: Ok(value.temperature),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateMessageResult {
        content: ::std::result::Result<super::CreateMessageResultContent, ::std::string::String>,
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
        role: ::std::result::Result<super::Role, ::std::string::String>,
        stop_reason: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateMessageResult {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                meta: Ok(Default::default()),
                model: Err("no value supplied for model".to_string()),
                role: Err("no value supplied for role".to_string()),
                stop_reason: Ok(Default::default()),
            }
        }
    }
    impl CreateMessageResult {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateMessageResultContent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Role>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
        pub fn stop_reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.stop_reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stop_reason: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateMessageResult> for super::CreateMessageResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateMessageResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                meta: value.meta?,
                model: value.model?,
                role: value.role?,
                stop_reason: value.stop_reason?,
            })
        }
    }
    impl ::std::convert::From<super::CreateMessageResult> for CreateMessageResult {
        fn from(value: super::CreateMessageResult) -> Self {
            Self {
                content: Ok(value.content),
                meta: Ok(value.meta),
                model: Ok(value.model),
                role: Ok(value.role),
                stop_reason: Ok(value.stop_reason),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ElicitRequest {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::ElicitRequestParams, ::std::string::String>,
    }
    impl ::std::default::Default for ElicitRequest {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl ElicitRequest {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ElicitRequestParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ElicitRequest> for super::ElicitRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ElicitRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::ElicitRequest> for ElicitRequest {
        fn from(value: super::ElicitRequest) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ElicitRequestParams {
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
        requested_schema:
            ::std::result::Result<super::ElicitRequestParamsRequestedSchema, ::std::string::String>,
    }
    impl ::std::default::Default for ElicitRequestParams {
        fn default() -> Self {
            Self {
                message: Err("no value supplied for message".to_string()),
                requested_schema: Err("no value supplied for requested_schema".to_string()),
            }
        }
    }
    impl ElicitRequestParams {
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
        pub fn requested_schema<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ElicitRequestParamsRequestedSchema>,
            T::Error: ::std::fmt::Display,
        {
            self.requested_schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for requested_schema: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ElicitRequestParams> for super::ElicitRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ElicitRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message: value.message?,
                requested_schema: value.requested_schema?,
            })
        }
    }
    impl ::std::convert::From<super::ElicitRequestParams> for ElicitRequestParams {
        fn from(value: super::ElicitRequestParams) -> Self {
            Self {
                message: Ok(value.message),
                requested_schema: Ok(value.requested_schema),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ElicitRequestParamsRequestedSchema {
        properties: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::PrimitiveSchemaDefinition>,
            ::std::string::String,
        >,
        required:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ElicitRequestParamsRequestedSchema {
        fn default() -> Self {
            Self {
                properties: Err("no value supplied for properties".to_string()),
                required: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ElicitRequestParamsRequestedSchema {
        pub fn properties<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::collections::HashMap<
                        ::std::string::String,
                        super::PrimitiveSchemaDefinition,
                    >,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.properties = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for properties: {e}"));
            self
        }
        pub fn required<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.required = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for required: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ElicitRequestParamsRequestedSchema>
        for super::ElicitRequestParamsRequestedSchema
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ElicitRequestParamsRequestedSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                properties: value.properties?,
                required: value.required?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ElicitRequestParamsRequestedSchema>
        for ElicitRequestParamsRequestedSchema
    {
        fn from(value: super::ElicitRequestParamsRequestedSchema) -> Self {
            Self {
                properties: Ok(value.properties),
                required: Ok(value.required),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ElicitResult {
        action: ::std::result::Result<super::ElicitResultAction, ::std::string::String>,
        content: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::ElicitResultContentValue>,
            ::std::string::String,
        >,
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ElicitResult {
        fn default() -> Self {
            Self {
                action: Err("no value supplied for action".to_string()),
                content: Ok(Default::default()),
                meta: Ok(Default::default()),
            }
        }
    }
    impl ElicitResult {
        pub fn action<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ElicitResultAction>,
            T::Error: ::std::fmt::Display,
        {
            self.action = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for action: {e}"));
            self
        }
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::collections::HashMap<
                        ::std::string::String,
                        super::ElicitResultContentValue,
                    >,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ElicitResult> for super::ElicitResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ElicitResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                action: value.action?,
                content: value.content?,
                meta: value.meta?,
            })
        }
    }
    impl ::std::convert::From<super::ElicitResult> for ElicitResult {
        fn from(value: super::ElicitResult) -> Self {
            Self {
                action: Ok(value.action),
                content: Ok(value.content),
                meta: Ok(value.meta),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EmbeddedResource {
        annotations:
            ::std::result::Result<::std::option::Option<super::Annotations>, ::std::string::String>,
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        resource: ::std::result::Result<super::EmbeddedResourceResource, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for EmbeddedResource {
        fn default() -> Self {
            Self {
                annotations: Ok(Default::default()),
                meta: Ok(Default::default()),
                resource: Err("no value supplied for resource".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl EmbeddedResource {
        pub fn annotations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Annotations>>,
            T::Error: ::std::fmt::Display,
        {
            self.annotations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for annotations: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn resource<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EmbeddedResourceResource>,
            T::Error: ::std::fmt::Display,
        {
            self.resource = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resource: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EmbeddedResource> for super::EmbeddedResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EmbeddedResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                annotations: value.annotations?,
                meta: value.meta?,
                resource: value.resource?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::EmbeddedResource> for EmbeddedResource {
        fn from(value: super::EmbeddedResource) -> Self {
            Self {
                annotations: Ok(value.annotations),
                meta: Ok(value.meta),
                resource: Ok(value.resource),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EnumSchema {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        enum_: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        enum_names:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for EnumSchema {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                enum_: Err("no value supplied for enum_".to_string()),
                enum_names: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl EnumSchema {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn enum_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.enum_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enum_: {e}"));
            self
        }
        pub fn enum_names<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.enum_names = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enum_names: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EnumSchema> for super::EnumSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EnumSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                enum_: value.enum_?,
                enum_names: value.enum_names?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::EnumSchema> for EnumSchema {
        fn from(value: super::EnumSchema) -> Self {
            Self {
                description: Ok(value.description),
                enum_: Ok(value.enum_),
                enum_names: Ok(value.enum_names),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetPromptRequest {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::GetPromptRequestParams, ::std::string::String>,
    }
    impl ::std::default::Default for GetPromptRequest {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl GetPromptRequest {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetPromptRequestParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetPromptRequest> for super::GetPromptRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetPromptRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::GetPromptRequest> for GetPromptRequest {
        fn from(value: super::GetPromptRequest) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetPromptRequestParams {
        arguments: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetPromptRequestParams {
        fn default() -> Self {
            Self {
                arguments: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl GetPromptRequestParams {
        pub fn arguments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::collections::HashMap<::std::string::String, ::std::string::String>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.arguments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for arguments: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetPromptRequestParams> for super::GetPromptRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetPromptRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                arguments: value.arguments?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::GetPromptRequestParams> for GetPromptRequestParams {
        fn from(value: super::GetPromptRequestParams) -> Self {
            Self {
                arguments: Ok(value.arguments),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetPromptResult {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        messages:
            ::std::result::Result<::std::vec::Vec<super::PromptMessage>, ::std::string::String>,
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GetPromptResult {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                messages: Err("no value supplied for messages".to_string()),
                meta: Ok(Default::default()),
            }
        }
    }
    impl GetPromptResult {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn messages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::PromptMessage>>,
            T::Error: ::std::fmt::Display,
        {
            self.messages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for messages: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetPromptResult> for super::GetPromptResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetPromptResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                messages: value.messages?,
                meta: value.meta?,
            })
        }
    }
    impl ::std::convert::From<super::GetPromptResult> for GetPromptResult {
        fn from(value: super::GetPromptResult) -> Self {
            Self {
                description: Ok(value.description),
                messages: Ok(value.messages),
                meta: Ok(value.meta),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ImageContent {
        annotations:
            ::std::result::Result<::std::option::Option<super::Annotations>, ::std::string::String>,
        data: ::std::result::Result<::std::string::String, ::std::string::String>,
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        mime_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ImageContent {
        fn default() -> Self {
            Self {
                annotations: Ok(Default::default()),
                data: Err("no value supplied for data".to_string()),
                meta: Ok(Default::default()),
                mime_type: Err("no value supplied for mime_type".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ImageContent {
        pub fn annotations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Annotations>>,
            T::Error: ::std::fmt::Display,
        {
            self.annotations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for annotations: {e}"));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn mime_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.mime_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mime_type: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ImageContent> for super::ImageContent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ImageContent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                annotations: value.annotations?,
                data: value.data?,
                meta: value.meta?,
                mime_type: value.mime_type?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ImageContent> for ImageContent {
        fn from(value: super::ImageContent) -> Self {
            Self {
                annotations: Ok(value.annotations),
                data: Ok(value.data),
                meta: Ok(value.meta),
                mime_type: Ok(value.mime_type),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Implementation {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        version: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Implementation {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                title: Ok(Default::default()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl Implementation {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Implementation> for super::Implementation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Implementation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                title: value.title?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::Implementation> for Implementation {
        fn from(value: super::Implementation) -> Self {
            Self {
                name: Ok(value.name),
                title: Ok(value.title),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InitializeRequest {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::InitializeRequestParams, ::std::string::String>,
    }
    impl ::std::default::Default for InitializeRequest {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl InitializeRequest {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::InitializeRequestParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InitializeRequest> for super::InitializeRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InitializeRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::InitializeRequest> for InitializeRequest {
        fn from(value: super::InitializeRequest) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InitializeRequestParams {
        capabilities: ::std::result::Result<super::ClientCapabilities, ::std::string::String>,
        client_info: ::std::result::Result<super::Implementation, ::std::string::String>,
        protocol_version: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for InitializeRequestParams {
        fn default() -> Self {
            Self {
                capabilities: Err("no value supplied for capabilities".to_string()),
                client_info: Err("no value supplied for client_info".to_string()),
                protocol_version: Err("no value supplied for protocol_version".to_string()),
            }
        }
    }
    impl InitializeRequestParams {
        pub fn capabilities<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ClientCapabilities>,
            T::Error: ::std::fmt::Display,
        {
            self.capabilities = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for capabilities: {e}"));
            self
        }
        pub fn client_info<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Implementation>,
            T::Error: ::std::fmt::Display,
        {
            self.client_info = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_info: {e}"));
            self
        }
        pub fn protocol_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.protocol_version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for protocol_version: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InitializeRequestParams> for super::InitializeRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InitializeRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                capabilities: value.capabilities?,
                client_info: value.client_info?,
                protocol_version: value.protocol_version?,
            })
        }
    }
    impl ::std::convert::From<super::InitializeRequestParams> for InitializeRequestParams {
        fn from(value: super::InitializeRequestParams) -> Self {
            Self {
                capabilities: Ok(value.capabilities),
                client_info: Ok(value.client_info),
                protocol_version: Ok(value.protocol_version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InitializeResult {
        capabilities: ::std::result::Result<super::ServerCapabilities, ::std::string::String>,
        instructions: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        protocol_version: ::std::result::Result<::std::string::String, ::std::string::String>,
        server_info: ::std::result::Result<super::Implementation, ::std::string::String>,
    }
    impl ::std::default::Default for InitializeResult {
        fn default() -> Self {
            Self {
                capabilities: Err("no value supplied for capabilities".to_string()),
                instructions: Ok(Default::default()),
                meta: Ok(Default::default()),
                protocol_version: Err("no value supplied for protocol_version".to_string()),
                server_info: Err("no value supplied for server_info".to_string()),
            }
        }
    }
    impl InitializeResult {
        pub fn capabilities<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ServerCapabilities>,
            T::Error: ::std::fmt::Display,
        {
            self.capabilities = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for capabilities: {e}"));
            self
        }
        pub fn instructions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.instructions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instructions: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn protocol_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.protocol_version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for protocol_version: {e}"));
            self
        }
        pub fn server_info<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Implementation>,
            T::Error: ::std::fmt::Display,
        {
            self.server_info = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for server_info: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InitializeResult> for super::InitializeResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InitializeResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                capabilities: value.capabilities?,
                instructions: value.instructions?,
                meta: value.meta?,
                protocol_version: value.protocol_version?,
                server_info: value.server_info?,
            })
        }
    }
    impl ::std::convert::From<super::InitializeResult> for InitializeResult {
        fn from(value: super::InitializeResult) -> Self {
            Self {
                capabilities: Ok(value.capabilities),
                instructions: Ok(value.instructions),
                meta: Ok(value.meta),
                protocol_version: Ok(value.protocol_version),
                server_info: Ok(value.server_info),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InitializedNotification {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            ::std::option::Option<super::InitializedNotificationParams>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for InitializedNotification {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Ok(Default::default()),
            }
        }
    }
    impl InitializedNotification {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::InitializedNotificationParams>>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InitializedNotification> for super::InitializedNotification {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InitializedNotification,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::InitializedNotification> for InitializedNotification {
        fn from(value: super::InitializedNotification) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InitializedNotificationParams {
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        extra: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for InitializedNotificationParams {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                extra: Err("no value supplied for extra".to_string()),
            }
        }
    }
    impl InitializedNotificationParams {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn extra<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.extra = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InitializedNotificationParams>
        for super::InitializedNotificationParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InitializedNotificationParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                extra: value.extra?,
            })
        }
    }
    impl ::std::convert::From<super::InitializedNotificationParams> for InitializedNotificationParams {
        fn from(value: super::InitializedNotificationParams) -> Self {
            Self {
                meta: Ok(value.meta),
                extra: Ok(value.extra),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JsonrpcError {
        error: ::std::result::Result<super::JsonrpcErrorError, ::std::string::String>,
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for JsonrpcError {
        fn default() -> Self {
            Self {
                error: Err("no value supplied for error".to_string()),
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
            }
        }
    }
    impl JsonrpcError {
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::JsonrpcErrorError>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<JsonrpcError> for super::JsonrpcError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JsonrpcError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error: value.error?,
                id: value.id?,
                jsonrpc: value.jsonrpc?,
            })
        }
    }
    impl ::std::convert::From<super::JsonrpcError> for JsonrpcError {
        fn from(value: super::JsonrpcError) -> Self {
            Self {
                error: Ok(value.error),
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JsonrpcErrorError {
        code: ::std::result::Result<i64, ::std::string::String>,
        data: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for JsonrpcErrorError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                data: Ok(Default::default()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl JsonrpcErrorError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<JsonrpcErrorError> for super::JsonrpcErrorError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JsonrpcErrorError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                data: value.data?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::JsonrpcErrorError> for JsonrpcErrorError {
        fn from(value: super::JsonrpcErrorError) -> Self {
            Self {
                code: Ok(value.code),
                data: Ok(value.data),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JsonrpcNotification {
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            ::std::option::Option<super::JsonrpcNotificationParams>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for JsonrpcNotification {
        fn default() -> Self {
            Self {
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Ok(Default::default()),
            }
        }
    }
    impl JsonrpcNotification {
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::JsonrpcNotificationParams>>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<JsonrpcNotification> for super::JsonrpcNotification {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JsonrpcNotification,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::JsonrpcNotification> for JsonrpcNotification {
        fn from(value: super::JsonrpcNotification) -> Self {
            Self {
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JsonrpcNotificationParams {
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        extra: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for JsonrpcNotificationParams {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                extra: Err("no value supplied for extra".to_string()),
            }
        }
    }
    impl JsonrpcNotificationParams {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn extra<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.extra = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<JsonrpcNotificationParams> for super::JsonrpcNotificationParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JsonrpcNotificationParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                extra: value.extra?,
            })
        }
    }
    impl ::std::convert::From<super::JsonrpcNotificationParams> for JsonrpcNotificationParams {
        fn from(value: super::JsonrpcNotificationParams) -> Self {
            Self {
                meta: Ok(value.meta),
                extra: Ok(value.extra),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JsonrpcRequest {
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            ::std::option::Option<super::JsonrpcRequestParams>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for JsonrpcRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Ok(Default::default()),
            }
        }
    }
    impl JsonrpcRequest {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::JsonrpcRequestParams>>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<JsonrpcRequest> for super::JsonrpcRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JsonrpcRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::JsonrpcRequest> for JsonrpcRequest {
        fn from(value: super::JsonrpcRequest) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JsonrpcRequestParams {
        meta: ::std::result::Result<
            ::std::option::Option<super::JsonrpcRequestParamsMeta>,
            ::std::string::String,
        >,
        extra: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for JsonrpcRequestParams {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                extra: Err("no value supplied for extra".to_string()),
            }
        }
    }
    impl JsonrpcRequestParams {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::JsonrpcRequestParamsMeta>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn extra<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.extra = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<JsonrpcRequestParams> for super::JsonrpcRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JsonrpcRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                extra: value.extra?,
            })
        }
    }
    impl ::std::convert::From<super::JsonrpcRequestParams> for JsonrpcRequestParams {
        fn from(value: super::JsonrpcRequestParams) -> Self {
            Self {
                meta: Ok(value.meta),
                extra: Ok(value.extra),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JsonrpcRequestParamsMeta {
        progress_token: ::std::result::Result<
            ::std::option::Option<super::ProgressToken>,
            ::std::string::String,
        >,
        extra: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for JsonrpcRequestParamsMeta {
        fn default() -> Self {
            Self {
                progress_token: Ok(Default::default()),
                extra: Err("no value supplied for extra".to_string()),
            }
        }
    }
    impl JsonrpcRequestParamsMeta {
        pub fn progress_token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ProgressToken>>,
            T::Error: ::std::fmt::Display,
        {
            self.progress_token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for progress_token: {e}"));
            self
        }
        pub fn extra<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.extra = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<JsonrpcRequestParamsMeta> for super::JsonrpcRequestParamsMeta {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JsonrpcRequestParamsMeta,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                progress_token: value.progress_token?,
                extra: value.extra?,
            })
        }
    }
    impl ::std::convert::From<super::JsonrpcRequestParamsMeta> for JsonrpcRequestParamsMeta {
        fn from(value: super::JsonrpcRequestParamsMeta) -> Self {
            Self {
                progress_token: Ok(value.progress_token),
                extra: Ok(value.extra),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JsonrpcResponse {
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        result: ::std::result::Result<super::Result, ::std::string::String>,
    }
    impl ::std::default::Default for JsonrpcResponse {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                result: Err("no value supplied for result".to_string()),
            }
        }
    }
    impl JsonrpcResponse {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Result>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<JsonrpcResponse> for super::JsonrpcResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JsonrpcResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                result: value.result?,
            })
        }
    }
    impl ::std::convert::From<super::JsonrpcResponse> for JsonrpcResponse {
        fn from(value: super::JsonrpcResponse) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                result: Ok(value.result),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListPromptsRequest {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            ::std::option::Option<super::ListPromptsRequestParams>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListPromptsRequest {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Ok(Default::default()),
            }
        }
    }
    impl ListPromptsRequest {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ListPromptsRequestParams>>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListPromptsRequest> for super::ListPromptsRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListPromptsRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::ListPromptsRequest> for ListPromptsRequest {
        fn from(value: super::ListPromptsRequest) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListPromptsRequestParams {
        cursor: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListPromptsRequestParams {
        fn default() -> Self {
            Self {
                cursor: Ok(Default::default()),
            }
        }
    }
    impl ListPromptsRequestParams {
        pub fn cursor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.cursor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cursor: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListPromptsRequestParams> for super::ListPromptsRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListPromptsRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cursor: value.cursor?,
            })
        }
    }
    impl ::std::convert::From<super::ListPromptsRequestParams> for ListPromptsRequestParams {
        fn from(value: super::ListPromptsRequestParams) -> Self {
            Self {
                cursor: Ok(value.cursor),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListPromptsResult {
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        next_cursor: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        prompts: ::std::result::Result<::std::vec::Vec<super::Prompt>, ::std::string::String>,
    }
    impl ::std::default::Default for ListPromptsResult {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                next_cursor: Ok(Default::default()),
                prompts: Err("no value supplied for prompts".to_string()),
            }
        }
    }
    impl ListPromptsResult {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn next_cursor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.next_cursor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next_cursor: {e}"));
            self
        }
        pub fn prompts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Prompt>>,
            T::Error: ::std::fmt::Display,
        {
            self.prompts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prompts: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListPromptsResult> for super::ListPromptsResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListPromptsResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                next_cursor: value.next_cursor?,
                prompts: value.prompts?,
            })
        }
    }
    impl ::std::convert::From<super::ListPromptsResult> for ListPromptsResult {
        fn from(value: super::ListPromptsResult) -> Self {
            Self {
                meta: Ok(value.meta),
                next_cursor: Ok(value.next_cursor),
                prompts: Ok(value.prompts),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListResourceTemplatesRequest {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            ::std::option::Option<super::ListResourceTemplatesRequestParams>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListResourceTemplatesRequest {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Ok(Default::default()),
            }
        }
    }
    impl ListResourceTemplatesRequest {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<super::ListResourceTemplatesRequestParams>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListResourceTemplatesRequest> for super::ListResourceTemplatesRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListResourceTemplatesRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::ListResourceTemplatesRequest> for ListResourceTemplatesRequest {
        fn from(value: super::ListResourceTemplatesRequest) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListResourceTemplatesRequestParams {
        cursor: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListResourceTemplatesRequestParams {
        fn default() -> Self {
            Self {
                cursor: Ok(Default::default()),
            }
        }
    }
    impl ListResourceTemplatesRequestParams {
        pub fn cursor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.cursor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cursor: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListResourceTemplatesRequestParams>
        for super::ListResourceTemplatesRequestParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListResourceTemplatesRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cursor: value.cursor?,
            })
        }
    }
    impl ::std::convert::From<super::ListResourceTemplatesRequestParams>
        for ListResourceTemplatesRequestParams
    {
        fn from(value: super::ListResourceTemplatesRequestParams) -> Self {
            Self {
                cursor: Ok(value.cursor),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListResourceTemplatesResult {
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        next_cursor: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        resource_templates:
            ::std::result::Result<::std::vec::Vec<super::ResourceTemplate>, ::std::string::String>,
    }
    impl ::std::default::Default for ListResourceTemplatesResult {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                next_cursor: Ok(Default::default()),
                resource_templates: Err("no value supplied for resource_templates".to_string()),
            }
        }
    }
    impl ListResourceTemplatesResult {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn next_cursor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.next_cursor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next_cursor: {e}"));
            self
        }
        pub fn resource_templates<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ResourceTemplate>>,
            T::Error: ::std::fmt::Display,
        {
            self.resource_templates = value.try_into().map_err(|e| {
                format!("error converting supplied value for resource_templates: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<ListResourceTemplatesResult> for super::ListResourceTemplatesResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListResourceTemplatesResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                next_cursor: value.next_cursor?,
                resource_templates: value.resource_templates?,
            })
        }
    }
    impl ::std::convert::From<super::ListResourceTemplatesResult> for ListResourceTemplatesResult {
        fn from(value: super::ListResourceTemplatesResult) -> Self {
            Self {
                meta: Ok(value.meta),
                next_cursor: Ok(value.next_cursor),
                resource_templates: Ok(value.resource_templates),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListResourcesRequest {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            ::std::option::Option<super::ListResourcesRequestParams>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListResourcesRequest {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Ok(Default::default()),
            }
        }
    }
    impl ListResourcesRequest {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ListResourcesRequestParams>>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListResourcesRequest> for super::ListResourcesRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListResourcesRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::ListResourcesRequest> for ListResourcesRequest {
        fn from(value: super::ListResourcesRequest) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListResourcesRequestParams {
        cursor: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListResourcesRequestParams {
        fn default() -> Self {
            Self {
                cursor: Ok(Default::default()),
            }
        }
    }
    impl ListResourcesRequestParams {
        pub fn cursor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.cursor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cursor: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListResourcesRequestParams> for super::ListResourcesRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListResourcesRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cursor: value.cursor?,
            })
        }
    }
    impl ::std::convert::From<super::ListResourcesRequestParams> for ListResourcesRequestParams {
        fn from(value: super::ListResourcesRequestParams) -> Self {
            Self {
                cursor: Ok(value.cursor),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListResourcesResult {
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        next_cursor: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        resources: ::std::result::Result<::std::vec::Vec<super::Resource>, ::std::string::String>,
    }
    impl ::std::default::Default for ListResourcesResult {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                next_cursor: Ok(Default::default()),
                resources: Err("no value supplied for resources".to_string()),
            }
        }
    }
    impl ListResourcesResult {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn next_cursor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.next_cursor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next_cursor: {e}"));
            self
        }
        pub fn resources<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Resource>>,
            T::Error: ::std::fmt::Display,
        {
            self.resources = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resources: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListResourcesResult> for super::ListResourcesResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListResourcesResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                next_cursor: value.next_cursor?,
                resources: value.resources?,
            })
        }
    }
    impl ::std::convert::From<super::ListResourcesResult> for ListResourcesResult {
        fn from(value: super::ListResourcesResult) -> Self {
            Self {
                meta: Ok(value.meta),
                next_cursor: Ok(value.next_cursor),
                resources: Ok(value.resources),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListRootsRequest {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            ::std::option::Option<super::ListRootsRequestParams>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListRootsRequest {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Ok(Default::default()),
            }
        }
    }
    impl ListRootsRequest {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ListRootsRequestParams>>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListRootsRequest> for super::ListRootsRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListRootsRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::ListRootsRequest> for ListRootsRequest {
        fn from(value: super::ListRootsRequest) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListRootsRequestParams {
        meta: ::std::result::Result<
            ::std::option::Option<super::ListRootsRequestParamsMeta>,
            ::std::string::String,
        >,
        extra: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListRootsRequestParams {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                extra: Err("no value supplied for extra".to_string()),
            }
        }
    }
    impl ListRootsRequestParams {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ListRootsRequestParamsMeta>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn extra<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.extra = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListRootsRequestParams> for super::ListRootsRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListRootsRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                extra: value.extra?,
            })
        }
    }
    impl ::std::convert::From<super::ListRootsRequestParams> for ListRootsRequestParams {
        fn from(value: super::ListRootsRequestParams) -> Self {
            Self {
                meta: Ok(value.meta),
                extra: Ok(value.extra),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListRootsRequestParamsMeta {
        progress_token: ::std::result::Result<
            ::std::option::Option<super::ProgressToken>,
            ::std::string::String,
        >,
        extra: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListRootsRequestParamsMeta {
        fn default() -> Self {
            Self {
                progress_token: Ok(Default::default()),
                extra: Err("no value supplied for extra".to_string()),
            }
        }
    }
    impl ListRootsRequestParamsMeta {
        pub fn progress_token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ProgressToken>>,
            T::Error: ::std::fmt::Display,
        {
            self.progress_token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for progress_token: {e}"));
            self
        }
        pub fn extra<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.extra = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListRootsRequestParamsMeta> for super::ListRootsRequestParamsMeta {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListRootsRequestParamsMeta,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                progress_token: value.progress_token?,
                extra: value.extra?,
            })
        }
    }
    impl ::std::convert::From<super::ListRootsRequestParamsMeta> for ListRootsRequestParamsMeta {
        fn from(value: super::ListRootsRequestParamsMeta) -> Self {
            Self {
                progress_token: Ok(value.progress_token),
                extra: Ok(value.extra),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListRootsResult {
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        roots: ::std::result::Result<::std::vec::Vec<super::Root>, ::std::string::String>,
    }
    impl ::std::default::Default for ListRootsResult {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                roots: Err("no value supplied for roots".to_string()),
            }
        }
    }
    impl ListRootsResult {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn roots<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Root>>,
            T::Error: ::std::fmt::Display,
        {
            self.roots = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for roots: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListRootsResult> for super::ListRootsResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListRootsResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                roots: value.roots?,
            })
        }
    }
    impl ::std::convert::From<super::ListRootsResult> for ListRootsResult {
        fn from(value: super::ListRootsResult) -> Self {
            Self {
                meta: Ok(value.meta),
                roots: Ok(value.roots),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListToolsRequest {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            ::std::option::Option<super::ListToolsRequestParams>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListToolsRequest {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Ok(Default::default()),
            }
        }
    }
    impl ListToolsRequest {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ListToolsRequestParams>>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListToolsRequest> for super::ListToolsRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListToolsRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::ListToolsRequest> for ListToolsRequest {
        fn from(value: super::ListToolsRequest) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListToolsRequestParams {
        cursor: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListToolsRequestParams {
        fn default() -> Self {
            Self {
                cursor: Ok(Default::default()),
            }
        }
    }
    impl ListToolsRequestParams {
        pub fn cursor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.cursor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cursor: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListToolsRequestParams> for super::ListToolsRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListToolsRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cursor: value.cursor?,
            })
        }
    }
    impl ::std::convert::From<super::ListToolsRequestParams> for ListToolsRequestParams {
        fn from(value: super::ListToolsRequestParams) -> Self {
            Self {
                cursor: Ok(value.cursor),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListToolsResult {
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        next_cursor: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        tools: ::std::result::Result<::std::vec::Vec<super::Tool>, ::std::string::String>,
    }
    impl ::std::default::Default for ListToolsResult {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                next_cursor: Ok(Default::default()),
                tools: Err("no value supplied for tools".to_string()),
            }
        }
    }
    impl ListToolsResult {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn next_cursor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.next_cursor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next_cursor: {e}"));
            self
        }
        pub fn tools<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Tool>>,
            T::Error: ::std::fmt::Display,
        {
            self.tools = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tools: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListToolsResult> for super::ListToolsResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListToolsResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                next_cursor: value.next_cursor?,
                tools: value.tools?,
            })
        }
    }
    impl ::std::convert::From<super::ListToolsResult> for ListToolsResult {
        fn from(value: super::ListToolsResult) -> Self {
            Self {
                meta: Ok(value.meta),
                next_cursor: Ok(value.next_cursor),
                tools: Ok(value.tools),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LoggingMessageNotification {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params:
            ::std::result::Result<super::LoggingMessageNotificationParams, ::std::string::String>,
    }
    impl ::std::default::Default for LoggingMessageNotification {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl LoggingMessageNotification {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LoggingMessageNotificationParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LoggingMessageNotification> for super::LoggingMessageNotification {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LoggingMessageNotification,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::LoggingMessageNotification> for LoggingMessageNotification {
        fn from(value: super::LoggingMessageNotification) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LoggingMessageNotificationParams {
        data: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        level: ::std::result::Result<super::LoggingLevel, ::std::string::String>,
        logger: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LoggingMessageNotificationParams {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                level: Err("no value supplied for level".to_string()),
                logger: Ok(Default::default()),
            }
        }
    }
    impl LoggingMessageNotificationParams {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LoggingLevel>,
            T::Error: ::std::fmt::Display,
        {
            self.level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for level: {e}"));
            self
        }
        pub fn logger<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.logger = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logger: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LoggingMessageNotificationParams>
        for super::LoggingMessageNotificationParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LoggingMessageNotificationParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                level: value.level?,
                logger: value.logger?,
            })
        }
    }
    impl ::std::convert::From<super::LoggingMessageNotificationParams>
        for LoggingMessageNotificationParams
    {
        fn from(value: super::LoggingMessageNotificationParams) -> Self {
            Self {
                data: Ok(value.data),
                level: Ok(value.level),
                logger: Ok(value.logger),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModelHint {
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ModelHint {
        fn default() -> Self {
            Self {
                name: Ok(Default::default()),
            }
        }
    }
    impl ModelHint {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ModelHint> for super::ModelHint {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModelHint,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { name: value.name? })
        }
    }
    impl ::std::convert::From<super::ModelHint> for ModelHint {
        fn from(value: super::ModelHint) -> Self {
            Self {
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModelPreferences {
        cost_priority: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        hints: ::std::result::Result<::std::vec::Vec<super::ModelHint>, ::std::string::String>,
        intelligence_priority:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        speed_priority: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for ModelPreferences {
        fn default() -> Self {
            Self {
                cost_priority: Ok(Default::default()),
                hints: Ok(Default::default()),
                intelligence_priority: Ok(Default::default()),
                speed_priority: Ok(Default::default()),
            }
        }
    }
    impl ModelPreferences {
        pub fn cost_priority<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.cost_priority = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cost_priority: {e}"));
            self
        }
        pub fn hints<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ModelHint>>,
            T::Error: ::std::fmt::Display,
        {
            self.hints = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hints: {e}"));
            self
        }
        pub fn intelligence_priority<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.intelligence_priority = value.try_into().map_err(|e| {
                format!("error converting supplied value for intelligence_priority: {e}")
            });
            self
        }
        pub fn speed_priority<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.speed_priority = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for speed_priority: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ModelPreferences> for super::ModelPreferences {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModelPreferences,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cost_priority: value.cost_priority?,
                hints: value.hints?,
                intelligence_priority: value.intelligence_priority?,
                speed_priority: value.speed_priority?,
            })
        }
    }
    impl ::std::convert::From<super::ModelPreferences> for ModelPreferences {
        fn from(value: super::ModelPreferences) -> Self {
            Self {
                cost_priority: Ok(value.cost_priority),
                hints: Ok(value.hints),
                intelligence_priority: Ok(value.intelligence_priority),
                speed_priority: Ok(value.speed_priority),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Notification {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            ::std::option::Option<super::NotificationParams>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Notification {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Ok(Default::default()),
            }
        }
    }
    impl Notification {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NotificationParams>>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Notification> for super::Notification {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Notification,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::Notification> for Notification {
        fn from(value: super::Notification) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NotificationParams {
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        extra: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for NotificationParams {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                extra: Err("no value supplied for extra".to_string()),
            }
        }
    }
    impl NotificationParams {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn extra<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.extra = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<NotificationParams> for super::NotificationParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NotificationParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                extra: value.extra?,
            })
        }
    }
    impl ::std::convert::From<super::NotificationParams> for NotificationParams {
        fn from(value: super::NotificationParams) -> Self {
            Self {
                meta: Ok(value.meta),
                extra: Ok(value.extra),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NumberSchema {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        maximum: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        minimum: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::NumberSchemaType, ::std::string::String>,
    }
    impl ::std::default::Default for NumberSchema {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                maximum: Ok(Default::default()),
                minimum: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl NumberSchema {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn maximum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.maximum = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for maximum: {e}"));
            self
        }
        pub fn minimum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.minimum = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for minimum: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NumberSchemaType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<NumberSchema> for super::NumberSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NumberSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                maximum: value.maximum?,
                minimum: value.minimum?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::NumberSchema> for NumberSchema {
        fn from(value: super::NumberSchema) -> Self {
            Self {
                description: Ok(value.description),
                maximum: Ok(value.maximum),
                minimum: Ok(value.minimum),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaginatedRequest {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            ::std::option::Option<super::PaginatedRequestParams>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PaginatedRequest {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Ok(Default::default()),
            }
        }
    }
    impl PaginatedRequest {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PaginatedRequestParams>>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaginatedRequest> for super::PaginatedRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaginatedRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::PaginatedRequest> for PaginatedRequest {
        fn from(value: super::PaginatedRequest) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaginatedRequestParams {
        cursor: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PaginatedRequestParams {
        fn default() -> Self {
            Self {
                cursor: Ok(Default::default()),
            }
        }
    }
    impl PaginatedRequestParams {
        pub fn cursor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.cursor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cursor: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaginatedRequestParams> for super::PaginatedRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaginatedRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cursor: value.cursor?,
            })
        }
    }
    impl ::std::convert::From<super::PaginatedRequestParams> for PaginatedRequestParams {
        fn from(value: super::PaginatedRequestParams) -> Self {
            Self {
                cursor: Ok(value.cursor),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaginatedResult {
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        next_cursor: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PaginatedResult {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                next_cursor: Ok(Default::default()),
            }
        }
    }
    impl PaginatedResult {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn next_cursor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.next_cursor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next_cursor: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaginatedResult> for super::PaginatedResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaginatedResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                next_cursor: value.next_cursor?,
            })
        }
    }
    impl ::std::convert::From<super::PaginatedResult> for PaginatedResult {
        fn from(value: super::PaginatedResult) -> Self {
            Self {
                meta: Ok(value.meta),
                next_cursor: Ok(value.next_cursor),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PingRequest {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            ::std::option::Option<super::PingRequestParams>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PingRequest {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Ok(Default::default()),
            }
        }
    }
    impl PingRequest {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PingRequestParams>>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PingRequest> for super::PingRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PingRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::PingRequest> for PingRequest {
        fn from(value: super::PingRequest) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PingRequestParams {
        meta: ::std::result::Result<
            ::std::option::Option<super::PingRequestParamsMeta>,
            ::std::string::String,
        >,
        extra: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PingRequestParams {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                extra: Err("no value supplied for extra".to_string()),
            }
        }
    }
    impl PingRequestParams {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PingRequestParamsMeta>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn extra<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.extra = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PingRequestParams> for super::PingRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PingRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                extra: value.extra?,
            })
        }
    }
    impl ::std::convert::From<super::PingRequestParams> for PingRequestParams {
        fn from(value: super::PingRequestParams) -> Self {
            Self {
                meta: Ok(value.meta),
                extra: Ok(value.extra),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PingRequestParamsMeta {
        progress_token: ::std::result::Result<
            ::std::option::Option<super::ProgressToken>,
            ::std::string::String,
        >,
        extra: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PingRequestParamsMeta {
        fn default() -> Self {
            Self {
                progress_token: Ok(Default::default()),
                extra: Err("no value supplied for extra".to_string()),
            }
        }
    }
    impl PingRequestParamsMeta {
        pub fn progress_token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ProgressToken>>,
            T::Error: ::std::fmt::Display,
        {
            self.progress_token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for progress_token: {e}"));
            self
        }
        pub fn extra<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.extra = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PingRequestParamsMeta> for super::PingRequestParamsMeta {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PingRequestParamsMeta,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                progress_token: value.progress_token?,
                extra: value.extra?,
            })
        }
    }
    impl ::std::convert::From<super::PingRequestParamsMeta> for PingRequestParamsMeta {
        fn from(value: super::PingRequestParamsMeta) -> Self {
            Self {
                progress_token: Ok(value.progress_token),
                extra: Ok(value.extra),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PrimitiveSchemaDefinition {
        subtype_0: ::std::result::Result<
            ::std::option::Option<super::StringSchema>,
            ::std::string::String,
        >,
        subtype_1: ::std::result::Result<
            ::std::option::Option<super::NumberSchema>,
            ::std::string::String,
        >,
        subtype_2: ::std::result::Result<
            ::std::option::Option<super::BooleanSchema>,
            ::std::string::String,
        >,
        subtype_3:
            ::std::result::Result<::std::option::Option<super::EnumSchema>, ::std::string::String>,
    }
    impl ::std::default::Default for PrimitiveSchemaDefinition {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
                subtype_2: Ok(Default::default()),
                subtype_3: Ok(Default::default()),
            }
        }
    }
    impl PrimitiveSchemaDefinition {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StringSchema>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {e}"));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NumberSchema>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {e}"));
            self
        }
        pub fn subtype_2<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::BooleanSchema>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_2: {e}"));
            self
        }
        pub fn subtype_3<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EnumSchema>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_3 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_3: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PrimitiveSchemaDefinition> for super::PrimitiveSchemaDefinition {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PrimitiveSchemaDefinition,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
                subtype_2: value.subtype_2?,
                subtype_3: value.subtype_3?,
            })
        }
    }
    impl ::std::convert::From<super::PrimitiveSchemaDefinition> for PrimitiveSchemaDefinition {
        fn from(value: super::PrimitiveSchemaDefinition) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
                subtype_2: Ok(value.subtype_2),
                subtype_3: Ok(value.subtype_3),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProgressNotification {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::ProgressNotificationParams, ::std::string::String>,
    }
    impl ::std::default::Default for ProgressNotification {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl ProgressNotification {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProgressNotificationParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProgressNotification> for super::ProgressNotification {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProgressNotification,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::ProgressNotification> for ProgressNotification {
        fn from(value: super::ProgressNotification) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProgressNotificationParams {
        message: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        progress: ::std::result::Result<f64, ::std::string::String>,
        progress_token: ::std::result::Result<super::ProgressToken, ::std::string::String>,
        total: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for ProgressNotificationParams {
        fn default() -> Self {
            Self {
                message: Ok(Default::default()),
                progress: Err("no value supplied for progress".to_string()),
                progress_token: Err("no value supplied for progress_token".to_string()),
                total: Ok(Default::default()),
            }
        }
    }
    impl ProgressNotificationParams {
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
        pub fn progress<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.progress = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for progress: {e}"));
            self
        }
        pub fn progress_token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProgressToken>,
            T::Error: ::std::fmt::Display,
        {
            self.progress_token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for progress_token: {e}"));
            self
        }
        pub fn total<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.total = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for total: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProgressNotificationParams> for super::ProgressNotificationParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProgressNotificationParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message: value.message?,
                progress: value.progress?,
                progress_token: value.progress_token?,
                total: value.total?,
            })
        }
    }
    impl ::std::convert::From<super::ProgressNotificationParams> for ProgressNotificationParams {
        fn from(value: super::ProgressNotificationParams) -> Self {
            Self {
                message: Ok(value.message),
                progress: Ok(value.progress),
                progress_token: Ok(value.progress_token),
                total: Ok(value.total),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Prompt {
        arguments:
            ::std::result::Result<::std::vec::Vec<super::PromptArgument>, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Prompt {
        fn default() -> Self {
            Self {
                arguments: Ok(Default::default()),
                description: Ok(Default::default()),
                meta: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                title: Ok(Default::default()),
            }
        }
    }
    impl Prompt {
        pub fn arguments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::PromptArgument>>,
            T::Error: ::std::fmt::Display,
        {
            self.arguments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for arguments: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Prompt> for super::Prompt {
        type Error = super::error::ConversionError;
        fn try_from(value: Prompt) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                arguments: value.arguments?,
                description: value.description?,
                meta: value.meta?,
                name: value.name?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::Prompt> for Prompt {
        fn from(value: super::Prompt) -> Self {
            Self {
                arguments: Ok(value.arguments),
                description: Ok(value.description),
                meta: Ok(value.meta),
                name: Ok(value.name),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PromptArgument {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        required: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PromptArgument {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                required: Ok(Default::default()),
                title: Ok(Default::default()),
            }
        }
    }
    impl PromptArgument {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn required<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.required = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for required: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PromptArgument> for super::PromptArgument {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PromptArgument,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                name: value.name?,
                required: value.required?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::PromptArgument> for PromptArgument {
        fn from(value: super::PromptArgument) -> Self {
            Self {
                description: Ok(value.description),
                name: Ok(value.name),
                required: Ok(value.required),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PromptListChangedNotification {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            ::std::option::Option<super::PromptListChangedNotificationParams>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PromptListChangedNotification {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Ok(Default::default()),
            }
        }
    }
    impl PromptListChangedNotification {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<super::PromptListChangedNotificationParams>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PromptListChangedNotification>
        for super::PromptListChangedNotification
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PromptListChangedNotification,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::PromptListChangedNotification> for PromptListChangedNotification {
        fn from(value: super::PromptListChangedNotification) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PromptListChangedNotificationParams {
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        extra: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PromptListChangedNotificationParams {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                extra: Err("no value supplied for extra".to_string()),
            }
        }
    }
    impl PromptListChangedNotificationParams {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn extra<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.extra = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PromptListChangedNotificationParams>
        for super::PromptListChangedNotificationParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PromptListChangedNotificationParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                extra: value.extra?,
            })
        }
    }
    impl ::std::convert::From<super::PromptListChangedNotificationParams>
        for PromptListChangedNotificationParams
    {
        fn from(value: super::PromptListChangedNotificationParams) -> Self {
            Self {
                meta: Ok(value.meta),
                extra: Ok(value.extra),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PromptMessage {
        content: ::std::result::Result<super::ContentBlock, ::std::string::String>,
        role: ::std::result::Result<super::Role, ::std::string::String>,
    }
    impl ::std::default::Default for PromptMessage {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl PromptMessage {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ContentBlock>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Role>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PromptMessage> for super::PromptMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PromptMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::PromptMessage> for PromptMessage {
        fn from(value: super::PromptMessage) -> Self {
            Self {
                content: Ok(value.content),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PromptReference {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PromptReference {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl PromptReference {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PromptReference> for super::PromptReference {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PromptReference,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::PromptReference> for PromptReference {
        fn from(value: super::PromptReference) -> Self {
            Self {
                name: Ok(value.name),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ReadResourceRequest {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::ReadResourceRequestParams, ::std::string::String>,
    }
    impl ::std::default::Default for ReadResourceRequest {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl ReadResourceRequest {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ReadResourceRequestParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ReadResourceRequest> for super::ReadResourceRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ReadResourceRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::ReadResourceRequest> for ReadResourceRequest {
        fn from(value: super::ReadResourceRequest) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ReadResourceRequestParams {
        uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ReadResourceRequestParams {
        fn default() -> Self {
            Self {
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl ReadResourceRequestParams {
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ReadResourceRequestParams> for super::ReadResourceRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ReadResourceRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { uri: value.uri? })
        }
    }
    impl ::std::convert::From<super::ReadResourceRequestParams> for ReadResourceRequestParams {
        fn from(value: super::ReadResourceRequestParams) -> Self {
            Self { uri: Ok(value.uri) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ReadResourceResult {
        contents: ::std::result::Result<
            ::std::vec::Vec<super::ReadResourceResultContentsItem>,
            ::std::string::String,
        >,
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ReadResourceResult {
        fn default() -> Self {
            Self {
                contents: Err("no value supplied for contents".to_string()),
                meta: Ok(Default::default()),
            }
        }
    }
    impl ReadResourceResult {
        pub fn contents<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ReadResourceResultContentsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.contents = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for contents: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ReadResourceResult> for super::ReadResourceResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ReadResourceResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                contents: value.contents?,
                meta: value.meta?,
            })
        }
    }
    impl ::std::convert::From<super::ReadResourceResult> for ReadResourceResult {
        fn from(value: super::ReadResourceResult) -> Self {
            Self {
                contents: Ok(value.contents),
                meta: Ok(value.meta),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Request {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            ::std::option::Option<super::RequestParams>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Request {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Ok(Default::default()),
            }
        }
    }
    impl Request {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RequestParams>>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Request> for super::Request {
        type Error = super::error::ConversionError;
        fn try_from(value: Request) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::Request> for Request {
        fn from(value: super::Request) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RequestParams {
        meta: ::std::result::Result<
            ::std::option::Option<super::RequestParamsMeta>,
            ::std::string::String,
        >,
        extra: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RequestParams {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                extra: Err("no value supplied for extra".to_string()),
            }
        }
    }
    impl RequestParams {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RequestParamsMeta>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn extra<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.extra = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RequestParams> for super::RequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                extra: value.extra?,
            })
        }
    }
    impl ::std::convert::From<super::RequestParams> for RequestParams {
        fn from(value: super::RequestParams) -> Self {
            Self {
                meta: Ok(value.meta),
                extra: Ok(value.extra),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RequestParamsMeta {
        progress_token: ::std::result::Result<
            ::std::option::Option<super::ProgressToken>,
            ::std::string::String,
        >,
        extra: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RequestParamsMeta {
        fn default() -> Self {
            Self {
                progress_token: Ok(Default::default()),
                extra: Err("no value supplied for extra".to_string()),
            }
        }
    }
    impl RequestParamsMeta {
        pub fn progress_token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ProgressToken>>,
            T::Error: ::std::fmt::Display,
        {
            self.progress_token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for progress_token: {e}"));
            self
        }
        pub fn extra<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.extra = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RequestParamsMeta> for super::RequestParamsMeta {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RequestParamsMeta,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                progress_token: value.progress_token?,
                extra: value.extra?,
            })
        }
    }
    impl ::std::convert::From<super::RequestParamsMeta> for RequestParamsMeta {
        fn from(value: super::RequestParamsMeta) -> Self {
            Self {
                progress_token: Ok(value.progress_token),
                extra: Ok(value.extra),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Resource {
        annotations:
            ::std::result::Result<::std::option::Option<super::Annotations>, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        mime_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        size: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Resource {
        fn default() -> Self {
            Self {
                annotations: Ok(Default::default()),
                description: Ok(Default::default()),
                meta: Ok(Default::default()),
                mime_type: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                size: Ok(Default::default()),
                title: Ok(Default::default()),
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl Resource {
        pub fn annotations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Annotations>>,
            T::Error: ::std::fmt::Display,
        {
            self.annotations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for annotations: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn mime_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.mime_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mime_type: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for size: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Resource> for super::Resource {
        type Error = super::error::ConversionError;
        fn try_from(value: Resource) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                annotations: value.annotations?,
                description: value.description?,
                meta: value.meta?,
                mime_type: value.mime_type?,
                name: value.name?,
                size: value.size?,
                title: value.title?,
                uri: value.uri?,
            })
        }
    }
    impl ::std::convert::From<super::Resource> for Resource {
        fn from(value: super::Resource) -> Self {
            Self {
                annotations: Ok(value.annotations),
                description: Ok(value.description),
                meta: Ok(value.meta),
                mime_type: Ok(value.mime_type),
                name: Ok(value.name),
                size: Ok(value.size),
                title: Ok(value.title),
                uri: Ok(value.uri),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResourceContents {
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        mime_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ResourceContents {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                mime_type: Ok(Default::default()),
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl ResourceContents {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn mime_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.mime_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mime_type: {e}"));
            self
        }
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ResourceContents> for super::ResourceContents {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResourceContents,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                mime_type: value.mime_type?,
                uri: value.uri?,
            })
        }
    }
    impl ::std::convert::From<super::ResourceContents> for ResourceContents {
        fn from(value: super::ResourceContents) -> Self {
            Self {
                meta: Ok(value.meta),
                mime_type: Ok(value.mime_type),
                uri: Ok(value.uri),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResourceLink {
        annotations:
            ::std::result::Result<::std::option::Option<super::Annotations>, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        mime_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        size: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
        uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ResourceLink {
        fn default() -> Self {
            Self {
                annotations: Ok(Default::default()),
                description: Ok(Default::default()),
                meta: Ok(Default::default()),
                mime_type: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                size: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl ResourceLink {
        pub fn annotations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Annotations>>,
            T::Error: ::std::fmt::Display,
        {
            self.annotations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for annotations: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn mime_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.mime_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mime_type: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for size: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ResourceLink> for super::ResourceLink {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResourceLink,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                annotations: value.annotations?,
                description: value.description?,
                meta: value.meta?,
                mime_type: value.mime_type?,
                name: value.name?,
                size: value.size?,
                title: value.title?,
                type_: value.type_?,
                uri: value.uri?,
            })
        }
    }
    impl ::std::convert::From<super::ResourceLink> for ResourceLink {
        fn from(value: super::ResourceLink) -> Self {
            Self {
                annotations: Ok(value.annotations),
                description: Ok(value.description),
                meta: Ok(value.meta),
                mime_type: Ok(value.mime_type),
                name: Ok(value.name),
                size: Ok(value.size),
                title: Ok(value.title),
                type_: Ok(value.type_),
                uri: Ok(value.uri),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResourceListChangedNotification {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            ::std::option::Option<super::ResourceListChangedNotificationParams>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ResourceListChangedNotification {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Ok(Default::default()),
            }
        }
    }
    impl ResourceListChangedNotification {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<super::ResourceListChangedNotificationParams>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ResourceListChangedNotification>
        for super::ResourceListChangedNotification
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResourceListChangedNotification,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::ResourceListChangedNotification>
        for ResourceListChangedNotification
    {
        fn from(value: super::ResourceListChangedNotification) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResourceListChangedNotificationParams {
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        extra: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ResourceListChangedNotificationParams {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                extra: Err("no value supplied for extra".to_string()),
            }
        }
    }
    impl ResourceListChangedNotificationParams {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn extra<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.extra = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ResourceListChangedNotificationParams>
        for super::ResourceListChangedNotificationParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResourceListChangedNotificationParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                extra: value.extra?,
            })
        }
    }
    impl ::std::convert::From<super::ResourceListChangedNotificationParams>
        for ResourceListChangedNotificationParams
    {
        fn from(value: super::ResourceListChangedNotificationParams) -> Self {
            Self {
                meta: Ok(value.meta),
                extra: Ok(value.extra),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResourceTemplate {
        annotations:
            ::std::result::Result<::std::option::Option<super::Annotations>, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        mime_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        uri_template: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ResourceTemplate {
        fn default() -> Self {
            Self {
                annotations: Ok(Default::default()),
                description: Ok(Default::default()),
                meta: Ok(Default::default()),
                mime_type: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                title: Ok(Default::default()),
                uri_template: Err("no value supplied for uri_template".to_string()),
            }
        }
    }
    impl ResourceTemplate {
        pub fn annotations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Annotations>>,
            T::Error: ::std::fmt::Display,
        {
            self.annotations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for annotations: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn mime_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.mime_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mime_type: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn uri_template<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri_template = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri_template: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ResourceTemplate> for super::ResourceTemplate {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResourceTemplate,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                annotations: value.annotations?,
                description: value.description?,
                meta: value.meta?,
                mime_type: value.mime_type?,
                name: value.name?,
                title: value.title?,
                uri_template: value.uri_template?,
            })
        }
    }
    impl ::std::convert::From<super::ResourceTemplate> for ResourceTemplate {
        fn from(value: super::ResourceTemplate) -> Self {
            Self {
                annotations: Ok(value.annotations),
                description: Ok(value.description),
                meta: Ok(value.meta),
                mime_type: Ok(value.mime_type),
                name: Ok(value.name),
                title: Ok(value.title),
                uri_template: Ok(value.uri_template),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResourceTemplateReference {
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
        uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ResourceTemplateReference {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl ResourceTemplateReference {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ResourceTemplateReference> for super::ResourceTemplateReference {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResourceTemplateReference,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
                uri: value.uri?,
            })
        }
    }
    impl ::std::convert::From<super::ResourceTemplateReference> for ResourceTemplateReference {
        fn from(value: super::ResourceTemplateReference) -> Self {
            Self {
                type_: Ok(value.type_),
                uri: Ok(value.uri),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResourceUpdatedNotification {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params:
            ::std::result::Result<super::ResourceUpdatedNotificationParams, ::std::string::String>,
    }
    impl ::std::default::Default for ResourceUpdatedNotification {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl ResourceUpdatedNotification {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ResourceUpdatedNotificationParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ResourceUpdatedNotification> for super::ResourceUpdatedNotification {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResourceUpdatedNotification,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::ResourceUpdatedNotification> for ResourceUpdatedNotification {
        fn from(value: super::ResourceUpdatedNotification) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResourceUpdatedNotificationParams {
        uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ResourceUpdatedNotificationParams {
        fn default() -> Self {
            Self {
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl ResourceUpdatedNotificationParams {
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ResourceUpdatedNotificationParams>
        for super::ResourceUpdatedNotificationParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResourceUpdatedNotificationParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { uri: value.uri? })
        }
    }
    impl ::std::convert::From<super::ResourceUpdatedNotificationParams>
        for ResourceUpdatedNotificationParams
    {
        fn from(value: super::ResourceUpdatedNotificationParams) -> Self {
            Self { uri: Ok(value.uri) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Result {
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        extra: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Result {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                extra: Err("no value supplied for extra".to_string()),
            }
        }
    }
    impl Result {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn extra<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.extra = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Result> for super::Result {
        type Error = super::error::ConversionError;
        fn try_from(value: Result) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                extra: value.extra?,
            })
        }
    }
    impl ::std::convert::From<super::Result> for Result {
        fn from(value: super::Result) -> Self {
            Self {
                meta: Ok(value.meta),
                extra: Ok(value.extra),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Root {
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Root {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                name: Ok(Default::default()),
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl Root {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Root> for super::Root {
        type Error = super::error::ConversionError;
        fn try_from(value: Root) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                name: value.name?,
                uri: value.uri?,
            })
        }
    }
    impl ::std::convert::From<super::Root> for Root {
        fn from(value: super::Root) -> Self {
            Self {
                meta: Ok(value.meta),
                name: Ok(value.name),
                uri: Ok(value.uri),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RootsListChangedNotification {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            ::std::option::Option<super::RootsListChangedNotificationParams>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RootsListChangedNotification {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Ok(Default::default()),
            }
        }
    }
    impl RootsListChangedNotification {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<super::RootsListChangedNotificationParams>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RootsListChangedNotification> for super::RootsListChangedNotification {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RootsListChangedNotification,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::RootsListChangedNotification> for RootsListChangedNotification {
        fn from(value: super::RootsListChangedNotification) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RootsListChangedNotificationParams {
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        extra: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RootsListChangedNotificationParams {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                extra: Err("no value supplied for extra".to_string()),
            }
        }
    }
    impl RootsListChangedNotificationParams {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn extra<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.extra = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RootsListChangedNotificationParams>
        for super::RootsListChangedNotificationParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RootsListChangedNotificationParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                extra: value.extra?,
            })
        }
    }
    impl ::std::convert::From<super::RootsListChangedNotificationParams>
        for RootsListChangedNotificationParams
    {
        fn from(value: super::RootsListChangedNotificationParams) -> Self {
            Self {
                meta: Ok(value.meta),
                extra: Ok(value.extra),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SamplingMessage {
        content: ::std::result::Result<super::SamplingMessageContent, ::std::string::String>,
        role: ::std::result::Result<super::Role, ::std::string::String>,
    }
    impl ::std::default::Default for SamplingMessage {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl SamplingMessage {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SamplingMessageContent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Role>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SamplingMessage> for super::SamplingMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SamplingMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::SamplingMessage> for SamplingMessage {
        fn from(value: super::SamplingMessage) -> Self {
            Self {
                content: Ok(value.content),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ServerCapabilities {
        completions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        experimental: ::std::result::Result<
            ::std::collections::HashMap<
                ::std::string::String,
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            ::std::string::String,
        >,
        logging: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        prompts: ::std::result::Result<
            ::std::option::Option<super::ServerCapabilitiesPrompts>,
            ::std::string::String,
        >,
        resources: ::std::result::Result<
            ::std::option::Option<super::ServerCapabilitiesResources>,
            ::std::string::String,
        >,
        tools: ::std::result::Result<
            ::std::option::Option<super::ServerCapabilitiesTools>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ServerCapabilities {
        fn default() -> Self {
            Self {
                completions: Ok(Default::default()),
                experimental: Ok(Default::default()),
                logging: Ok(Default::default()),
                prompts: Ok(Default::default()),
                resources: Ok(Default::default()),
                tools: Ok(Default::default()),
            }
        }
    }
    impl ServerCapabilities {
        pub fn completions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.completions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for completions: {e}"));
            self
        }
        pub fn experimental<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::collections::HashMap<
                        ::std::string::String,
                        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                    >,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.experimental = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for experimental: {e}"));
            self
        }
        pub fn logging<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.logging = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logging: {e}"));
            self
        }
        pub fn prompts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ServerCapabilitiesPrompts>>,
            T::Error: ::std::fmt::Display,
        {
            self.prompts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prompts: {e}"));
            self
        }
        pub fn resources<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ServerCapabilitiesResources>>,
            T::Error: ::std::fmt::Display,
        {
            self.resources = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resources: {e}"));
            self
        }
        pub fn tools<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ServerCapabilitiesTools>>,
            T::Error: ::std::fmt::Display,
        {
            self.tools = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tools: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ServerCapabilities> for super::ServerCapabilities {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ServerCapabilities,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                completions: value.completions?,
                experimental: value.experimental?,
                logging: value.logging?,
                prompts: value.prompts?,
                resources: value.resources?,
                tools: value.tools?,
            })
        }
    }
    impl ::std::convert::From<super::ServerCapabilities> for ServerCapabilities {
        fn from(value: super::ServerCapabilities) -> Self {
            Self {
                completions: Ok(value.completions),
                experimental: Ok(value.experimental),
                logging: Ok(value.logging),
                prompts: Ok(value.prompts),
                resources: Ok(value.resources),
                tools: Ok(value.tools),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ServerCapabilitiesPrompts {
        list_changed: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for ServerCapabilitiesPrompts {
        fn default() -> Self {
            Self {
                list_changed: Ok(Default::default()),
            }
        }
    }
    impl ServerCapabilitiesPrompts {
        pub fn list_changed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.list_changed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list_changed: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ServerCapabilitiesPrompts> for super::ServerCapabilitiesPrompts {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ServerCapabilitiesPrompts,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                list_changed: value.list_changed?,
            })
        }
    }
    impl ::std::convert::From<super::ServerCapabilitiesPrompts> for ServerCapabilitiesPrompts {
        fn from(value: super::ServerCapabilitiesPrompts) -> Self {
            Self {
                list_changed: Ok(value.list_changed),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ServerCapabilitiesResources {
        list_changed: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        subscribe: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for ServerCapabilitiesResources {
        fn default() -> Self {
            Self {
                list_changed: Ok(Default::default()),
                subscribe: Ok(Default::default()),
            }
        }
    }
    impl ServerCapabilitiesResources {
        pub fn list_changed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.list_changed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list_changed: {e}"));
            self
        }
        pub fn subscribe<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.subscribe = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subscribe: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ServerCapabilitiesResources> for super::ServerCapabilitiesResources {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ServerCapabilitiesResources,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                list_changed: value.list_changed?,
                subscribe: value.subscribe?,
            })
        }
    }
    impl ::std::convert::From<super::ServerCapabilitiesResources> for ServerCapabilitiesResources {
        fn from(value: super::ServerCapabilitiesResources) -> Self {
            Self {
                list_changed: Ok(value.list_changed),
                subscribe: Ok(value.subscribe),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ServerCapabilitiesTools {
        list_changed: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for ServerCapabilitiesTools {
        fn default() -> Self {
            Self {
                list_changed: Ok(Default::default()),
            }
        }
    }
    impl ServerCapabilitiesTools {
        pub fn list_changed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.list_changed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list_changed: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ServerCapabilitiesTools> for super::ServerCapabilitiesTools {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ServerCapabilitiesTools,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                list_changed: value.list_changed?,
            })
        }
    }
    impl ::std::convert::From<super::ServerCapabilitiesTools> for ServerCapabilitiesTools {
        fn from(value: super::ServerCapabilitiesTools) -> Self {
            Self {
                list_changed: Ok(value.list_changed),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SetLevelRequest {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::SetLevelRequestParams, ::std::string::String>,
    }
    impl ::std::default::Default for SetLevelRequest {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl SetLevelRequest {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SetLevelRequestParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SetLevelRequest> for super::SetLevelRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SetLevelRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::SetLevelRequest> for SetLevelRequest {
        fn from(value: super::SetLevelRequest) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SetLevelRequestParams {
        level: ::std::result::Result<super::LoggingLevel, ::std::string::String>,
    }
    impl ::std::default::Default for SetLevelRequestParams {
        fn default() -> Self {
            Self {
                level: Err("no value supplied for level".to_string()),
            }
        }
    }
    impl SetLevelRequestParams {
        pub fn level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LoggingLevel>,
            T::Error: ::std::fmt::Display,
        {
            self.level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for level: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SetLevelRequestParams> for super::SetLevelRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SetLevelRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                level: value.level?,
            })
        }
    }
    impl ::std::convert::From<super::SetLevelRequestParams> for SetLevelRequestParams {
        fn from(value: super::SetLevelRequestParams) -> Self {
            Self {
                level: Ok(value.level),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StringSchema {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        format: ::std::result::Result<
            ::std::option::Option<super::StringSchemaFormat>,
            ::std::string::String,
        >,
        max_length: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        min_length: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for StringSchema {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                format: Ok(Default::default()),
                max_length: Ok(Default::default()),
                min_length: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl StringSchema {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StringSchemaFormat>>,
            T::Error: ::std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {e}"));
            self
        }
        pub fn max_length<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_length = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_length: {e}"));
            self
        }
        pub fn min_length<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.min_length = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min_length: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<StringSchema> for super::StringSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StringSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                format: value.format?,
                max_length: value.max_length?,
                min_length: value.min_length?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::StringSchema> for StringSchema {
        fn from(value: super::StringSchema) -> Self {
            Self {
                description: Ok(value.description),
                format: Ok(value.format),
                max_length: Ok(value.max_length),
                min_length: Ok(value.min_length),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SubscribeRequest {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::SubscribeRequestParams, ::std::string::String>,
    }
    impl ::std::default::Default for SubscribeRequest {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl SubscribeRequest {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SubscribeRequestParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SubscribeRequest> for super::SubscribeRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SubscribeRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::SubscribeRequest> for SubscribeRequest {
        fn from(value: super::SubscribeRequest) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SubscribeRequestParams {
        uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for SubscribeRequestParams {
        fn default() -> Self {
            Self {
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl SubscribeRequestParams {
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SubscribeRequestParams> for super::SubscribeRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SubscribeRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { uri: value.uri? })
        }
    }
    impl ::std::convert::From<super::SubscribeRequestParams> for SubscribeRequestParams {
        fn from(value: super::SubscribeRequestParams) -> Self {
            Self { uri: Ok(value.uri) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TextContent {
        annotations:
            ::std::result::Result<::std::option::Option<super::Annotations>, ::std::string::String>,
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TextContent {
        fn default() -> Self {
            Self {
                annotations: Ok(Default::default()),
                meta: Ok(Default::default()),
                text: Err("no value supplied for text".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TextContent {
        pub fn annotations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Annotations>>,
            T::Error: ::std::fmt::Display,
        {
            self.annotations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for annotations: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TextContent> for super::TextContent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TextContent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                annotations: value.annotations?,
                meta: value.meta?,
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TextContent> for TextContent {
        fn from(value: super::TextContent) -> Self {
            Self {
                annotations: Ok(value.annotations),
                meta: Ok(value.meta),
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TextResourceContents {
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        mime_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TextResourceContents {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                mime_type: Ok(Default::default()),
                text: Err("no value supplied for text".to_string()),
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl TextResourceContents {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn mime_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.mime_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mime_type: {e}"));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TextResourceContents> for super::TextResourceContents {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TextResourceContents,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                mime_type: value.mime_type?,
                text: value.text?,
                uri: value.uri?,
            })
        }
    }
    impl ::std::convert::From<super::TextResourceContents> for TextResourceContents {
        fn from(value: super::TextResourceContents) -> Self {
            Self {
                meta: Ok(value.meta),
                mime_type: Ok(value.mime_type),
                text: Ok(value.text),
                uri: Ok(value.uri),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Tool {
        annotations: ::std::result::Result<
            ::std::option::Option<super::ToolAnnotations>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        input_schema: ::std::result::Result<super::ToolInputSchema, ::std::string::String>,
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        output_schema: ::std::result::Result<
            ::std::option::Option<super::ToolOutputSchema>,
            ::std::string::String,
        >,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Tool {
        fn default() -> Self {
            Self {
                annotations: Ok(Default::default()),
                description: Ok(Default::default()),
                input_schema: Err("no value supplied for input_schema".to_string()),
                meta: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                output_schema: Ok(Default::default()),
                title: Ok(Default::default()),
            }
        }
    }
    impl Tool {
        pub fn annotations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ToolAnnotations>>,
            T::Error: ::std::fmt::Display,
        {
            self.annotations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for annotations: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn input_schema<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ToolInputSchema>,
            T::Error: ::std::fmt::Display,
        {
            self.input_schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_schema: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn output_schema<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ToolOutputSchema>>,
            T::Error: ::std::fmt::Display,
        {
            self.output_schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for output_schema: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Tool> for super::Tool {
        type Error = super::error::ConversionError;
        fn try_from(value: Tool) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                annotations: value.annotations?,
                description: value.description?,
                input_schema: value.input_schema?,
                meta: value.meta?,
                name: value.name?,
                output_schema: value.output_schema?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::Tool> for Tool {
        fn from(value: super::Tool) -> Self {
            Self {
                annotations: Ok(value.annotations),
                description: Ok(value.description),
                input_schema: Ok(value.input_schema),
                meta: Ok(value.meta),
                name: Ok(value.name),
                output_schema: Ok(value.output_schema),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ToolAnnotations {
        destructive_hint: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        idempotent_hint: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        open_world_hint: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        read_only_hint: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ToolAnnotations {
        fn default() -> Self {
            Self {
                destructive_hint: Ok(Default::default()),
                idempotent_hint: Ok(Default::default()),
                open_world_hint: Ok(Default::default()),
                read_only_hint: Ok(Default::default()),
                title: Ok(Default::default()),
            }
        }
    }
    impl ToolAnnotations {
        pub fn destructive_hint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.destructive_hint = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for destructive_hint: {e}"));
            self
        }
        pub fn idempotent_hint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.idempotent_hint = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for idempotent_hint: {e}"));
            self
        }
        pub fn open_world_hint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.open_world_hint = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for open_world_hint: {e}"));
            self
        }
        pub fn read_only_hint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.read_only_hint = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for read_only_hint: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ToolAnnotations> for super::ToolAnnotations {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ToolAnnotations,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                destructive_hint: value.destructive_hint?,
                idempotent_hint: value.idempotent_hint?,
                open_world_hint: value.open_world_hint?,
                read_only_hint: value.read_only_hint?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::ToolAnnotations> for ToolAnnotations {
        fn from(value: super::ToolAnnotations) -> Self {
            Self {
                destructive_hint: Ok(value.destructive_hint),
                idempotent_hint: Ok(value.idempotent_hint),
                open_world_hint: Ok(value.open_world_hint),
                read_only_hint: Ok(value.read_only_hint),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ToolInputSchema {
        properties: ::std::result::Result<
            ::std::collections::HashMap<
                ::std::string::String,
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            ::std::string::String,
        >,
        required:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ToolInputSchema {
        fn default() -> Self {
            Self {
                properties: Ok(Default::default()),
                required: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ToolInputSchema {
        pub fn properties<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::collections::HashMap<
                        ::std::string::String,
                        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                    >,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.properties = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for properties: {e}"));
            self
        }
        pub fn required<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.required = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for required: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ToolInputSchema> for super::ToolInputSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ToolInputSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                properties: value.properties?,
                required: value.required?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ToolInputSchema> for ToolInputSchema {
        fn from(value: super::ToolInputSchema) -> Self {
            Self {
                properties: Ok(value.properties),
                required: Ok(value.required),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ToolListChangedNotification {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            ::std::option::Option<super::ToolListChangedNotificationParams>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ToolListChangedNotification {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Ok(Default::default()),
            }
        }
    }
    impl ToolListChangedNotification {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<super::ToolListChangedNotificationParams>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ToolListChangedNotification> for super::ToolListChangedNotification {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ToolListChangedNotification,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::ToolListChangedNotification> for ToolListChangedNotification {
        fn from(value: super::ToolListChangedNotification) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ToolListChangedNotificationParams {
        meta: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        extra: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ToolListChangedNotificationParams {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                extra: Err("no value supplied for extra".to_string()),
            }
        }
    }
    impl ToolListChangedNotificationParams {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn extra<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.extra = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ToolListChangedNotificationParams>
        for super::ToolListChangedNotificationParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ToolListChangedNotificationParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                extra: value.extra?,
            })
        }
    }
    impl ::std::convert::From<super::ToolListChangedNotificationParams>
        for ToolListChangedNotificationParams
    {
        fn from(value: super::ToolListChangedNotificationParams) -> Self {
            Self {
                meta: Ok(value.meta),
                extra: Ok(value.extra),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ToolOutputSchema {
        properties: ::std::result::Result<
            ::std::collections::HashMap<
                ::std::string::String,
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            ::std::string::String,
        >,
        required:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ToolOutputSchema {
        fn default() -> Self {
            Self {
                properties: Ok(Default::default()),
                required: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ToolOutputSchema {
        pub fn properties<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::collections::HashMap<
                        ::std::string::String,
                        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                    >,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.properties = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for properties: {e}"));
            self
        }
        pub fn required<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.required = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for required: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ToolOutputSchema> for super::ToolOutputSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ToolOutputSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                properties: value.properties?,
                required: value.required?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ToolOutputSchema> for ToolOutputSchema {
        fn from(value: super::ToolOutputSchema) -> Self {
            Self {
                properties: Ok(value.properties),
                required: Ok(value.required),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UnsubscribeRequest {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::UnsubscribeRequestParams, ::std::string::String>,
    }
    impl ::std::default::Default for UnsubscribeRequest {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl UnsubscribeRequest {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UnsubscribeRequestParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UnsubscribeRequest> for super::UnsubscribeRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UnsubscribeRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::UnsubscribeRequest> for UnsubscribeRequest {
        fn from(value: super::UnsubscribeRequest) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UnsubscribeRequestParams {
        uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UnsubscribeRequestParams {
        fn default() -> Self {
            Self {
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl UnsubscribeRequestParams {
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UnsubscribeRequestParams> for super::UnsubscribeRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UnsubscribeRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { uri: value.uri? })
        }
    }
    impl ::std::convert::From<super::UnsubscribeRequestParams> for UnsubscribeRequestParams {
        fn from(value: super::UnsubscribeRequestParams) -> Self {
            Self { uri: Ok(value.uri) }
        }
    }
}
