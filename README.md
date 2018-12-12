![base image](https://github.com/raphaeltraviss/rust_rails/blob/master/crates/subject_detect/test_stock/squares.jpeg "The base image")

# Rust on Rails tutorial


As a Ruby developer, you might yearn to eat of the "forbidden fruit"--things normally reserved for native programmers:
* interacting directly with hardware and IO streams
* performing efficient computations on the CPU
* inventing your own concurrency models to architect your code

Normally, attempting any of these things would require you to walk a path full of pain, segfaults, and buffer overrun exploits.

Enter Rust, a native programming language that has a lot to offer for Ruby developers.  And now, the Helix project creates a bridge between those two worlds.

## Setting up and running this project

#### Install the Prerequisites
- Ruby
- bundler
- Rust compiler

If you're on a Mac, these are just a `brew install` away.

On Windows, you can check out rubyinstaller.org and rustup.rs.  Take care to use the correct shell
for your build environment: MinGW bundled with Ruby might be a good place to start.  If that doesn't
work, MSYS2 would be the next thing to try.

#### Clone, Build, and Run the Project
```
git clone some-provider.rust_rails.git
cd rust_rails
bundle install
cd crates/subject_detect
rake build
rails server
```

#### Watch it do something

Look at the directory in rust_rails/crates/subject_detect.  It contains an image called squares.jpeg.  In a web browser, visit localhost:3000/compression_rating.  You should see a number on the screen, indicating the number of edges found in the image.  If you check the directory, you should see the squares_edges.png and square_edges_lines.png images sitting there, which will show you the intermediate images that were generated during the computation.

## What is this actually supposed to do?

![edge image, from base image](https://github.com/raphaeltraviss/rust_rails/blob/master/crates/subject_detect/test_stock/squares_edges.png "Intermediate image showing the edges detected in the base image")

![lined image, from edge image](https://github.com/raphaeltraviss/rust_rails/blob/master/crates/subject_detect/test_stock/square_edges_lines.png "The final lined image, used to get the line count")

The finished tutorial will have Posts that you can attach images to.  When you upload an image, a native Helix extension will immediately begin processing the image: no intermediate files needed.  Since this all happens in-process, you can use the computation as a validation.  Everything is handled in Rust, so there are other opportunities to use threading.  Later, we can come full-circle, and break the extension out into an out-of-process "thread pool" executable, that is a lot more efficient than spinning up several ImageMagick processes.  Who knows, we might even measure things.

## T H E  E N D
