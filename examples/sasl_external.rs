use std::{
    env,
    ptr,
    ffi::{
        CStr,
        CString,
        c_void,
    },
};

use anyhow::{anyhow, Context, Result};

use libldap_sys::{
    LDAP,
    LDAP_SUCCESS,
    LDAP_VERSION3,
    LDAP_OPT_PROTOCOL_VERSION,
    LDAP_SASL_QUIET,
    sasl_interact_t,
    ldap_initialize,
    ldap_set_option,
    ldap_err2string,
    ldap_unbind_ext_s,
    ldap_sasl_interactive_bind_s,
};

extern "C" fn sasl_interact(
    _ldap: *mut LDAP,
    _msgid: u32,
    _arg: *mut c_void,
    interact_ptr: *mut c_void,
) -> i32 {
    log::debug!("SASL interact callback invoked");
    unsafe {
        let mut interact = interact_ptr as *mut sasl_interact_t;
        while !(*interact).prompt.is_null() {
            log::debug!(
                "SASL interaction: id={}, prompt={}",
                (*interact).id,
                CStr::from_ptr((*interact).prompt)
                    .to_string_lossy()
                    .into_owned()
            );
            (*interact).result = ptr::null(); // no user interaction required for external
            (*interact).len = 0;
            interact = interact.add(1); // continue to the next interaction
        }

    }
    LDAP_SUCCESS as i32
}

pub unsafe fn ldap_error_to_string(result: i32) -> String {
    CStr::from_ptr(ldap_err2string(result)).to_string_lossy().into_owned()
}

fn main() -> Result<()> {
    env_logger::init();
    log::debug!("Logger initialized!");

    let socket_path = env::var("LDAP_SOCKET_PATH")
        .context("Environment variable `LDAP_SOCKET_PATH` not set")?;
    log::debug!("Connecting to the socket path: {}", &socket_path);

    let socket_path = urlencoding::encode(&socket_path);

    let ldap_url = CString::new(format!("ldapi://{}/", socket_path))
        .context("CString::new failed")?;
    let mut ldap: *mut LDAP = ptr::null_mut();

    let result = unsafe {
        ldap_initialize(&mut ldap, ldap_url.as_ptr())
    };
    if result != LDAP_SUCCESS as i32 {
        return Err(anyhow!("LDAP initialization failed: {}", unsafe {
            ldap_error_to_string(result)
        }));
    }
    log::debug!("LDAP initialized successfully.");

    let version: i32 = LDAP_VERSION3 as i32;
    unsafe {
        ldap_set_option(
            ldap,
            LDAP_OPT_PROTOCOL_VERSION as i32,
            &version as *const _ as *const std::os::raw::c_void,
        );
    }
    log::debug!("LDAP protocol version set to 3.");

    log::debug!("Performing SASL EXTERNAL bind...");
    let mech = CString::new("EXTERNAL").context("CString::new failed")?;

    let bind_result = unsafe {
        ldap_sasl_interactive_bind_s(
            ldap,
            ptr::null(),         // dn
            mech.as_ptr(),       // sasl mechanism
            ptr::null_mut(),     // server controls
            ptr::null_mut(),     // client controls
            LDAP_SASL_QUIET,     // quiet mode
            Some(sasl_interact), // interaction callback
            ptr::null_mut(),     // no argument
        )
    };

    if bind_result != LDAP_SUCCESS as i32 {
        log::error!("SASL bind failed: {}", unsafe {
            ldap_error_to_string(bind_result)
        });
    } else {
        log::info!("SASL EXTERNAL bind successful!");
    }

    unsafe {
        ldap_unbind_ext_s(ldap, ptr::null_mut(), ptr::null_mut());
    }
    log::debug!("Cleanup completed.");
    Ok(())
}

