use custom_logger::Logging;
use semver::Version;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Vec<u32>>,
    pub conditional_edges: Vec<ConditionalEdge>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub version: String,
    pub payload: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionalEdge {
    pub edges: Vec<Edge>,
    pub risks: Vec<Risk>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Risk {
    pub url: String,
    pub name: String,
    pub message: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    pub from: String,
    pub to: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeResult {
    pub version: String,
    pub image: String,
}

// parse the json for graphdata
pub fn parse_json_graphdata(data: String) -> Result<Graph, Box<dyn std::error::Error>> {
    // Parse the string of data into serde_json::ManifestSchema.
    let graph: Graph = serde_json::from_str(&data)?;
    Ok(graph)
}

// calculate the upgradepath
pub fn get_upgrade_path(
    log: &Logging,
    from_version: String,
    to_version: String,
    graphdata: Graph,
) -> Vec<UpgradeResult> {
    // get ConditionalEdge
    let mut to: Vec<Version> = vec![];
    let mut risks: Vec<Risk> = vec![];
    let mut upgrade_images: Vec<UpgradeResult> = vec![];

    for edge in graphdata.conditional_edges.iter() {
        for e in edge.edges.iter() {
            if e.from == from_version {
                let version = Version::parse(&e.to).unwrap();
                to.push(version);
                for r in edge.risks.iter() {
                    risks.push(r.clone());
                }
            }
        }
    }

    to.sort();
    log.lo(&format!("list : {:#?}", to.len()));

    if to.len() == 0 {
        return vec![UpgradeResult {
            version: "".to_string(),
            image: "".to_string(),
        }];
    }

    let last_version = to[to.len() - 1].to_string();

    // find the index of the node with version 4.13.38
    let mut counter: u32 = 0;
    let mut index: u32 = 0;
    for node in graphdata.nodes.iter() {
        if node.version == last_version.to_string() {
            index = counter;
            log.lo(&format!("index: {}", index));
        }
        counter += 1;
    }

    /*
    for risk in risks.iter() {
        println!("risks : ");
        println!("        {}", risk.url);
        println!("        {}", risk.name);
        println!("        {}", risk.message);
    }
    */

    let mut counter: u32 = 0;
    let mut to_index: u32 = 0;

    for node in graphdata.nodes.iter() {
        if node.version == to_version {
            to_index = counter;
        }
        counter += 1;
    }

    // get all toVersion links
    for edge in graphdata.edges.iter() {
        if edge[0] == to_index {
            //println!("to_version {} {}",edge[1], graphdata.nodes[edge[1]].version);
        }
    }

    let mut upgrade_list: Vec<Version> = vec![];
    // get all links from this index
    for edge in graphdata.edges.iter() {
        if edge[0] == index {
            let idx = edge[1] as usize;
            let version = Version::parse(&graphdata.nodes[idx].version).unwrap();
            upgrade_list.push(version);
        }
    }
    upgrade_list.push(Version::parse(&from_version).unwrap());
    upgrade_list.push(Version::parse(&last_version).unwrap());
    upgrade_list.push(Version::parse(&to_version).unwrap());
    upgrade_list.sort();

    // finally look up the image references (for v3)
    for node in graphdata.nodes.iter() {
        for version in upgrade_list.iter() {
            if node.version == version.to_string() {
                match &node.payload {
                    Some(image) => {
                        let upgrade_result = UpgradeResult {
                            version: version.to_string(),
                            image: image.clone(),
                        };
                        upgrade_images.push(upgrade_result);
                    }
                    None => {
                        log.lo("no image found");
                    }
                }
            }
        }
    }
    upgrade_images.sort_by(|a, b| a.version.cmp(&b.version));
    upgrade_images
}
