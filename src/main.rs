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
const MAGENTA: &str = PINK;                         

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
    println!("{PURPLE}           ╭────────────────────────────────────────────╮");
    println!("           │  {ROSE}\"Scanning feelings for memory leaks...\"   {PURPLE}│");
    println!("           │   {PINK}100% thread-safe pyaar milega! 💖        {PURPLE}│");
    println!("           ╰───────────────────────╮────────────────────╯");
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
        println!("\n{RED}Error: NullPointerException! Pyaar bina naam ke nahi hota!{RESET}");
        return;
    }

    println!("\n{YELLOW}⚡ Initiating Romantic Compatibility Audit...{RESET}\n");
    sleep(Duration::from_millis(600));

    
    let checks = [
        "1. Scanning biometric heartbeat over TCP socket...",
        "2. Consulting Rust Borrow Checker for jealousy issues...",
        "3. Checking multi-threaded synchronization & data races...",
        "4. Validating lifetime annotations ('static vs temporary)...",
        "5. Checking Git merge conflicts in feelings...",
    ];

    for check in checks {
        typewriter(&format!("  {CYAN}▸ {check}{RESET} "), 10);
        sleep(Duration::from_millis(400));
        println!("{GREEN}[PASSED]{RESET}");
    }

    sleep(Duration::from_millis(500));
    println!("\n{YELLOW}======================================================{RESET}");
    println!("{BOLD}              📊 AUDIT REPORT GENERATED               {RESET}");
    println!("{YELLOW}======================================================{RESET}\n");

    let score = calculate_love_score(name1, name2);

    print!("{BOLD}Pair: {CYAN}{name1}{RESET} {RED}♥{RESET} {CYAN}{name2}{RESET}\n");
    print!("{BOLD}Compatibility Score: {RESET}");
    draw_heart_bar(score);
    println!();

    // Verdicts
    print!("{BOLD}Official Compiler Verdict: {RESET}");
    if score >= 95 {
        println!("{GREEN}PERFECT THREAD SAFETY! 💍🦀{RESET}");
        println!(
            "{MAGENTA}“Aapka rishta 'static lifetime jaisa hai—hamesha amar! \n\
             Zero data races, 100% memory safe romance. \n\
             Panditji ko call karo, shaadi fix hai!”{RESET}"
        );
    } else if score >= 80 {
        println!("{GREEN}HIGH COMPATIBILITY! 💕✨{RESET}");
        println!(
            "{CYAN}“Bohot zabardast tuning hai! \n\
             Bas yaad rakhna: Pyaar mein sirf EK hi mutable reference allowed hai, \n\
             warna jealousy ka bug aa sakta hai! 😉”{RESET}"
        );
    } else if score >= 65 {
        println!("{YELLOW}STABLE BUT NEEDS REFACTORING! 🛠️{RESET}");
        println!(
            "{YELLOW}“Connection theek chal raha hai, lekin thoda latency aur \n\
             garbage collection ki zaroorat hai. Ek acchi coffee date par jaao! ☕”{RESET}"
        );
    } else if score >= 50 {
        println!("{YELLOW}WARNING: RUNTIME OVERHEAD! ⚠️{RESET}");
        println!(
            "{YELLOW}“Dangling pointer detected! Ek partner emotionally unallocated lag raha hai. \n\
             Communication ka bandwidth badhao!”{RESET}"
        );
    } else {
        println!("{RED}CRITICAL PANIC: SEGMENTATION FAULT! 💔💥{RESET}");
        println!(
            "{RED}“Dil crash ho gaya! Borrow checker ne pyaar reject kar diya! \n\
             Immediate rollback karke dost ban ke raho.”{RESET}"
        );
    }

    println!("\n{CYAN}======================================================{RESET}");
    println!("{MAGENTA}    Stay Safe, Stay Concurrent & Keep Rusting! 🦀     {RESET}");
    println!("{CYAN}======================================================{RESET}\n");
}
