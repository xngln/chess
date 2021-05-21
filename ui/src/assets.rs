const ROOT: &str = "https://youthful-ritchie-fda6ba.netlify.app";

// themes
const TOURNAMENT: &str = "tournament";

const SELECTED_THEME: &str = TOURNAMENT;

// icons
const LOGO: &str = "pawn-dark.svg";

// board
const BOARD: &str = "board.png";

// Pieces
const WP: &str = "WP.png";
const WN: &str = "WN.png";
const WB: &str = "WB.png";
const WR: &str = "WR.png";
const WQ: &str = "WQ.png";
const WK: &str = "WK.png";

const BP: &str = "BP.png";
const BN: &str = "BN.png";
const BB: &str = "BB.png";
const BR: &str = "BR.png";
const BQ: &str = "BQ.png";
const BK: &str = "BK.png";

pub enum Image {
    Logo,

    Board,

    Wp,
    Wn,
    Wb,
    Wr,
    Wq,
    Wk,

    Bp,
    Bn,
    Bb,
    Br,
    Bq,
    Bk,
}

pub fn url(image: Image) -> String {
    let path: String;
    match image {
        Image::Logo => {
            path = String::from(LOGO);
        }

        Image::Board => {
            path = format!("{}/{}", SELECTED_THEME, BOARD);
        }

        Image::Wp => {
            path = format!("{}/{}", SELECTED_THEME, WP);
        }
        Image::Wn => {
            path = format!("{}/{}", SELECTED_THEME, WN);
        }
        Image::Wb => {
            path = format!("{}/{}", SELECTED_THEME, WB);
        }
        Image::Wr => {
            path = format!("{}/{}", SELECTED_THEME, WR);
        }
        Image::Wk => {
            path = format!("{}/{}", SELECTED_THEME, WK);
        }
        Image::Wq => {
            path = format!("{}/{}", SELECTED_THEME, WQ);
        }

        Image::Bp => {
            path = format!("{}/{}", SELECTED_THEME, BP);
        }
        Image::Bn => {
            path = format!("{}/{}", SELECTED_THEME, BN);
        }
        Image::Bb => {
            path = format!("{}/{}", SELECTED_THEME, BB);
        }
        Image::Br => {
            path = format!("{}/{}", SELECTED_THEME, BR);
        }
        Image::Bq => {
            path = format!("{}/{}", SELECTED_THEME, BQ);
        }
        Image::Bk => {
            path = format!("{}/{}", SELECTED_THEME, BK);
        }
    };

    let url = format!("{}/{}", ROOT, path);
    return url;
}
