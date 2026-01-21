use pollux_capabilities::{Capability, LifecyclePhase, ModuleType, Target};
use pollux_contracts::{validate_authorization, validate_lifecycle_transition, validate_target};
use std::str::FromStr;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeRequestType {
    ValidateManifest,
    ValidateCapabilityRequest,
    ValidateLifecycleTransition,
    ValidateTargetCompatibility,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BridgeRequest {
    request_type: BridgeRequestType,
    payload: BridgePayload,
}

#[derive(Debug, Clone)]
pub enum BridgePayload {
    ManifestValidation(ManifestValidationPayload),
    CapabilityRequest(CapabilityRequestPayload),
    LifecycleTransition(LifecycleTransitionPayload),
    TargetCompatibility(TargetCompatibilityPayload),
}

#[derive(Debug, Clone)]
pub struct ManifestValidationPayload {
    pub capabilities: Vec<String>,
    pub target: String,
    pub module_type: String,
}

#[derive(Debug, Clone)]
pub struct CapabilityRequestPayload {
    pub capability: String,
}

#[derive(Debug, Clone)]
pub struct LifecycleTransitionPayload {
    pub from_phase: String,
    pub to_phase: String,
}

#[derive(Debug, Clone)]
pub struct TargetCompatibilityPayload {
    pub declared_target: String,
    pub runtime_target: String,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeResponseStatus {
    Approved,
    Rejected,
    InvalidRequest,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BridgeResponse {
    pub status: BridgeResponseStatus,
    pub reason: Option<String>,
}

impl BridgeResponse {
    pub fn approved() -> Self {
        Self {
            status: BridgeResponseStatus::Approved,
            reason: None,
        }
    }

    pub fn rejected(reason: String) -> Self {
        Self {
            status: BridgeResponseStatus::Rejected,
            reason: Some(reason),
        }
    }

    pub fn invalid_request(reason: String) -> Self {
        Self {
            status: BridgeResponseStatus::InvalidRequest,
            reason: Some(reason),
        }
    }
}

pub struct PolyglotBridge;

impl PolyglotBridge {
    pub fn process_request(request: BridgeRequest) -> BridgeResponse {
        match request.payload {
            BridgePayload::ManifestValidation(payload) => {
                Self::validate_manifest(payload)
            }
            BridgePayload::CapabilityRequest(payload) => {
                Self::validate_capability_request(payload)
            }
            BridgePayload::LifecycleTransition(payload) => {
                Self::validate_lifecycle_transition(payload)
            }
            BridgePayload::TargetCompatibility(payload) => {
                Self::validate_target_compatibility(payload)
            }
        }
    }

    fn validate_manifest(payload: ManifestValidationPayload) -> BridgeResponse {
        let target = match Self::parse_target(&payload.target) {
            Ok(t) => t,
            Err(e) => return BridgeResponse::invalid_request(e),
        };

        let module_type = match Self::parse_module_type(&payload.module_type) {
            Ok(mt) => mt,
            Err(e) => return BridgeResponse::invalid_request(e),
        };

        let capabilities: Result<Vec<Capability>, _> = payload
            .capabilities
            .iter()
            .map(|s| Capability::from_str(s.as_str()))
            .collect();

        let capabilities = match capabilities {
            Ok(caps) => caps,
            Err(e) => return BridgeResponse::invalid_request(format!("{}", e)),
        };

        match validate_authorization(&capabilities, target, module_type) {
            Ok(()) => BridgeResponse::approved(),
            Err(e) => BridgeResponse::rejected(format!("{}", e)),
        }
    }

    fn validate_capability_request(payload: CapabilityRequestPayload) -> BridgeResponse {
        match Capability::from_str(&payload.capability) {
            Ok(_) => BridgeResponse::approved(),
            Err(e) => BridgeResponse::invalid_request(format!("{}", e)),
        }
    }

    fn validate_lifecycle_transition(payload: LifecycleTransitionPayload) -> BridgeResponse {
        let from_phase = match Self::parse_lifecycle_phase(&payload.from_phase) {
            Ok(p) => p,
            Err(e) => return BridgeResponse::invalid_request(e),
        };

        let to_phase = match Self::parse_lifecycle_phase(&payload.to_phase) {
            Ok(p) => p,
            Err(e) => return BridgeResponse::invalid_request(e),
        };

        match validate_lifecycle_transition(from_phase, to_phase) {
            Ok(()) => BridgeResponse::approved(),
            Err(e) => BridgeResponse::rejected(format!("{}", e)),
        }
    }

    fn validate_target_compatibility(payload: TargetCompatibilityPayload) -> BridgeResponse {
        let declared = match Self::parse_target(&payload.declared_target) {
            Ok(t) => t,
            Err(e) => return BridgeResponse::invalid_request(e),
        };

        let runtime = match Self::parse_target(&payload.runtime_target) {
            Ok(t) => t,
            Err(e) => return BridgeResponse::invalid_request(e),
        };

        match validate_target(declared, runtime) {
            Ok(()) => BridgeResponse::approved(),
            Err(e) => BridgeResponse::rejected(format!("{}", e)),
        }
    }

    fn parse_target(s: &str) -> Result<Target, String> {
        match s {
            "streamline" => Ok(Target::Streamline),
            "coresense" => Ok(Target::CoreSense),
            "both" => Ok(Target::Both),
            _ => Err(format!("Invalid target: {}", s)),
        }
    }

    fn parse_module_type(s: &str) -> Result<ModuleType, String> {
        match s {
            "core_module" => Ok(ModuleType::CoreModule),
            "extension" => Ok(ModuleType::Extension),
            "theme" => Ok(ModuleType::Theme),
            "tool" => Ok(ModuleType::Tool),
            _ => Err(format!("Invalid module type: {}", s)),
        }
    }

    fn parse_lifecycle_phase(s: &str) -> Result<LifecyclePhase, String> {
        match s {
            "discovery" => Ok(LifecyclePhase::Discovery),
            "admission" => Ok(LifecyclePhase::Admission),
            "resolution" => Ok(LifecyclePhase::Resolution),
            "initialization" => Ok(LifecyclePhase::Initialization),
            "activation" => Ok(LifecyclePhase::Activation),
            "operational" => Ok(LifecyclePhase::Operational),
            "suspension" => Ok(LifecyclePhase::Suspension),
            "termination" => Ok(LifecyclePhase::Termination),
            _ => Err(format!("Invalid lifecycle phase: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_validation_success() {
        let request = BridgeRequest {
            request_type: BridgeRequestType::ValidateManifest,
            payload: BridgePayload::ManifestValidation(ManifestValidationPayload {
                capabilities: vec!["filesystem.read".into()],
                target: "both".into(),
                module_type: "extension".into(),
            }),
        };

        let response = PolyglotBridge::process_request(request);
        assert_eq!(response.status, BridgeResponseStatus::Approved);
    }

    #[test]
    fn test_manifest_validation_forbidden_combination() {
        let request = BridgeRequest {
            request_type: BridgeRequestType::ValidateManifest,
            payload: BridgePayload::ManifestValidation(ManifestValidationPayload {
                capabilities: vec!["filesystem.write".into(), "process.execute".into()],
                target: "both".into(),
                module_type: "extension".into(),
            }),
        };

        let response = PolyglotBridge::process_request(request);
        assert_eq!(response.status, BridgeResponseStatus::Rejected);
    }

    #[test]
    fn test_manifest_validation_invalid_capability() {
        let request = BridgeRequest {
            request_type: BridgeRequestType::ValidateManifest,
            payload: BridgePayload::ManifestValidation(ManifestValidationPayload {
                capabilities: vec!["invalid.capability".into()],
                target: "both".into(),
                module_type: "extension".into(),
            }),
        };

        let response = PolyglotBridge::process_request(request);
        assert_eq!(response.status, BridgeResponseStatus::InvalidRequest);
    }

    #[test]
    fn test_lifecycle_transition_valid() {
        let request = BridgeRequest {
            request_type: BridgeRequestType::ValidateLifecycleTransition,
            payload: BridgePayload::LifecycleTransition(LifecycleTransitionPayload {
                from_phase: "discovery".into(),
                to_phase: "admission".into(),
            }),
        };

        let response = PolyglotBridge::process_request(request);
        assert_eq!(response.status, BridgeResponseStatus::Approved);
    }

    #[test]
    fn test_lifecycle_transition_invalid() {
        let request = BridgeRequest {
            request_type: BridgeRequestType::ValidateLifecycleTransition,
            payload: BridgePayload::LifecycleTransition(LifecycleTransitionPayload {
                from_phase: "discovery".into(),
                to_phase: "operational".into(),
            }),
        };

        let response = PolyglotBridge::process_request(request);
        assert_eq!(response.status, BridgeResponseStatus::Rejected);
    }

    #[test]
    fn test_target_compatibility_match() {
        let request = BridgeRequest {
            request_type: BridgeRequestType::ValidateTargetCompatibility,
            payload: BridgePayload::TargetCompatibility(TargetCompatibilityPayload {
                declared_target: "streamline".into(),
                runtime_target: "streamline".into(),
            }),
        };

        let response = PolyglotBridge::process_request(request);
        assert_eq!(response.status, BridgeResponseStatus::Approved);
    }

    #[test]
    fn test_target_compatibility_mismatch() {
        let request = BridgeRequest {
            request_type: BridgeRequestType::ValidateTargetCompatibility,
            payload: BridgePayload::TargetCompatibility(TargetCompatibilityPayload {
                declared_target: "streamline".into(),
                runtime_target: "coresense".into(),
            }),
        };

        let response = PolyglotBridge::process_request(request);
        assert_eq!(response.status, BridgeResponseStatus::Rejected);
    }

    #[test]
    fn test_target_both_always_compatible() {
        let request = BridgeRequest {
            request_type: BridgeRequestType::ValidateTargetCompatibility,
            payload: BridgePayload::TargetCompatibility(TargetCompatibilityPayload {
                declared_target: "both".into(),
                runtime_target: "streamline".into(),
            }),
        };

        let response = PolyglotBridge::process_request(request);
        assert_eq!(response.status, BridgeResponseStatus::Approved);
    }
}

