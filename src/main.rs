use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

const PINK: &str = "\x1b[38;2;255;105;180;1m";       
const ROSE: &str = "\x1b[38;2;255;160;190;1m";       
const PURPLE: &str = "\x1b[38;2;195;130;255;1m";    
const CYAN: &str = "\x1b[38;2;77;208;225;1m";        
const YELLOW: &str = "\x1b[38;2;255;213;79;1m";      
const GREEN: &str = "\x1b[38;2;129;199;132;1m";      
const RED: &str = "\x1b[38;2;255;82;82;1m";          
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

fn typewriter(text: &str, delay_ms: u64) {
    for ch in text.chars() {
        print!("{}", ch);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(delay_ms));
    }
}

fn clear_screen() {
    print!("\x1b[2J\x1b[1;1H");
    io::stdout().flush().unwrap();
}

fn calculate_love_score(name1: &str, name2: &str) -> u32 {
    let combined = format!("{}-loves-{}", name1.trim().to_lowercase(), name2.trim().to_lowercase());
    let mut sum: u32 = 0;
    for (i, b) in combined.bytes().enumerate() {
        sum = sum.wrapping_add((b as u32).wrapping_mul((i as u32) + 7));
    }

   
    let lower1 = name1.to_lowercase();
    let lower2 = name2.to_lowercase();
    if lower1.contains("rust") || lower2.contains("rust") {
        return 99; 
    }
    if lower1.contains("javascript") || lower2.contains("javascript") {
        return 55; 
    }

    
    45 + (sum % 54)
}

fn draw_heart_bar(score: u32) {
    let filled = (score / 10) as usize;
    let empty = 10 - filled;
    print!("   [");
    for _ in 0..filled {
        print!("{}♥{}", RED, RESET);
    }
    for _ in 0..empty {
        print!("♡");
    }
    println!("] {BOLD}{score}%{RESET}");
}

fn print_cat_love() {
    println!("{PURPLE}           ╭──────────────────────────────────────────────╮");
    println!("           │ {ROSE}\"Chubby Billi scanning dil ka memory leak...\" {PURPLE}│");
    println!("           │  {PINK}Pyaar mein break statement mat lagana! 💖   {PURPLE}│");
    println!("           ╰───────────────────────╮──────────────────────╯");
    println!("                                  /{RESET}");
    println!("{ROSE}                  /\\_____/\\      /  ");
    println!("                 /  o   o  \\ <──'   ");
    println!("                ( ==  w  == )       ");
    println!("                 )    {RED}♥{ROSE}    (        ");
    println!("                (           )       ");
    println!("               ( (  )   (  ) )      ");
    println!("              (__(__)___(__)__)     {RESET}");
    println!("            {PINK}✨ ~ Chubby Cat in Love ~ ✨{RESET}\n");
}

fn main() {
    clear_screen();

    println!("{PURPLE}╔══════════════════════════════════════════════════════════╗{RESET}");
    println!("{PINK}║       💖  CODER'S LOVE & COMPATIBILITY CALCULATOR  💖    ║{RESET}");
    println!("{YELLOW}║          [ Powered by Rust Borrow Checker v1.0 ]         ║{RESET}");
    println!("{PURPLE}╚══════════════════════════════════════════════════════════╝{RESET}\n");

    print_cat_love();

    print!("{BOLD}Enter Your Name : {RESET}");
    io::stdout().flush().unwrap();
    let mut name1 = String::new();
    io::stdin().read_line(&mut name1).unwrap();
    let name1 = name1.trim();

    print!("{BOLD}Enter Crush / Partner: {RESET}");
    io::stdout().flush().unwrap();
    let mut name2 = String::new();
    io::stdin().read_line(&mut name2).unwrap();
    let name2 = name2.trim();

    if name1.is_empty() || name2.is_empty() {
        println!("\n{RED}Error 404: Crush Not Found! Bina naam ke hawa mein teer chala rahe ho? 💘{RESET}");
        return;
    }

    println!("\n{YELLOW}⚡ Initiating Romantic Compatibility Audit...{RESET}\n");
    sleep(Duration::from_millis(500));

    let checks = [
        "1. Wi-Fi aur dil ka signal match kar rahe hain...",
        "2. Rust Borrow Checker se jealousy & third-party audit chal rahi hai...",
        "3. Late-night text latency aur 'seen' timestamps check ho rahe hain...",
        "4. Validating Lifetime annotations: ('forever vs 'kat_gaya)...",
        "5. Checking Git merge conflicts: Koi teesra branch toh nahi ghus raha?...",
    ];

    for check in checks {
        typewriter(&format!("  {CYAN}▸ {check}{RESET} "), 8);
        sleep(Duration::from_millis(350));
        println!("{GREEN}[PASSED]{RESET}");
    }

    sleep(Duration::from_millis(400));
    println!("\n{PURPLE}╔══════════════════════════════════════════════════════════╗{RESET}");
    println!("{PINK}║              📊 AUDIT REPORT GENERATED                   ║{RESET}");
    println!("{PURPLE}╚══════════════════════════════════════════════════════════╝{RESET}\n");

    let score = calculate_love_score(name1, name2);

    print!("{BOLD}Pair: {CYAN}{name1}{RESET} {RED}♥{RESET} {CYAN}{name2}{RESET}\n");
    print!("{BOLD}Compatibility Score: {RESET}");
    draw_heart_bar(score);
    println!();

    // Verdicts
    print!("{BOLD}Official Compiler Verdict: {RESET}");
    if score >= 95 {
        println!("{GREEN}PERFECT MUTUAL OWNERSHIP! 💍🦀{RESET}");
        println!(
            "{PINK}“Bhai yeh rishta nahi, Rust ka production-ready code hai! \n\
             Zero data races, 'static lifetime amar romance! \n\
             Panditji ko bulao date fix karein, aur shaadi ke card pe QR code lagwao!”{RESET}"
        );
    } else if score >= 80 {
        println!("{GREEN}STRONG THREAD SAFETY! 💕🚀{RESET}");
        println!(
            "{CYAN}“Tuning ekdum first-class hai! \n\
             Bas compiler ki ek hi warning hai: Ek time pe sirf EK hi mutable reference rakhna... \n\
             Doosri crush ko dekha toh borrow checker ghar aake pitega! 😉”{RESET}"
        );
    } else if score >= 65 {
        println!("{YELLOW}STABLE BUT HIGH LATENCY! ☕📶{RESET}");
        println!(
            "{YELLOW}“Connection theek-thaak chal raha hai, par message 'Seen' karke late reply ka \n\
             thoda runtime overhead hai! Cache clear karo aur ek shaam momos/coffee date pe jao! ☕”{RESET}"
        );
    } else if score >= 50 {
        println!("{YELLOW}WARNING: RUNTIME OVERHEAD & DANGLING POINTER! ⚠️👀{RESET}");
        println!(
            "{YELLOW}“Borderline passing marks mile hain guru! \n\
             Dangling pointer detected—dono mein se ek partner emotionally unallocated lag raha hai! \n\
             Ya toh efforts badhao, warna 'friendzone' wali branch mein merge ho jaoge! 😅”{RESET}"
        );
    } else {
        println!("{RED}CRITICAL KERNEL PANIC: DIL CRASH HO GAYA! 💔🚑{RESET}");
        println!(
            "{RED}“Bhaago! Unhandled Exception! Borrow checker ne haath khade kar diye hain! \n\
             Rishte mein itne merge conflicts hain ki Git bhi ro raha hai. \n\
             Immediate rollback karo aur dost ban kar samosa khao!”{RESET}"
        );
    }

    println!("\n{PURPLE}╔══════════════════════════════════════════════════════════╗{RESET}");
    println!("{PINK}║       Stay Safe, Stay Concurrent & Keep Rusting! 🦀      ║{RESET}");
    println!("{PURPLE}╚══════════════════════════════════════════════════════════╝{RESET}\n");
}
