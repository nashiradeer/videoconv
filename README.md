# Nashira Deer // Videoconv

Convert a single file or a entire directory of videos in MKV format to MP4 (H264 + AAC) using CUDA hardware acceleration with FFmpeg (command line).

_A very specific tool for a very specific need._

[![PayPal](https://img.shields.io/badge/Paypal-003087?style=for-the-badge&logo=paypal&logoColor=%23fff)](https://www.paypal.com/donate/?business=QQGMTC3FQAJF6&no_recurring=0&item_name=Thanks+for+donating+for+me%2C+this+helps+me+a+lot+to+continu+developing+and+maintaining+my+projects.&currency_code=USD)
[![GitHub Sponsor](https://img.shields.io/badge/GitHub%20Sponsor-181717?style=for-the-badge&logo=github&logoColor=%23fff)](https://github.com/sponsors/nashiradeer)

## Building

1. Install [Rust v1.81.0](https://www.rust-lang.org/tools/install) or later.
2. Clone the repository and navigate to the project directory.
3. Run `cargo build --release` to build the project.
4. The compiled binary will be located at `target/release/videoconv`.

## Using

1. Install [FFmpeg](https://ffmpeg.org/download.html) and make sure it is in your system's PATH.
2. Move the compiled binary from the `target/release` directory to a location in your system's PATH.
3. Now you can run the binary from the command line with `videoconv` followed by the path to the file or directory you want to convert.
   3.1. Example: `videoconv /path/to/file.mkv` or `videoconv /path/to/directory/`.
   3.2. The converted files will be saved in the same directory as the original files with the same name but with the `.mp4` extension.
   3.3. If no path is provided, the program will convert all MKV files in the current directory.
   3.4. The program will skip files that are not in MKV format and files that are already have an file with the same name and `.mp4` extension.

## Credits

This project has been created by Nashira Deer and is licensed under the [MIT License](https://github.com/nashiradeer/videoconv/blob/main/LICENSE.txt).
