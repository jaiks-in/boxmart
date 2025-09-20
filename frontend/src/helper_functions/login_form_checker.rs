use crate::utils::user_types::LoginPayload;

pub fn login_form_checker(payload: LoginPayload) -> Result<LoginPayload, String> {
    match validate_form(&payload) {
        Ok(()) => Ok(payload),
        Err(err) => Err(err),
    }
}

fn validate_form(payload: &LoginPayload) -> Result<(), String> {
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

    // All validations passed
    Ok(())
}
