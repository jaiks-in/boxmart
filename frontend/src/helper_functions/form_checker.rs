use crate::utils::user_types::{CheckPayload, SignupPayload};
use leptos::logging::log;

pub fn form_checker(payload: &CheckPayload) -> Result<SignupPayload, String> {
    log!("=== FORM VALIDATION STARTED ===");

    // Validate all fields first - this returns Result<(), String>
    match validate_fields(payload) {
        Ok(()) => {
            log!("=== ALL VALIDATIONS PASSED ===");
            // If validations pass, return Ok with SignupPayload
            Ok(SignupPayload {
                name: payload.name.clone(),
                username: payload.username.clone(),
                email: payload.email.clone(),
                password: payload.password.clone(),
                org_name: payload.org_name.clone(),
                user_type: payload.user_type.clone(),
            })
        }
        Err(error_message) => {
            log!("=== VALIDATION FAILED: {} ===", error_message);
            // If validations fail, return Err with error message
            Err(error_message)
        }
    }
}

// Validation function that returns Result<(), String>
fn validate_fields(payload: &CheckPayload) -> Result<(), String> {
    // Username validation
    if payload.username.trim().is_empty() {
        return Err("Username required".to_string());
    }
    if payload.username.trim().len()<3{
         return Err("Username must be more than 2 characters".to_string());
    }
    
    // Email validation
    if payload.email.trim().is_empty() {
        return Err("Email required".to_string());
    }
    
    if !payload.email.contains('@') || !payload.email.contains('.') {
        return Err("Valid email address required".to_string());
    }
    
    // Password validation
    if payload.password.len() < 6 {
        return Err("Password must be at least 6 characters".to_string());
    }
    
    // Password confirmation
    if payload.password != payload.confirm_password {
        return Err("Passwords must match".to_string());
    }

    // All validations passed
    Ok(())
}