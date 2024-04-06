use chrono::NaiveDate;
use octocrab::Octocrab;
use std::env;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    // Token
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    // Args
    let args: Vec<String> = env::args().collect();
    let start = NaiveDate::parse_from_str(&args[1], "%Y-%m-%d").expect("Failed to parse date");
    let end = NaiveDate::parse_from_str(&args[2], "%Y-%m-%d").expect("Failed to parse date");
    println!("Args {} - {}", start, end);

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let engines = vec![
        "Rust-SDL2/rust-sdl2",
        "bevyengine/bevy",
        "PistonDevelopers/piston",
        "not-fl3/macroquad",
        "ggez/ggez",
        "nannou-org/nannou",
        "jeremyletang/rust-sfml",
        "amethyst/bracket-lib",
        "17cupsofcoffee/tetra",
        "godot-rust/gdnative",
        "deltaphc/raylib-rs",
        "PsichiX/oxygengine",
        "VincentFoulon80/console_engine",
        "AryanpurTech/BlueEngine",
        "Nazariglez/notan",
        "CleanCut/rusty_engine",
        "geng-engine/geng",
        "FyroxEngine/Fyrox",
        "redpenguinyt/gemini-rust",
        "attackgoat/screen-13",
        "MalekiRe/stereokit-rs",
        "jice-nospam/doryen-rs",
        "polymonster/hotline",
        "AmbientRun/Ambient",
        "PistonDevelopers/turbine",
        "markusmoenig/Eldiron",
        "JustAPotota/defold-rs",
        "leetvr/hotham",
        "PikuseruConsole/pikuseru",
        "gamercade-io/gamercade_console",
        "jjant/runty8",
        "Maix0/pixel_engine",
    ];

    for engine in engines {
        let (owner, repo) = engine.split_once('/').unwrap();

        println!("{}/{}", owner, repo);

        let releases = octocrab
            .repos(owner, repo)
            .releases()
            .list()
            .send()
            .await?
            .items;

        for release in releases {
            println!(
                "Found release: {status} {tag_name} {published_at}",
                published_at = release.published_at.unwrap(),
                tag_name = release.tag_name,
                status = if release.published_at.unwrap().naive_utc().date() >= start
                    && release.published_at.unwrap().naive_utc().date() <= end
                {
                    "✅"
                } else {
                    "❌"
                }
            );
        }
    }

    Ok(())
}