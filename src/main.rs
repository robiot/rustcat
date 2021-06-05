////////////////////
// Rustcat (rc) 
// by: robiot
////////////////////

use termion::color;
use getopts::Options;

/* Global Variables */
const  __VERSION__: &'static str = env!("CARGO_PKG_VERSION");

/* Use laters */
struct Opts<'a> {
    host: &'a str,
    port: &'a str,
    transport: Protocol
}

enum Protocol {
    Tcp,
    Udp,
}

/* Help -h */
fn print_help(program: &str, opts: Options, code: i32) {
    let brief = format!("Usage: {} [options] [destination] [port]", program);
    print!("{}", opts.usage(&brief));
    if code != 0 {
        std::process::exit(code);
    }
}

/* Prints error */
fn print_error(err: &str) {
    eprintln!("{}rc:{} {}", color::Fg(color::LightRed), color::Fg(color::Reset), err);
}

/* Print when connection recieved */
fn print_started_listen(opts: &Opts) {
    println!("Listening on {}{}{}:{}{}{}", color::Fg(color::LightGreen), opts.host, color::Fg(color::Reset), color::Fg(color::LightCyan), opts.port, color::Fg(color::Reset)); //move this?
}

/* Piped thread */
fn pipe_thread<R, W>(mut r: R, mut w: W) -> std::thread::JoinHandle<()>  where R: std::io::Read + Send + 'static, W: std::io::Write + Send + 'static
{
    std::thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            let len = r.read(&mut buffer).unwrap();
            if len == 0 {
                println!("\n{}[-]{}Connection lost",color::Fg(color::LightRed), color::Fg(color::Reset));
                std::process::exit(0x0100);
            }
            w.write(&buffer[..len]).unwrap();
            w.flush().unwrap();
        }
    })
}

/* Listen on given host and port */
fn listen(opts: &Opts) -> std::io::Result<()>{
    match opts.transport {
        Protocol::Tcp => {
            let listener = std::net::TcpListener::bind(format!("{}:{}",opts.host, opts.port))?;
            print_started_listen(opts);

            let (stream, _) = listener.accept()?;
            let t1 = pipe_thread(std::io::stdin(), stream.try_clone()?);
            println!("{}[+]{} Connection received", color::Fg(color::LightGreen), color::Fg(color::Reset));
            let t2 = pipe_thread(stream, std::io::stdout());
            t1.join().unwrap();
            t2.join().unwrap();
        }

        Protocol::Udp => {
            //todo: add udp alternative
            println!("Udp is curently not supported. Please wait for a future update")
        }
    }
    return Ok(());
}

/* Main */
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "This help text");
    opts.optflag("v", "version", "Application Version");
    opts.optflag("l", "", "Listen mode");
    opts.optflag("p", "", "Listen port");
    opts.optflag("u", "", "UDP mode (Not available yet)");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(err) => {
            print_error(&err.to_string());
            return;
        }
    };

    if matches.opt_present("h") {
        print_help(&program, opts, 0);
        return;
    } else if matches.opt_present("v") {
        println!("Rustcat v{}",__VERSION__);
        return;
    }
    

    if matches.opt_present("l") {
        let (opt_host, opt_port) = if matches.free.len() == 1 && matches.opt_present("p"){
            ("0.0.0.0", matches.free[0].as_str())
        } else if matches.free.len() == 2{
            (matches.free[0].as_str(), matches.free[1].as_str())
        }
        else {
            print_help(&program, opts, 1);
            ("","")
        };

        let opts = Opts {
            host: opt_host,
            port: opt_port,
            transport: if matches.opt_present("u") {
                Protocol::Udp
            } else {
                Protocol::Tcp
            }
        };
    
        if let Err(err) = listen(&opts) {
            print_error(&err.to_string());
            return;
        };
    }
    else {
        print_help(&program, opts, 1);
        return
    };
}