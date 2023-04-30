mod filters;
mod transform;
mod visualgen;

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    let subcommand = args.remove(0);
    match subcommand.as_str() {
        "blur" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let parse_job = args.remove(0).parse::<f32>();
            if parse_job.is_ok() {
                filters::blur(infile, outfile, parse_job.unwrap());
            } else {
                println!("Invalid amount");
            }
        }

        "brighten" => {
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let parse_job = args.remove(0).parse::<i32>();
            if parse_job.is_ok() {
                filters::brighten(infile, outfile, parse_job.unwrap());
            } else {
                println!("Invalid amount");
            }
        }

        "crop" => {
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let x_parse_job = args.remove(0).parse::<u32>();
            let y_parse_job = args.remove(0).parse::<u32>();
            let w_parse_job = args.remove(0).parse::<u32>();
            let h_parse_job = args.remove(0).parse::<u32>();

            if x_parse_job.is_ok()
                && y_parse_job.is_ok()
                && w_parse_job.is_ok()
                && h_parse_job.is_ok()
            {
                transform::crop(
                    infile,
                    outfile,
                    x_parse_job.unwrap(),
                    y_parse_job.unwrap(),
                    w_parse_job.unwrap(),
                    h_parse_job.unwrap(),
                );
            } else {
                println!("Invalid amount");
            }
        }

        "rotate" => {
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let parse_job = args.remove(0).parse::<i32>();
            if parse_job.is_ok() {
                transform::rotate(infile, outfile, parse_job.unwrap());
            } else {
                println!("Invalid amount");
            }
        }

        "invert" => {
            let infile = args.remove(0);
            let outfile = args.remove(0);
            filters::invert(infile, outfile);
        }

        "grayscale" => {
            let infile = args.remove(0);
            let outfile = args.remove(0);
            filters::grayscale(infile, outfile);
        }

        "fractal" => {
            if args.len() != 1 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            visualgen::fractal(outfile);
        }

        _ => {
            print_usage_and_exit();
        }
    }
}

fn print_usage_and_exit() {
    println!("blur INFILE OUTFILE AMOUNT -> Blurs the image");
    println!("brighten INFILE OUTFILE AMOUNT -> changes lumenosity by amount. (Negative values to darken)");
    println!("crop INFILE OUTFILE X_POS Y_POS WIDTH HEIGHT -> Crops the image by the provided rectangle.");
    println!("rotate INFILE OUTFILE AMOUNT -> Rotates the image clockwise.");
    println!("invert INFILE OUTFILE -> Inverts the color of each pixel in the image.");
    println!("grayscale INFILE OUTFILE -> Changes the image to black and white.");
    println!("fractal OUTFILE -> Generates a fractal image.");
    std::process::exit(-1);
}
