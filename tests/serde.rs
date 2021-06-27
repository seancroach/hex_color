use hex_color::HexColor;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Text {
    foreground: HexColor,
    background: HexColor,
}

#[test]
fn test_serde() -> Result<()> {
    let data = r##"
    {
        "foreground": "#FFFFFF",
        "background": "#000000"
    }"##;
    let t: Text = serde_json::from_str(data)?;

    assert_eq!(t.foreground, HexColor::new(255, 255, 255));
    assert_eq!(t.background, HexColor::new(0, 0, 0));

    Ok(())
}
