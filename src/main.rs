use sbm::{Category, Sbm, Bookmark, Header};
use rand::{seq::SliceRandom, Rng};

// Consider using lazy_static for these constants
const NAMES: [&str; 55] = [
    "Google Search",
    "DuckDuckGo",
    "Rust Language",
    "GitHub",
    "GitLab",
    "BitBucket",
    "StackOverflow",
    "Reddit",
    "Hacker News",
    "Medium",
    "Dev.to",
    "Wikipedia",
    "YouTube",
    "Twitch",
    "Netflix",
    "Amazon",
    "eBay",
    "AliExpress",
    "Wish",
    "Facebook",
    "Twitter",
    "Instagram",
    "LinkedIn",
    "Snapchat",
    "TikTok",
    "WhatsApp",
    "Telegram",
    "Signal",
    "Discord",
    "Slack",
    "Zoom",
    "Microsoft",
    "Apple",
    "Google",
    "Mozilla",
    "Opera",
    "Brave",
    "Vivaldi",
    "Edge",
    "Safari",
    "Firefox",
    "Chrome",
    "Chromium",
    "Tor",
    "NordVPN",
    "ExpressVPN",
    "ProtonVPN",
    "OpenVPN",
    "WireGuard",
    "IPVanish",
    "Surfshark",
    "CyberGhost",
    "Private Internet Access",
    "NordPass",
    "LastPass",
];

const DESCRIPTIONS: [&str; 12] = [
    "Search the web",
    "Code Hosting",
    "A cool website",
    "Social Media",
    "Video Streaming",
    "Online Shopping",
    "Messaging",
    "Video Conferencing",
    "VPN",
    "Password Manager",
    "General stuff",
    "Neat project",
];

const ICONS: [&str; 10] = ["🔍", "💻", "📱", "📺", "🛍️", "💬", "📹", "🔒", "🔑", "🌐"];


fn random_bookmark(rng: &mut impl Rng) -> Bookmark {
    let name = NAMES.choose(rng).unwrap();
    let description = DESCRIPTIONS.choose(rng).unwrap();
    Bookmark {
        name: name.to_string(),
        description: description.to_string(),
        url: "https://".to_owned() + name.to_lowercase().replace(' ', "").as_str() + ".com",
    }
}

fn random_category(rng: &mut impl Rng) -> Category {
    let name = NAMES.choose(rng).unwrap().to_string();
    let icon = if rng.gen_bool(0.5) {
        Some(ICONS.choose(rng).unwrap().to_string())
    } else {
        None
    };
    let mut bookmarks = Vec::new();
    for _ in 0..rng.gen_range(1..=10) {
        bookmarks.push(random_bookmark(rng));
    }
    Category {
        header: Header {
            name,
            icon
        },
        bookmarks,
    }
}

fn generate(count: usize, rng: &mut impl Rng) -> Sbm {
    let mut sbm = Sbm(Vec::new());
    for _ in 0..count {
        sbm.0.push(random_category(rng));
    }
    sbm 
}

fn main() {
    let mut rng = rand::thread_rng();
    let count = std::env::args()
        .nth(1)
        .map_or(10, |arg| arg.parse().unwrap_or(10));
    let timer = if std::env::args().any(|arg| arg == "--time") {
        Some(std::time::Instant::now())
    } else {
        None
    };
    let sbm = generate(count, &mut rng);
    println!("{}", sbm);
    if let Some(timer) = timer {
        println!("# [bmgen] Generated {} categories with {} bookmarks in {:?}", count, sbm.0.iter().map(|c| c.bookmarks.len()).sum::<usize>(), timer.elapsed());
    }
}