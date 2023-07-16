
let sign_resp = send_apdu(dev, U2F_AUTHENTICATE, flags, &sign_data)?;
Ok(&sign_resp[0..2] == SW_CONDITIONS_NOT_SATISFIED)
