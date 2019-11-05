use v3::messages::{MessageType, A2AMessageKinds};
use v3::messages::mime_type::MimeType;
use error::{VcxError, VcxResult, VcxErrorKind};

pub mod credential;
pub mod credential_offer;
pub mod credential_proposal;
pub mod credential_request;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CredentialPreviewData {
    #[serde(rename = "@type")]
    pub _type: MessageType,
    pub attributes: Vec<CredentialValue>
}

impl CredentialPreviewData {
    pub fn new() -> Self {
        CredentialPreviewData {
            _type: MessageType::build(A2AMessageKinds::CredentialPreview),
            attributes: vec![]
        }
    }

    pub fn add_value(mut self, name: &str, value: &str, mime_type: MimeType) -> VcxResult<CredentialPreviewData> {
        let data_value = match mime_type {
            MimeType::Plain => {
                Ok(CredentialValue {
                    name: name.to_string(),
                    value: value.to_string(),
                    _type: None,
                })
            }
            _ => {
                Err(VcxError::from_msg(VcxErrorKind::InvalidAttributesStructure, "Invalid mime type of value in credential preview"))
            }
        }?;
        self.attributes.push(data_value);
        Ok(self)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CredentialValue {
    pub name: String,
    pub value: String,
    pub _type: Option<MimeType>,
}