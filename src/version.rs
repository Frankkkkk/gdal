use libc::c_char;
use std::ffi::CString;
use utils::_string;

#[link(name="gdal")]
extern {
    fn GDALVersionInfo(key: *const c_char) -> *const c_char;
}

pub fn version_info(key: &str) -> String {
    let c_key = CString::new(key.as_bytes()).unwrap();
    return _string(unsafe { GDALVersionInfo(c_key.as_ptr()) });
}

#[cfg(test)]
mod tests {
    use super::version_info;

    #[test]
    fn test_version_info() {
        let release_date = version_info("RELEASE_DATE");
        let release_name = version_info("RELEASE_NAME");
        let version_text = version_info("--version");

        let expected_text: String = format!(
            "GDAL {}, released {}/{}/{}",
            release_name,
            release_date.as_str().slice_chars(0, 4),
            release_date.as_str().slice_chars(4, 6),
            release_date.as_str().slice_chars(6, 8),
        );

        assert_eq!(version_text, expected_text);
    }
}
