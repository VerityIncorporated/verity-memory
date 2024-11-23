use crate::errors::AobScanError;

pub(crate) fn convert_pattern(pattern: &str) -> Result<Vec<u8>, AobScanError> {
    pattern.split_whitespace()
        .map(|s| if s == "??" {
            Ok(0x00)
        } else {
            u8::from_str_radix(s, 16).map_err(|_| AobScanError::InvalidPattern)
        })
        .collect()
}

pub(crate) fn kmp_search_unique(data: &[u8], pattern: &[u8]) -> Result<usize, AobScanError> {
    if pattern.is_empty() {
        return Err(AobScanError::InvalidPattern);
    }

    let lps = compute_lps(pattern);
    let mut i = 0;
    let mut j = 0;

    while i < data.len() {
        if pattern[j] == data[i] || pattern[j] == 0x00 {
            i += 1;
            j += 1;
        }

        if j == pattern.len() {
            return Ok(i - j);
        } else if i < data.len() && pattern[j] != data[i] && pattern[j] != 0x00 {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }

    Err(AobScanError::PatternNotFound)
}

pub(crate) fn kmp_search_all(data: &[u8], pattern: &[u8]) -> Result<Vec<usize>, AobScanError> {
    if pattern.is_empty() {
        return Err(AobScanError::InvalidPattern);
    }

    let lps = compute_lps(pattern);
    let mut indices = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < data.len() {
        if pattern[j] == data[i] || pattern[j] == 0x00 {
            i += 1;
            j += 1;
        }

        if j == pattern.len() {
            indices.push(i - j);
            j = lps[j - 1];
        } else if i < data.len() && pattern[j] != data[i] && pattern[j] != 0x00 {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }

    if indices.is_empty() {
        Err(AobScanError::PatternNotFound)
    } else {
        Ok(indices)
    }
}

pub(crate) fn compute_lps(pattern: &[u8]) -> Vec<usize> {
    let mut lps = vec![0; pattern.len()];
    let mut j = 0;
    let mut i = 1;

    while i < pattern.len() {
        if pattern[i] == pattern[j] || pattern[j] == 0x00 {
            j += 1;
            lps[i] = j;
            i += 1;
        } else {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }

    lps
}