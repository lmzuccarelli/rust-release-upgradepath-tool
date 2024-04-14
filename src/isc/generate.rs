use crate::UpgradeResult;
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IscV2Alpha1 {
    pub api_version: String,
    pub channel: String,
    pub min_version: String,
    pub max_version: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IscV3Alpha1 {
    pub api_version: String,
    pub version: String,
    pub image: String,
}

impl IscV2Alpha1 {
    pub fn new() -> Self {
        IscV2Alpha1 {
            api_version: "v2alpha1".to_string(),
            channel: "".to_string(),
            min_version: "".to_string(),
            max_version: "".to_string(),
        }
    }

    pub fn to_yaml(&self, channel: String, results: Vec<UpgradeResult>) -> String {
        let mut body = "".to_string();
        let yaml = format!(
            "\n---\napiVersion: mirror.openshift/{}
kind: ImageSetConfiguration
metadata:
  name: ImageSetConfiguration
  annotations: 
    autogenerated: 'rust-release-introspection-tool'
mirror:
  platform:
",
            self.api_version
        );
        for result in results {
            body += &format!(
                "    channels: 
    - name: {}
      minVersion: {:#?}
      maxVersion: {:#?}\n",
                channel, result.version, result.version
            );
        }
        let all = yaml + &body;
        all
    }
}

impl IscV3Alpha1 {
    pub fn new() -> Self {
        IscV3Alpha1 {
            api_version: "v3alpha1".to_string(),
            version: "".to_string(),
            image: "".to_string(),
        }
    }

    pub fn to_yaml(&self, results: Vec<UpgradeResult>) -> String {
        let mut body = "".to_string();
        let yaml = format!(
            "\n---\napiVersion: mirror.openshift/{}
kind: ImageSetConfiguration
metadata:
  name: ImageSetConfiguration
  annotations: 
    autogenerated: 'rust-release-introspection-tool-tool'
mirror:
  platform:
",
            self.api_version
        );
        for result in results {
            body += &format!(
                "  - name: {}
    images: {}\n",
                result.version, result.image
            );
        }
        let all = yaml + &body;
        all
    }
}
