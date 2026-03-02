/// Table CRC32 précalculée (polynôme IEEE 802.3 : 0xEDB88320)
const CRC32_TABLE: [u32; 256] = make_crc32_table();

const fn make_crc32_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut i = 0usize;
    while i < 256 {
        let mut crc = i as u32;
        let mut j = 0;
        while j < 8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB8_8320;
            } else {
                crc >>= 1;
            }
            j += 1;
        }
        table[i] = crc;
        i += 1;
    }
    table
}

/// Calcule le CRC32 d'un slice d'octets
pub fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &byte in data {
        let index = ((crc ^ byte as u32) & 0xFF) as usize;
        crc = (crc >> 8) ^ CRC32_TABLE[index];
    }
    crc ^ 0xFFFF_FFFF   
}

#[cfg(test)]
mod tests {
    use super::crc32;

    #[test]
    fn crc32_empty() {
        // CRC32 d'un slice vide, valeur de référence connue
        assert_eq!(crc32(&[]), 0x0000_0000);
    }

    #[test]
    fn crc32_known_vector() {
        // CRC32("123456789") = 0xCBF43926, vecteur de test standard IEEE
        let data = b"123456789";
        assert_eq!(crc32(data), 0xCBF4_3926);
    }

    #[test]
    fn crc32_single_byte() {
        assert_eq!(crc32(&[0x00]), 0xD202_EF8D);
    }
}