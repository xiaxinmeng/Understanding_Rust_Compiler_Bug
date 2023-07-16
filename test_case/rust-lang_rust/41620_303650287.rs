rust
/// Calculates the UTM zone this longitude falls in
/// Handles exceptions for Norway / Svalbard
/// For a visual representation: https://upload.wikimedia.org/wikipedia/commons/a/a5/UTM-Zone.svg
///
/// Inputs: Longitude, in degrees
///         Latitude, in degrees
///
/// Returns: UTM Zone
///
#[allow(non_snake_case)]
pub fn get_utm_zone(lon: f64, lat: f64) -> u8 {

    let mut zone = ((lon + 180.0) / 6.0).floor() as u8 + 1;

    match lat {
        56.0..64.0 => {
            /* Zone V, Norway */
            match lon {
                3.0..6.0 => {
                    zone += 1;
                }
                _ => {}
            }
        }

        72.0..84.0 => {
            /* Zone X, Svalbard */
            match lon {
                6.0..9.0 => {
                    zone -= 1;
                }
                9.0..12.0 => {
                    zone += 1;
                }
                18.0..21.0 => {
                    zone -= 1;
                }
                21.0..24.0 => {
                    zone += 1;
                }
                30.0..33.0 => {
                    zone -= 1;
                }
                33.0..36.0 => {
                    zone += 1;
                }
                _ => {}
            }
        }

        _ => {}
    }

    zone
}
