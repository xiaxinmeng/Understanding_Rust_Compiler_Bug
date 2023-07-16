rust
#[macro_use] extern crate failure_derive;

#[derive(Fail, Debug)]
#[fail(display = "An error occurred.")]
struct SiteError;

impl From<std::option::NoneError> for SiteError {
    fn from(_err: std::option::NoneError) -> Self {
        SiteError
    }
}

fn build_piece(cur_dir: &PathBuf, piece: &PathBuf) -> Result<Piece, SiteError> {
    let title: String = piece
        .file_stem()?
        .to_str()?
        .to_string();
    Ok(Piece {
        title: title,
        url: piece
            .strip_prefix(cur_dir)?
            .to_str()
            .ok_or(err_msg("tostr"))?
            .to_string(),
    })
}

