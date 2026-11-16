use crate::{Config, HoistingLimits, NodeLinker};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NodeLinkerSetting {
    Type(NodeLinker),
    Options(NodeLinkerOptions),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case", deny_unknown_fields)]
pub enum NodeLinkerOptions {
    Isolated {
        #[serde(default)]
        hoist: Option<HoistSetting>,
    },
    Hoisted {
        #[serde(default)]
        #[serde(rename = "hoistingLimits")]
        hoisting_limits: Option<HoistingLimits>,
    },
    Pnp,
    Loaded {
        #[serde(default)]
        excluded: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HoistSetting {
    Enabled(bool),
    Patterns(NodeLinkerHoistPatterns),
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeLinkerHoistPatterns {
    pub public: Option<Vec<String>>,
    pub private: Option<Vec<String>>,
}

impl From<NodeLinker> for NodeLinkerSetting {
    fn from(linker: NodeLinker) -> Self {
        Self::Type(linker)
    }
}

impl NodeLinkerSetting {
    /// Apply the selected linker and its options.
    pub fn apply_to(self, config: &mut Config) {
        match self {
            Self::Type(linker) => {
                config.node_linker = linker;
                config.node_linker_excluded.clear();
            }
            Self::Options(NodeLinkerOptions::Isolated { hoist }) => {
                config.node_linker = NodeLinker::Isolated;
                config.node_linker_excluded.clear();
                match hoist {
                    Some(HoistSetting::Enabled(false)) => {
                        config.hoist = false;
                        config.hoist_pattern = None;
                        config.public_hoist_pattern = None;
                    }
                    Some(HoistSetting::Enabled(true)) => {
                        config.hoist = true;
                        config.hoist_pattern = Some(crate::default_hoist_pattern());
                        config.public_hoist_pattern = Some(crate::default_public_hoist_pattern());
                    }
                    Some(HoistSetting::Patterns(patterns)) => {
                        config.hoist = true;
                        if let Some(private) = patterns.private {
                            config.hoist_pattern = Some(private);
                        }
                        if let Some(public) = patterns.public {
                            config.public_hoist_pattern = Some(public);
                        }
                    }
                    None => {}
                }
            }
            Self::Options(NodeLinkerOptions::Hoisted { hoisting_limits }) => {
                config.node_linker = NodeLinker::Hoisted;
                config.node_linker_excluded.clear();
                if let Some(limits) = hoisting_limits {
                    config.hoisting_limits = limits;
                }
            }
            Self::Options(NodeLinkerOptions::Pnp) => {
                config.node_linker = NodeLinker::Pnp;
                config.node_linker_excluded.clear();
            }
            Self::Options(NodeLinkerOptions::Loaded { excluded }) => {
                config.node_linker = NodeLinker::Loaded;
                config.node_linker_excluded = excluded;
            }
        }
    }

    pub(crate) fn from_resolved(config: &Config) -> Self {
        match config.node_linker {
            NodeLinker::Isolated => Self::Options(NodeLinkerOptions::Isolated {
                hoist: Some(if config.hoist {
                    HoistSetting::Patterns(NodeLinkerHoistPatterns {
                        public: config.public_hoist_pattern.clone(),
                        private: config.hoist_pattern.clone(),
                    })
                } else {
                    HoistSetting::Enabled(false)
                }),
            }),
            NodeLinker::Hoisted => Self::Options(NodeLinkerOptions::Hoisted {
                hoisting_limits: Some(config.hoisting_limits),
            }),
            NodeLinker::Loaded => Self::Options(NodeLinkerOptions::Loaded {
                excluded: config.node_linker_excluded.clone(),
            }),
            NodeLinker::Pnp => Self::Type(NodeLinker::Pnp),
        }
    }
}

#[cfg(test)]
mod tests;
