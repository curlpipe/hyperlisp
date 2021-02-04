# Hyperlisp

## What the heck is hyperlisp?

- It's a markup language designed for the web
- It transpiles directly into html
- It's written in Rust therefore it can transpile quickly
- It's inspired by Lisp and isn't too verbose
- It's free to use for any purpose you wish
- Feel free to modify and play around with how it's made
  - It's only 145 lines of code (excluding comments and blank lines)
  - The code is well commented
<!-- It'll alert you to any errors in your markup, making it easier to debug -->

## Installation

You can run the following commands on linux to get it up and running
```sh
git clone https://github.com/curlpipe/hyperlisp.git
cd hyperlisp
cargo build --release
sudo cp target/release/hyperlisp /usr/bin/
```

Or you can use the prebuilt binaries provided in the releases section and move it to `/usr/bin/`

## Conversion from hyperlisp to html
After installation, you'll be able to convert hyperlisp files into html

To show the help menu: 
```
hyperlisp -h
```

To convert `index.hls` file into html and print it out:
```
hyperlisp -i index.hls
```

To convert `index.hls` file into `index.html`:
```
hyperlisp -i index.hls -o index.html
```

## Syntax
You can check out the examples folder to see some examples of how to write it.

Below are some smaller snippets to help you quickly grasp the language.

### Single tags

```lisp
(h1 Hello World)
```

```html
<h1>Hello World</h1>
```

### Nested tags
```lisp
(h1 This is an example of (b Nested tags!) Pretty cool!)
```

```html
<h1>This is an example of <b>Nested tags!</b> Pretty cool!</h1>
```

### Attributes
```lisp
(div id="background" class="container gradient" 
  Here's a tag: (h1 This is a div)
)
```

```html
<div id="background" class="container gradient">
  Here's a tag: <h1>This is a div</h1>
</div>
```

### Multiple nested tags
```lisp
(body (h1 Tag number 1) (p Tag number 2))
```

```html
<body><h1>Tag number 1</h1> <p>Tag number 2</p></body>
```

### Comments
```lisp
(h1 This is a heading, it is displayed) !(This is a comment and it's not displayed)
```

```html
<h1>This is a heading, it is displayed</h1>
```

