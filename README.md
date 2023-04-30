# Image-Tool

A simple Rust command-line program to explore the scripting capabilities of Rust.


### Overview
Image tool is an image editing application written in Rust to be used on Ubuntu operating systems
for fast and precise image alterations. Best use-cases are for data transformations in pipelines
and batch shell scripting.

### Usage
Tested only on Ubuntu 22.04.2 LTS

- Ensure [Rust & cargo](https://www.rust-lang.org/tools/install) is installed on tour machine then clone or download the repo and run: 
`cargo build --release`

- The compiled binary will be in **target/release/** as image_tool

- You can optionally create an alias to the release file in your .bashrc:
`echo 'alias image-tool="ABSOLUTE_PATH_TO_RELEASE_FILE"' >> ~/.bashrc`

*As an example:* 
`echo 'alias image-tool="/home/<user>/dev/image_tool/target/release/image_tool"' >> ~/.bashrc`

- Restart your terminal and check by running `image-tool`

If everything was done correctly this should print the command line arguments available for 
generating and modifying an image.
